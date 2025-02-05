use crate::bridge::Bridge;
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

    pub async fn chat(&self, message: &str) -> Result<String> {
        self.bridge.chat(message).await
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
pub struct ChatHistory {
    // TODO: 实现历史记录结构
}

pub struct PromptManager {
    // TODO: 实现 prompt 管理
} 