pub mod cli;
pub mod core;
pub mod bridge;

pub fn run_cli() -> Result<(), Box<dyn std::error::Error>> {
    cli::run()
} 