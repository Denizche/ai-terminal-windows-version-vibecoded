use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIProvider {
    Ollama,
    LocalAI,
    OpenAI,
}

impl std::fmt::Display for AIProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AIProvider::Ollama => write!(f, "Ollama"),
            AIProvider::LocalAI => write!(f, "LocalAI"),
            AIProvider::OpenAI => write!(f, "OpenAI"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalAIRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stream: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String, // "system", "user", "assistant"
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalAIResponse {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<u64>,
    pub model: Option<String>,
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub index: Option<u32>,
    pub message: Option<ChatMessage>,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
}

// Generic AI request that can be used for different providers
#[derive(Debug, Serialize, Deserialize)]
pub struct GenericAIRequest {
    pub provider: AIProvider,
    pub model: String,
    pub prompt: String,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

// Generic AI response
#[derive(Debug, Serialize, Deserialize)]
pub struct GenericAIResponse {
    pub provider: AIProvider,
    pub model: String,
    pub content: String,
    pub usage: Option<Usage>,
}