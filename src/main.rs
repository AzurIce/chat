use chat::run_cli;

fn main() {
    if let Err(e) = run_cli() {
        eprintln!("错误: {}", e);
        std::process::exit(1);
    }
}
