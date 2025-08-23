extern crate fix_path_env;

use ai_terminal_lib::command;
use ai_terminal_lib::command::git_commands::git::new_git_command;
use ai_terminal_lib::command::types::command_manager::CommandManager;
use ai_terminal_lib::ollama::types::ollama_model_list::OllamaModelList;
use ai_terminal_lib::ollama::types::ollama_request::OllamaRequest;
use ai_terminal_lib::ollama::types::ollama_response::OllamaResponse;
use ai_terminal_lib::utils::operating_system_utils::get_operating_system;
use serde_json;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{command, State};

#[command]
fn autocomplete(
    input: String,
    session_id: String,
    command_manager: State<'_, CommandManager>,
) -> Result<Vec<String>, String> {
    let states = command_manager.commands.lock().map_err(|e| e.to_string())?;
    let key = session_id;

    let current_dir = if let Some(state) = states.get(&key) {
        &state.current_dir
    } else {
        return Err("Could not determine current directory".to_string());
    };

    let input_parts: Vec<&str> = input.split_whitespace().collect();

    // Autocomplete commands if it's the first word
    if input_parts.len() <= 1 && input_parts.first() != Some(&"cd") {
        // Common shell commands to suggest
        let common_commands = vec![
            "cd", "ls", "pwd", "mkdir", "touch", "cat", "echo", "grep", "find", "cp", "mv", "rm",
            "tar", "gzip", "ssh", "curl", "wget", "history", "exit", "clear", "top", "ps", "kill",
            "ping",
        ];

        // Filter commands that match input prefix
        let input_prefix = input_parts.first().unwrap_or(&"");

        // Case-insensitive filtering for commands
        let matches: Vec<String> = common_commands
            .iter()
            .filter(|&cmd| cmd.to_lowercase().starts_with(&input_prefix.to_lowercase()))
            .map(|&cmd| cmd.to_string())
            .collect();

        if !matches.is_empty() {
            return Ok(matches);
        }
    }

    // If we have a cd command, autocomplete directories
    let path_to_complete = if input_parts.first() == Some(&"cd") {
        if input_parts.len() > 1 {
            // Handle cd command with argument
            input_parts.last().unwrap_or(&"")
        } else {
            // Handle cd with no argument - show all directories in current folder
            ""
        }
    } else if !input_parts.is_empty() && input_parts[0].contains('/') {
        // Handle path directly
        input_parts[0]
    } else if input_parts.len() > 1 {
        // Handle second argument as path for any command
        input_parts.last().unwrap_or(&"")
    } else {
        // Default to empty string if no path found
        ""
    };

    // If input starts with cd, or we have a potential path to complete
    if input_parts.first() == Some(&"cd") || !path_to_complete.is_empty() {
        let (dir_to_search, prefix) = split_path_prefix(path_to_complete);

        // Create a Path for the directory to search
        let search_path = if dir_to_search.starts_with('/') || dir_to_search.starts_with('~') {
            if dir_to_search.starts_with('~') {
                let home = dirs::home_dir().ok_or("Could not determine home directory")?;
                let without_tilde = dir_to_search.trim_start_matches('~');
                let rel_path = without_tilde.trim_start_matches('/');
                if rel_path.is_empty() {
                    home
                } else {
                    home.join(rel_path)
                }
            } else {
                PathBuf::from(dir_to_search)
            }
        } else {
            Path::new(current_dir).join(dir_to_search)
        };

        if search_path.exists() && search_path.is_dir() {
            let entries = fs::read_dir(search_path).map_err(|e| e.to_string())?;

            let mut matches = Vec::new();
            for entry in entries.flatten() {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();

                // Include all entries for empty prefix, otherwise filter by prefix (case-insensitive)
                if prefix.is_empty()
                    || file_name_str
                        .to_lowercase()
                        .starts_with(&prefix.to_lowercase())
                {
                    let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);

                    // For 'cd' command, only show directories
                    if input_parts.len() > 0 && input_parts[0] == "cd" {
                        if !is_dir {
                            continue;
                        }
                    }

                    // Add trailing slash for directories
                    let suggestion = if is_dir {
                        format!("{}/", file_name_str)
                    } else {
                        file_name_str.to_string()
                    };

                    // Construct the full path suggestion for the command
                    let base_path = if dir_to_search.is_empty() {
                        "".to_string()
                    } else {
                        format!("{}/", dir_to_search.trim_end_matches('/'))
                    };

                    matches.push(format!("{}{}", base_path, suggestion));
                }
            }

            if !matches.is_empty() {
                // Sort matches alphabetically, case-insensitive
                matches.sort_by_key(|a| a.to_lowercase());
                return Ok(matches);
            }
        }
    }

    Ok(Vec::new())
}

// Helper function to split a path into directory and file prefix parts
fn split_path_prefix(path: &str) -> (&str, &str) {
    match path.rfind('/') {
        Some(index) => {
            let (dir, file) = path.split_at(index + 1);
            (dir, file)
        }
        None => ("", path),
    }
}

#[command]
fn get_working_directory(
    session_id: String,
    command_manager: State<'_, CommandManager>,
) -> Result<String, String> {
    let states = command_manager.commands.lock().map_err(|e| e.to_string())?;
    let key = session_id;

    if let Some(state) = states.get(&key) {
        if state.is_ssh_session_active {
            // Return the stored remote CWD, or a default if not yet known
            Ok(state
                .remote_current_dir
                .clone()
                .unwrap_or_else(|| "remote:~".to_string()))
        } else {
            Ok(state.current_dir.clone())
        }
    } else {
        // Fallback if session doesn't exist - create new default state
        Ok(env::current_dir()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string())
    }
}

#[command]
fn get_home_directory() -> Result<String, String> {
    dirs::home_dir()
        .map(|path| path.to_string_lossy().to_string())
        .ok_or_else(|| "Could not determine home directory".to_string())
}

// Implement the ask_ai function for Ollama integration
#[command]
async fn ask_ai(
    question: String,
    model_override: Option<String>,
    command_manager: State<'_, CommandManager>,
) -> Result<String, String> {
    // Check if this is a special command
    if question.starts_with('/') {
        return handle_special_command(question, command_manager).await;
    }

    // Regular message to Ollama
    let model;
    let api_host;

    // Scope the mutex lock to drop it before any async operations
    {
        let ollama_state = command_manager.ollama.lock().map_err(|e| e.to_string())?;
        // Use the model_override if provided, otherwise use the default
        model = model_override.unwrap_or_else(|| ollama_state.current_model.clone());
        api_host = ollama_state.api_host.clone();
        // MutexGuard is dropped here at the end of scope
    }

    // Get the current operating system
    let os = get_operating_system();

    // Create a system prompt that includes OS information and formatting instructions
    let system_prompt = format!(
        "You are a helpful terminal assistant. The user is using a {} operating system. \
        When providing terminal commands, ensure they are compatible with {}. \
        When asked for a command, respond with ONLY the command in this format: ```command```\
        The command should be a single line without any explanation or additional text.",
        os, os
    );

    // Combine the system prompt with the user's question
    let combined_prompt = format!("{}\n\nUser: {}", system_prompt, question);

    let client = reqwest::Client::new();
    let res = client
        .post(format!("{}/api/generate", api_host))
        .json(&OllamaRequest {
            model,
            prompt: combined_prompt,
            stream: false,
        })
        .send()
        .await
        .map_err(|e| format!("Failed to send request to Ollama API: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("Ollama API error: {}", res.status()));
    }

    let response: OllamaResponse = res
        .json()
        .await
        .map_err(|e| format!("Failed to parse Ollama response: {}", e))?;

    Ok(response.response)
}

// Handle special commands like /help, /models, /model
async fn handle_special_command(
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

// Add function to get models from Ollama API
#[command]
async fn get_models(command_manager: State<'_, CommandManager>) -> Result<String, String> {
    // Get the API host from the Ollama state
    let api_host;
    {
        let ollama_state = command_manager.ollama.lock().map_err(|e| e.to_string())?;
        api_host = ollama_state.api_host.clone();
    }

    // Request the list of models from Ollama
    let client = reqwest::Client::new();
    let res = client
        .get(format!("{}/api/tags", api_host))
        .send()
        .await
        .map_err(|e| format!("Failed to get models from Ollama API: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("Ollama API error: {}", res.status()));
    }

    // Parse the response
    let models: OllamaModelList = res
        .json()
        .await
        .map_err(|e| format!("Failed to parse models list: {}", e))?;

    // Format the response
    let mut result = String::from("Available models:\n");
    for model in models.models {
        result.push_str(&format!("- {} ({} bytes)\n", model.name, model.size));
    }
    Ok(result)
}

// Add function to switch model
#[command]
fn switch_model(
    model: String,
    command_manager: State<'_, CommandManager>,
) -> Result<String, String> {
    let mut ollama_state = command_manager.ollama.lock().map_err(|e| e.to_string())?;
    ollama_state.current_model = model.clone();
    Ok(format!("Switched to model: {}", model))
}

// Add function to get current API host
#[command]
fn get_host(command_manager: State<'_, CommandManager>) -> Result<String, String> {
    let ollama_state = command_manager.ollama.lock().map_err(|e| e.to_string())?;
    Ok(format!(
        "Current Ollama API host: {}",
        ollama_state.api_host
    ))
}

// Add function to set API host
#[command]
fn set_host(host: String, command_manager: State<'_, CommandManager>) -> Result<String, String> {
    let mut ollama_state = command_manager.ollama.lock().map_err(|e| e.to_string())?;
    ollama_state.api_host = host.clone();
    Ok(format!("Changed Ollama API host to: {}", host))
}

#[command]
fn get_git_branch(
    session_id: String,
    command_manager: State<'_, CommandManager>,
) -> Result<String, String> {
    let states = command_manager.commands.lock().map_err(|e| e.to_string())?;
    let key = session_id;

    let current_dir = if let Some(state) = states.get(&key) {
        &state.current_dir
    } else {
        return Ok("".to_string());
    };

    // Get current branch
    let mut cmd = new_git_command();
    cmd.arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .current_dir(current_dir);

    let output = cmd.output().map_err(|e| e.to_string())?;

    if output.status.success() {
        let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(branch)
    } else {
        Ok("".to_string())
    }
}

#[command]
fn get_git_branches(
    session_id: String,
    command_manager: State<'_, CommandManager>,
) -> Result<Vec<String>, String> {
    let states = command_manager.commands.lock().map_err(|e| e.to_string())?;
    let key = session_id;

    let current_dir = if let Some(state) = states.get(&key) {
        &state.current_dir
    } else {
        return Err("Could not determine current directory for session".to_string());
    };

    let mut cmd = new_git_command();
    cmd.arg("branch")
        .arg("-a")
        .arg("--no-color")
        .current_dir(current_dir);

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to execute git branch: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let branches = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|line| line.trim().replace("* ", "").to_string())
        .filter(|line| !line.contains("->")) // Filter out HEAD pointers
        .collect::<Vec<String>>();

    Ok(branches)
}

#[command]
fn switch_branch(
    branch_name: String,
    session_id: String,
    command_manager: State<'_, CommandManager>,
) -> Result<(), String> {
    let states = command_manager.commands.lock().map_err(|e| e.to_string())?;
    let key = session_id;

    let current_dir = if let Some(state) = states.get(&key) {
        state.current_dir.clone()
    } else {
        return Err("Could not determine current directory for session".to_string());
    };

    // 1. Check for local changes
    let mut status_cmd = new_git_command();
    status_cmd
        .arg("status")
        .arg("--porcelain")
        .current_dir(current_dir.clone());

    let status_output = status_cmd
        .output()
        .map_err(|e| format!("Failed to execute git status: {}", e))?;

    let needs_stash = !status_output.stdout.is_empty();

    if needs_stash {
        // 2. Stash changes if necessary
        let mut stash_cmd = new_git_command();
        stash_cmd.arg("stash").current_dir(current_dir.clone());

        let stash_output = stash_cmd
            .output()
            .map_err(|e| format!("Failed to execute git stash: {}", e))?;
        if !stash_output.status.success() {
            return Err(String::from_utf8_lossy(&stash_output.stderr).to_string());
        }
    }

    // 3. Checkout the new branch
    let mut checkout_cmd = new_git_command();
    checkout_cmd
        .arg("checkout")
        .arg(branch_name.clone())
        .current_dir(current_dir.clone());

    let checkout_output = checkout_cmd
        .output()
        .map_err(|e| format!("Failed to execute git checkout: {}", e))?;

    if !checkout_output.status.success() {
        // If checkout fails, try to pop stash if we created one
        if needs_stash {
            let mut stash_pop_cmd = new_git_command();
            stash_pop_cmd
                .arg("stash")
                .arg("pop")
                .current_dir(current_dir.clone());

            stash_pop_cmd.output().map_err(|e| {
                format!(
                    "Failed to execute git stash pop after failed checkout: {}",
                    e
                )
            })?;
        }
        return Err(String::from_utf8_lossy(&checkout_output.stderr).to_string());
    }

    // 4. Pop stash if changes were stashed
    if needs_stash {
        let mut stash_pop_cmd = new_git_command();
        stash_pop_cmd
            .arg("stash")
            .arg("pop")
            .current_dir(current_dir);

        let stash_pop_output = stash_pop_cmd
            .output()
            .map_err(|e| format!("Failed to execute git stash pop: {}", e))?;

        if !stash_pop_output.status.success() {
            // This is not ideal, the user has switched branch but stash pop failed.
            // We can return an error message to inform the user.
            let error_message = String::from_utf8_lossy(&stash_pop_output.stderr).to_string();
            return Err(format!(
                "Branch switched to {}, but 'git stash pop' failed: {}",
                branch_name, error_message
            ));
        }
    }

    Ok(())
}

#[tauri::command]
fn get_current_pid(
    session_id: String,
    command_manager: State<'_, CommandManager>,
) -> Result<u32, String> {
    let states = command_manager.commands.lock().map_err(|e| e.to_string())?;
    let key = session_id;

    if let Some(state) = states.get(&key) {
        Ok(state.pid.unwrap_or(0))
    } else {
        Ok(0)
    }
}

#[tauri::command]
fn terminate_command(
    session_id: String,
    command_manager: State<'_, CommandManager>,
) -> Result<(), String> {
    let mut states = command_manager.commands.lock().map_err(|e| e.to_string())?;
    let key = session_id;

    let pid = if let Some(state) = states.get(&key) {
        state.pid.unwrap_or(0)
    } else {
        return Err("No active process found".to_string());
    };

    if pid == 0 {
        return Err("No active process to terminate".to_string());
    }

    #[cfg(unix)]
    {
        use nix::sys::signal::{kill, Signal};
        use nix::unistd::Pid;

        // Try to send SIGTERM first
        if let Err(err) = kill(Pid::from_raw(pid as i32), Signal::SIGTERM) {
            return Err(format!("Failed to send SIGTERM: {}", err));
        }

        // Give the process a moment to terminate gracefully
        std::thread::sleep(std::time::Duration::from_millis(100));

        // If it's still running, force kill with SIGKILL
        if let Err(err) = kill(Pid::from_raw(pid as i32), Signal::SIGKILL) {
            return Err(format!("Failed to send SIGKILL: {}", err));
        }
    }

    // Clear the PID after successful termination
    if let Some(state) = states.get_mut(&key) {
        state.pid = None;
    }

    Ok(())
}

// Add a new command to get all system environment variables
#[tauri::command]
fn get_system_env() -> Result<Vec<(String, String)>, String> {
    let env_vars: Vec<(String, String)> = std::env::vars().collect();
    Ok(env_vars)
}

#[tauri::command]
fn git_fetch_and_pull(
    session_id: String,
    command_manager: State<'_, CommandManager>,
) -> Result<String, String> {
    let mut command_manager_guard = command_manager.commands.lock().unwrap();
    let command_state = command_manager_guard
        .get_mut(&session_id)
        .ok_or_else(|| "Session not found".to_string())?;

    let mut fetch_cmd = new_git_command();
    fetch_cmd.current_dir(&command_state.current_dir);
    fetch_cmd.arg("fetch");

    let fetch_output = fetch_cmd.output().map_err(|e| e.to_string())?;
    if !fetch_output.status.success() {
        return Err(String::from_utf8_lossy(&fetch_output.stderr).to_string());
    }

    let mut pull_cmd = new_git_command();
    pull_cmd.current_dir(&command_state.current_dir);
    pull_cmd.arg("pull");

    let pull_output = pull_cmd.output().map_err(|e| e.to_string())?;
    if !pull_output.status.success() {
        return Err(String::from_utf8_lossy(&pull_output.stderr).to_string());
    }

    let mut output = String::new();
    output.push_str("Fetch output:\\n");
    output.push_str(&String::from_utf8_lossy(&fetch_output.stdout));
    output.push_str(&String::from_utf8_lossy(&fetch_output.stderr));
    output.push_str("\\nPull output:\\n");
    output.push_str(&String::from_utf8_lossy(&pull_output.stdout));
    output.push_str(&String::from_utf8_lossy(&pull_output.stderr));

    Ok(output)
}

#[tauri::command]
fn git_commit_and_push(
    session_id: String,
    message: String,
    command_manager: State<'_, CommandManager>,
) -> Result<String, String> {
    let mut command_manager_guard = command_manager.commands.lock().unwrap();
    let command_state = command_manager_guard
        .get_mut(&session_id)
        .ok_or_else(|| "Session not found".to_string())?;

    let mut add_cmd = new_git_command();
    add_cmd.current_dir(&command_state.current_dir);
    add_cmd.arg("add").arg(".");
    let add_output = add_cmd.output().map_err(|e| e.to_string())?;
    if !add_output.status.success() {
        return Err(String::from_utf8_lossy(&add_output.stderr).to_string());
    }

    let mut commit_cmd = new_git_command();
    commit_cmd.current_dir(&command_state.current_dir);
    commit_cmd.arg("commit").arg("-m").arg(&message);
    let commit_output = commit_cmd.output().map_err(|e| e.to_string())?;
    if !commit_output.status.success() {
        return Err(String::from_utf8_lossy(&commit_output.stderr).to_string());
    }

    let mut push_cmd = new_git_command();
    push_cmd.current_dir(&command_state.current_dir);
    push_cmd.arg("push");
    let push_output = push_cmd.output().map_err(|e| e.to_string())?;
    if !push_output.status.success() {
        return Err(String::from_utf8_lossy(&push_output.stderr).to_string());
    }

    let mut output = String::new();
    output.push_str("Commit output:\\n");
    output.push_str(&String::from_utf8_lossy(&commit_output.stdout));
    output.push_str(&String::from_utf8_lossy(&commit_output.stderr));
    output.push_str("\\nPush output:\\n");
    output.push_str(&String::from_utf8_lossy(&push_output.stdout));
    output.push_str(&String::from_utf8_lossy(&push_output.stderr));

    Ok(output)
}

#[tauri::command]
fn get_github_remote_and_branch(
    session_id: String,
    command_manager: tauri::State<'_, CommandManager>,
) -> Result<serde_json::Value, String> {
    let states = command_manager.commands.lock().map_err(|e| e.to_string())?;
    let key = session_id;
    let current_dir = if let Some(state) = states.get(&key) {
        &state.current_dir
    } else {
        return Err("Could not determine current directory for session".to_string());
    };

    // Get remote URL
    let mut remote_cmd = new_git_command();
    remote_cmd
        .arg("remote")
        .arg("get-url")
        .arg("origin")
        .current_dir(current_dir);
    let remote_output = remote_cmd.output().map_err(|e| e.to_string())?;
    if !remote_output.status.success() {
        return Err(String::from_utf8_lossy(&remote_output.stderr).to_string());
    }
    let remote_url = String::from_utf8_lossy(&remote_output.stdout)
        .trim()
        .to_string();

    // Get branch name
    let mut branch_cmd = new_git_command();
    branch_cmd
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .current_dir(current_dir);
    let branch_output = branch_cmd.output().map_err(|e| e.to_string())?;
    if !branch_output.status.success() {
        return Err(String::from_utf8_lossy(&branch_output.stderr).to_string());
    }
    let branch = String::from_utf8_lossy(&branch_output.stdout)
        .trim()
        .to_string();

    Ok(serde_json::json!({ "remoteUrl": remote_url, "branch": branch }))
}

fn main() {
    let _ = fix_path_env::fix();
    // Create a new command manager
    let command_manager = CommandManager::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|_app| {
            // Add any setup logic here
            Ok(())
        })
        .manage(command_manager)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            command::core::execute_command::execute_command,
            command::core::execute_command::execute_sudo_command,
            terminate_command,
            get_current_pid,
            autocomplete,
            get_working_directory,
            get_home_directory,
            ask_ai,
            get_models,
            switch_model,
            get_host,
            set_host,
            get_git_branch,
            get_git_branches,
            switch_branch,
            get_system_env,
            git_fetch_and_pull,
            git_commit_and_push,
            get_github_remote_and_branch,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
