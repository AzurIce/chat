// TODO: 实现 LLM 接口和转发功能
pub trait LLMProvider {
    fn send_message(&self, message: &str) -> Result<String, Box<dyn std::error::Error>>;
}

use serde::{Deserialize, Serialize};
use anyhow::Result;
use futures::Stream;
use futures::StreamExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: Message,
}

pub struct Bridge {
    client: reqwest::Client,
    api_base: String,
    token: String,
    model: String,
    // 可选配置项
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    top_k: Option<u32>,
    frequency_penalty: Option<f32>,
}

impl Bridge {
    pub fn new(api_base: String, token: String, model: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_base,
            token,
            model,
            max_tokens: None,
            temperature: None,
            top_p: None,
            top_k: None,
            frequency_penalty: None,
        }
    }

    // 配置方法
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn with_top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    pub fn with_top_k(mut self, top_k: u32) -> Self {
        self.top_k = Some(top_k);
        self
    }

    pub fn with_frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        self.frequency_penalty = Some(frequency_penalty);
        self
    }

    // 聊天方法
    pub async fn chat(&self, message: &str) -> Result<String> {
        let request = ChatRequest {
            model: self.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: message.to_string(),
            }],
            stream: false,
            max_tokens: self.max_tokens,
            temperature: self.temperature,
            top_p: self.top_p,
            top_k: self.top_k,
            frequency_penalty: self.frequency_penalty,
            n: Some(1),
        };

        let response = self.client
            .post(&self.api_base)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;
        
        // debug
        println!("Raw Response: {:?}", response);

        let chat_response: ChatResponse = response.json().await?;
        Ok(chat_response.choices[0].message.content.clone())
    }

    // 流式聊天方法
    pub async fn chat_stream(&self, message: &str) -> Result<impl Stream<Item = Result<String>>> {
        let request = ChatRequest {
            model: self.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: message.to_string(),
            }],
            stream: true,
            max_tokens: self.max_tokens,
            temperature: self.temperature,
            top_p: self.top_p,
            top_k: self.top_k,
            frequency_penalty: self.frequency_penalty,
            n: Some(1),
        };

        let response = self.client
            .post(&self.api_base)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let stream = response
            .bytes_stream()
            .map(|result| {
                result.map_err(anyhow::Error::from)
                    .and_then(|bytes| {
                        let text = String::from_utf8(bytes.to_vec())?;
                        Ok(text)
                    })
            });

        Ok(stream)
    }

    // 新增：支持带历史记录的对话方法
    pub async fn chat_with_history(&self, messages: &[Message]) -> Result<String> {
        let request = ChatRequest {
            model: self.model.clone(),
            messages: messages.to_vec(),
            stream: false,
            max_tokens: self.max_tokens,
            temperature: self.temperature,
            top_p: self.top_p,
            top_k: self.top_k,
            frequency_penalty: self.frequency_penalty,
            n: Some(1),
        };

        let response = self.client
            .post(&self.api_base)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let chat_response: ChatResponse = response.json().await?;
        Ok(chat_response.choices[0].message.content.clone())
    }
} 