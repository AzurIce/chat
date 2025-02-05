use crate::bridge::{Bridge, Message};
use crate::config::Config;
use anyhow::Result;

pub struct Core {
    bridge: Bridge,
    config: Config,
}

impl Core {
    pub fn new() -> Result<Self> {
        let config = Config::load()?;
        let bridge = Bridge::new(
            config.api_base.clone(),
            config.token.clone(),
            config.model.clone(),
        );
        
        Ok(Self { bridge, config })
    }

    pub async fn chat(&mut self, message: &str) -> Result<String> {
        // 从配置中获取历史记录并转换为消息格式
        let mut messages = Vec::new();
        for item in self.config.get_history() {
            messages.push(Message {
                role: "user".to_string(),
                content: item.question.clone(),
            });
            messages.push(Message {
                role: "assistant".to_string(),
                content: item.answer.clone(),
            });
        }
        messages.push(Message {
            role: "user".to_string(),
            content: message.to_string(),
        });

        let response = self.bridge.chat_with_history(&messages).await?;
        
        // 保存新的对话记录
        self.config.add_history(message.to_string(), response.clone());
        
        Ok(response)
    }

    pub fn clear_history(&mut self) -> Result<()> {
        self.config.clear_history()
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn update_config(&mut self, new_config: Config) -> Result<()> {
        new_config.save()?;
        self.config = new_config;
        self.bridge = Bridge::new(
            self.config.api_base.clone(),
            self.config.token.clone(),
            self.config.model.clone(),
        );
        Ok(())
    }
}

// TODO: 实现核心 API 功能
pub struct PromptManager {
    // TODO: 实现 prompt 管理
} 