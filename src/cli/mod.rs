use crate::core::Core;
use anyhow::{Result, Context};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "chat")]
#[command(about = "A command line chat tool")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Message to send (when no subcommand is provided)
    #[arg(name = "MESSAGE")]
    message: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Config management
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// History management
    History {
        #[command(subcommand)]
        action: HistoryAction,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Get configuration value
    Get {
        /// Configuration key (optional)
        #[arg(default_value = None)]
        key: Option<String>,
    },
    /// Set configuration value
    Set {
        /// Configuration key
        key: String,
        /// New value
        value: String,
    },
}

#[derive(Subcommand)]
enum HistoryAction {
    /// Clear chat history
    Clear,
    /// Get chat history
    Get,
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut core = Core::new()?;

    match cli.command {
        Some(Commands::Config { action }) => {
            match action {
                ConfigAction::Get { key } => {
                    let config = core.get_config();
                    match key {
                        Some(key) => match key.as_str() {
                            "api_base" => println!("api_base: {}", config.api_base),
                            "token" => println!("token: {}", config.token),
                            "model" => println!("model: {}", config.model),
                            "max_tokens" => println!("max_tokens: {:?}", config.max_tokens),
                            "temperature" => println!("temperature: {:?}", config.temperature),
                            _ => println!("Unknown configuration key: {}", key),
                        },
                        None => {
                            println!("Current configuration:");
                            println!("  api_base: {}", config.api_base);
                            println!("  token: {}", config.token);
                            println!("  model: {}", config.model);
                            println!("  max_tokens: {:?}", config.max_tokens);
                            println!("  temperature: {:?}", config.temperature);
                        }
                    }
                }
                ConfigAction::Set { key, value } => {
                    let mut new_config = core.get_config().clone();
                    match key.as_str() {
                        "api_base" => new_config.api_base = value,
                        "token" => new_config.token = value,
                        "model" => new_config.model = value,
                        "max_tokens" => {
                            new_config.max_tokens = if value.to_lowercase() == "none" {
                                None
                            } else {
                                Some(value.parse().context("Invalid max_tokens value")?)
                            }
                        }
                        "temperature" => {
                            new_config.temperature = if value.to_lowercase() == "none" {
                                None
                            } else {
                                Some(value.parse().context("Invalid temperature value")?)
                            }
                        }
                        _ => {
                            println!("Unknown configuration key: {}", key);
                            return Ok(());
                        }
                    }
                    core.update_config(new_config)?;
                    println!("Configuration updated successfully");
                }
            }
        }
        Some(Commands::History { action }) => {
            match action {
                HistoryAction::Clear => {
                    core.clear_history()?;
                    println!("Chat history cleared");
                }
                HistoryAction::Get => {
                    let history = core.get_config().get_history();
                    if history.is_empty() {
                        println!("No chat history");
                    } else {
                        let mut is_end_with_nn = false;
                        for (i, item) in history.iter().enumerate() {
                            if !is_end_with_nn {
                                println!("");
                            }
                            println!("--- Conversation {} ---", i + 1);
                            println!("Q: {}", item.question);
                            println!("A: {}", item.answer);
                            is_end_with_nn = item.answer.ends_with("\n\n");
                        }
                    }
                }

            }
        }
        None => {
            if let Some(message) = cli.message {
                // println!("Message: {}", message);
                // println!("Response: ");
                let _response = core.chat_stream(&message).await?;
            } else {
                println!("No message provided. Use 'chat --help' for usage information.");
            }
        }
    }

    Ok(())
}