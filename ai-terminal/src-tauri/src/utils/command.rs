use crate::command::types::command_manager::CommandManager;
use crate::ollama::types::ollama_model_list::OllamaModelList;
use tauri::State;

// Handle special commands like /help, /models, /model
pub async fn handle_special_command(
    command: String,
    command_manager: State<'_, CommandManager>,
) -> Result<String, String> {
    match command.as_str() {
        "/help" => Ok("Available commands:\n\
                /help - Show this help message\n\
                /models - List available models\n\
                /model [name] - Show current model or switch to a different model\n\
                /host [url] - Show current API host or set a new one"
            .to_string()),
        "/models" => {
            // Get list of available models from Ollama API
            let api_host;

            // Scope the mutex lock to drop it before any async operations
            {
                let ollama_state = command_manager.ollama.lock().map_err(|e| e.to_string())?;
                api_host = ollama_state.api_host.clone();
                // MutexGuard is dropped here
            }

            let client = reqwest::Client::new();
            let res = client
                .get(format!("{}/api/tags", api_host))
                .send()
                .await
                .map_err(|e| format!("Failed to get models from Ollama API: {}", e))?;

            if !res.status().is_success() {
                return Err(format!("Ollama API error: {}", res.status()));
            }

            let models: OllamaModelList = res
                .json()
                .await
                .map_err(|e| format!("Failed to parse models list: {}", e))?;

            let mut result = String::from("Available models:\n");
            for model in models.models {
                result.push_str(&format!("- {} ({} bytes)\n", model.name, model.size));
            }
            Ok(result)
        }
        cmd if cmd.starts_with("/model") => {
            let parts: Vec<&str> = cmd.split_whitespace().collect();

            // Handle showing current model
            if parts.len() == 1 {
                let current_model;
                {
                    let ollama_state = command_manager.ollama.lock().map_err(|e| e.to_string())?;
                    current_model = ollama_state.current_model.clone();
                }
                Ok(format!("Current model: {}", current_model))
            }
            // Handle switching model
            else if parts.len() >= 2 {
                let new_model = parts[1].to_string();
                {
                    let mut ollama_state =
                        command_manager.ollama.lock().map_err(|e| e.to_string())?;
                    ollama_state.current_model = new_model.clone();
                }
                Ok(format!("Switched to model: {}", new_model))
            } else {
                Err("Invalid model command. Use /model [name] to switch models.".to_string())
            }
        }
        cmd if cmd.starts_with("/host") => {
            let parts: Vec<&str> = cmd.split_whitespace().collect();

            // Handle showing current host
            if parts.len() == 1 {
                let current_host;
                {
                    let ollama_state = command_manager.ollama.lock().map_err(|e| e.to_string())?;
                    current_host = ollama_state.api_host.clone();
                }
                Ok(format!("Current Ollama API host: {}", current_host))
            }
            // Handle changing host
            else if parts.len() >= 2 {
                let new_host = parts[1].to_string();
                {
                    let mut ollama_state =
                        command_manager.ollama.lock().map_err(|e| e.to_string())?;
                    ollama_state.api_host = new_host.clone();
                }
                Ok(format!("Changed Ollama API host to: {}", new_host))
            } else {
                Err("Invalid host command. Use /host [url] to change the API host.".to_string())
            }
        }
        _ => Err(format!(
            "Unknown command: {}. Type /help for available commands.",
            command
        )),
    }
}
