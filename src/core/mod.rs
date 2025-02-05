use crate::bridge::Bridge;
use anyhow::Result;

pub struct Core {
    bridge: Bridge,
}

impl Core {
    pub fn new() -> Self {
        // TODO: 从配置文件读取这些配置
        let api_base = "https://api.siliconflow.cn/v1/chat/completions".to_string();
        let token = "sk-wtdfnppnpnujbjxekmttfluckctmtttjktkspqfgbqyrqtpb".to_string();
        let model = "deepseek-ai/DeepSeek-V3".to_string();

        let bridge = Bridge::new(api_base, token, model);
        
        Self { bridge }
    }

    pub async fn chat(&self, message: &str) -> Result<String> {
        self.bridge.chat(message).await
    }
}

// TODO: 实现核心 API 功能
pub struct ChatHistory {
    // TODO: 实现历史记录结构
}

pub struct PromptManager {
    // TODO: 实现 prompt 管理
} 