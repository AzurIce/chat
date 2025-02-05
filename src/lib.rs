pub mod cli;
pub mod core;
pub mod bridge;
pub mod config;

pub async fn run_cli() -> Result<(), Box<dyn std::error::Error>> {
    cli::run().await
} 