// use crate::core;
use crate::core::Core;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: 实现命令行解析和处理逻辑
    println!("命令行工具启动成功！");

    let core = Core::new();

    let message = "Hello.";
    println!("Message: {}", message);
    let response = core.chat(message).await?;
    println!("Response: {}", response);

    Ok(())
} 