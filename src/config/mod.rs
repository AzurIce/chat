use std::{fs, path::Path};
use std::path::PathBuf;
use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub api_base: String,
    pub token: String,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(default = "default_max_history")]
    pub max_history: usize,
    #[serde(default)]
    pub history: VecDeque<HistoryItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoryItem {
    pub question: String,
    pub answer: String,
}

fn default_max_history() -> usize {
    10
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_base: "https://api.siliconflow.cn/v1/chat/completions".to_string(),
            token: "sk-default-token".to_string(),
            model: "deepseek-ai/DeepSeek-V3".to_string(),
            max_tokens: None,
            temperature: None,
            max_history: default_max_history(),
            history: VecDeque::new(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = get_config_path()?;
        
        if !config_path.exists() {
            let config = Config::default();
            config.save()?;

            println!("Config file auto created successfully. Path: {}", config_path.display());

            return Ok(config);
        }

        let content = fs::read_to_string(&config_path)
            .context("Failed to read config file")?;
        let config: Config = toml::from_str(&content)
            .context("Failed to parse config file")?;
        
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
       let config_path = get_config_path()?;
       
       // 确保配置目录存在
       if let Some(parent) = config_path.parent() {
           fs::create_dir_all(parent)
               .context("Failed to create config directory")?;
       }

       let content = toml::to_string_pretty(self)
           .context("Failed to serialize config")?;
       fs::write(&config_path, content)
           .context("Failed to write config file")?;
       
       Ok(())
    }

    pub fn add_history(&mut self, question: String, answer: String) {
        if self.history.len() >= self.max_history {
            self.history.pop_front();
        }
        self.history.push_back(HistoryItem { question, answer });
        // 每次添加历史后自动保存
        self.save().expect("Failed to save history");
    }

    pub fn clear_history(&mut self) -> Result<()> {
        self.history.clear();
        self.save()
    }

    pub fn get_history(&self) -> &VecDeque<HistoryItem> {
        &self.history
    }
}

fn get_config_path() -> Result<PathBuf> {
    // user config dir
    // let config_dir = dirs::config_dir()
    //     .context("Failed to get config directory")?;
    // Ok(config_dir.join("chat").join("config.toml"))

    // project root dir
    let config_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("config.toml");
    Ok(config_dir)
} 