use std::collections::HashMap;
use std::env;
use std::sync::Mutex;
use crate::command::types::command_state::CommandState;
use crate::ollama::types::ollama_state::OllamaState;

// Structure to handle command output streaming
pub struct CommandManager {
    pub commands: Mutex<HashMap<String, CommandState>>,
    pub ollama: Mutex<OllamaState>,
}

impl CommandManager {
    pub fn new() -> Self {
        let mut initial_commands = HashMap::new();
        initial_commands.insert(
            "default_state".to_string(),
            CommandState {
                current_dir: env::current_dir().unwrap_or_default().to_string_lossy().to_string(),
                child_wait_handle: None,
                child_stdin: None,
                pid: None,
                is_ssh_session_active: false, // Initialize here
                remote_current_dir: None, // Initialize new field
            },
        );
        CommandManager {
            commands: Mutex::new(initial_commands),
            ollama: Mutex::new(OllamaState {
                current_model: "llama3.2:latest".to_string(), // Default model will now be overridden by frontend
                api_host: "http://localhost:11434".to_string(), // Default Ollama host
            }),
        }
    }
}
