use std::env;
use std::path::PathBuf;

use crate::config::{
    AI_INSTRUCTIONS, AI_WELCOME_MESSAGE, DEFAULT_OLLAMA_MODEL, DEFAULT_PANEL_RATIO,
    TERMINAL_INSTRUCTIONS, WINDOW_WIDTH, WINDOW_HEIGHT, FocusTarget,
};
use crate::model::{CommandStatus, Panel};

impl crate::model::App {
    pub fn new() -> Self {
        // Start with root directory as default
        let mut current_dir = PathBuf::from("/");

        // Set the current working directory to the root
        // Ensure we properly handle errors when setting the current directory
        if let Err(e) = env::set_current_dir(&current_dir) {
            eprintln!("Warning: Failed to set current directory to /: {}", e);
            // In case of error, try to use the home directory instead
            if let Some(home) = dirs_next::home_dir() {
                if let Err(e) = env::set_current_dir(&home) {
                    eprintln!("Warning: Failed to set current directory to home: {}", e);
                } else {
                    // Successfully set to home directory
                    eprintln!("Using home directory instead: {:?}", home);
                    current_dir = home;
                }
            }
        }
        
        // Double-check the actual current directory after attempts to set it
        if let Ok(actual_dir) = env::current_dir() {
            current_dir = actual_dir;
        }
        eprintln!("App initialized with current directory: {:?}", current_dir);

        // Detect OS information
        let os_info = detect_os();

        // Check if current directory is a git repository
        let (is_git_repo, git_branch) = crate::terminal::utils::get_git_info(&current_dir);

        // Initial output messages
        let mut initial_output = vec![
            format!("Operating System: {}", os_info),
        ];

        // Add terminal instructions
        for instruction in TERMINAL_INSTRUCTIONS.iter() {
            initial_output.push(instruction.to_string());
        }

        // Initial AI output messages
        let mut initial_ai_output = vec![AI_WELCOME_MESSAGE.to_string()];

        // Add AI instructions
        for instruction in AI_INSTRUCTIONS.iter() {
            initial_ai_output.push(instruction.to_string());
        }

        // Initialize command status for any commands in the initial output
        let mut command_status = Vec::new();
        for line in &initial_output {
            if line.starts_with("> ") {
                command_status.push(CommandStatus::Success);
            }
        }

        crate::model::App {
            input: String::new(),
            output: initial_output.clone(),
            cursor_position: 0,
            current_dir,
            is_git_repo,
            git_branch,
            // Initialize AI assistant fields
            ai_input: String::new(),
            ai_output: initial_ai_output.clone(),
            ai_cursor_position: 0,
            active_panel: Panel::Terminal,
            // Panel management
            panel_ratio: DEFAULT_PANEL_RATIO,
            is_resizing: false,
            window_width: WINDOW_WIDTH as f32,
            window_height: WINDOW_HEIGHT as f32,
            // Initialize scroll state
            terminal_scroll: 0,
            assistant_scroll: 0,
            // Initialize command status tracking
            command_status,
            // Initialize command history
            command_history: Vec::new(),
            command_history_index: None,
            // Initialize autocomplete
            autocomplete_suggestions: Vec::new(),
            autocomplete_index: None,
            // Ollama integration
            ollama_model: DEFAULT_OLLAMA_MODEL.to_string(),
            ollama_thinking: false,
            // Extracted commands from AI responses
            extracted_commands: Vec::new(),
            // Most recent command from AI assistant
            last_ai_command: None,
            // Last terminal command and output for context
            last_terminal_context: None,
            // System information
            os_info,
            // Auto-execute commands (disabled by default)
            auto_execute_commands: false,
            // Focus target
            focus: FocusTarget::Terminal,
            // Change the command_receiver to use Arc to make it cloneable
            command_receiver: None,
            // Password mode
            password_mode: false,
            initial_output_count: initial_output.len(),
            initial_ai_output_count: initial_ai_output.len(),
        }
    }
}

// Helper function to detect OS information
fn detect_os() -> String {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let family = std::env::consts::FAMILY;
    
    format!("{} ({}, {})", os, arch, family)
}
