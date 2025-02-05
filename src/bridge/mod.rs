// TODO: 实现 LLM 接口和转发功能
pub trait LLMProvider {
    fn send_message(&self, message: &str) -> Result<String, Box<dyn std::error::Error>>;
} 