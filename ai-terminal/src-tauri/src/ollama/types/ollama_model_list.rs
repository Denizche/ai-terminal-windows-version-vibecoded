use crate::ollama::types::ollama_model::OllamaModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaModelList {
    pub models: Vec<OllamaModel>,
}
