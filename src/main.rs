use chat::run_cli;

#[tokio::main]
async fn main() {
    if let Err(e) = run_cli().await {
        eprintln!("错误: {}", e);
        std::process::exit(1);
    }
}
