use serde::{Deserialize, Serialize};
use crate::ollama::types::ollama_model::OllamaModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaModelList {
    pub models: Vec<OllamaModel>,
}