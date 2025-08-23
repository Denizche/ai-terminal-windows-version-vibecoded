use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaResponse {
    model: String,
    pub response: String,
    done: bool,
}
