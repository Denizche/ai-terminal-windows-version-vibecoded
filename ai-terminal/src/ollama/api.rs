use reqwest::Client;
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::config::{OLLAMA_API_URL, OLLAMA_LIST_MODELS_URL};
use crate::model::{OllamaModelList, OllamaRequest, OllamaResponse};

// Global flag to track if a request is in progress
pub static IS_THINKING: AtomicBool = AtomicBool::new(false);

// Send a prompt to Ollama and get the response
pub async fn send_prompt(model: &str, prompt: &str) -> Result<String, String> {
    println!("send_prompt: Sending prompt to model {}", model);
    let result = {
        let client = Client::new();
        
        let request = OllamaRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            stream: false,
            system: None, // add here the system prompt
        };
        
        println!("send_prompt: Sending request to {}", OLLAMA_API_URL);
        match client.post(OLLAMA_API_URL).json(&request).send().await {
            Ok(response) => {
                println!("send_prompt: Got response with status {}", response.status());
                if response.status().is_success() {
                    match response.json::<OllamaResponse>().await {
                        Ok(ollama_response) => {
                            println!("send_prompt: Successfully parsed response");
                            Ok(ollama_response.response)
                        },
                        Err(e) => {
                            println!("send_prompt: Failed to parse response: {}", e);
                            Err(format!("Failed to parse response: {}", e))
                        }
                    }
                } else {
                    println!("send_prompt: API error with status {}", response.status());
                    Err(format!("API error: {}", response.status()))
                }
            }
            Err(e) => {
                println!("send_prompt: Request error: {}", e);
                Err(format!("Request error: {}", e))
            }
        }
    };
    
    // Set the thinking flag back to false after getting the response
    IS_THINKING.store(false, Ordering::SeqCst);
    
    result
}

// Get a list of available models from Ollama
pub async fn list_models() -> Result<Vec<String>, String> {
    println!("list_models: Fetching models from {}", OLLAMA_LIST_MODELS_URL);
    let client = Client::new();
    
    match client.get(OLLAMA_LIST_MODELS_URL).send().await {
        Ok(response) => {
            println!("list_models: Got response with status {}", response.status());
            if response.status().is_success() {
                match response.json::<OllamaModelList>().await {
                    Ok(model_list) => {
                        println!("list_models: Successfully parsed {} models", model_list.models.len());
                        let models = model_list.models.into_iter()
                            .map(|model| model.name)
                            .collect();
                        Ok(models)
                    }
                    Err(e) => {
                        println!("list_models: Failed to parse response: {}", e);
                        Err(format!("Failed to parse response: {}", e))
                    }
                }
            } else {
                println!("list_models: API error with status {}", response.status());
                Err(format!("API error: {}", response.status()))
            }
        }
        Err(e) => {
            println!("list_models: Request error: {}", e);
            Err(format!("Request error: {}", e))
        }
    }
}