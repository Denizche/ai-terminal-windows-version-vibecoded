use crate::command::types::command_manager::CommandManager;
use std::env;
use std::process::Command;
use tauri::{command, State};

pub fn get_shell_path() -> Option<String> {
    // First try to get the user's default shell
    let shell = if cfg!(target_os = "windows") {
        "cmd"
    } else {
        // Try to get the user's default shell from /etc/shells or fallback to common shells
        let shells = ["/bin/zsh", "/bin/bash", "/bin/sh"];
        for shell in shells.iter() {
            if std::path::Path::new(shell).exists() {
                return Some(shell.to_string());
            }
        }
        "sh" // Fallback
    };

    // Try to get PATH using the shell's login mode and sourcing initialization files
    let command = if shell.contains("zsh") {
        "source ~/.zshrc 2>/dev/null || true; source ~/.zshenv 2>/dev/null || true; echo $PATH"
    } else if shell.contains("bash") {
        "source ~/.bashrc 2>/dev/null || true; source ~/.bash_profile 2>/dev/null || true; echo $PATH"
    } else {
        "echo $PATH"
    };

    let output = Command::new(shell)
        .arg("-l") // Login shell to get proper environment
        .arg("-c")
        .arg(command)
        .output()
        .ok()?;

    if output.status.success() {
        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !path.is_empty() {
            return Some(path);
        }
    }

    // If shell method fails, try to get PATH from environment
    std::env::var("PATH").ok()
}

#[command]
pub fn get_working_directory(
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
pub fn get_home_directory() -> Result<String, String> {
    dirs::home_dir()
        .map(|path| path.to_string_lossy().to_string())
        .ok_or_else(|| "Could not determine home directory".to_string())
}

// Helper function to split a path into directory and file prefix parts
pub fn split_path_prefix(path: &str) -> (&str, &str) {
    match path.rfind('/') {
        Some(index) => {
            let (dir, file) = path.split_at(index + 1);
            (dir, file)
        }
        None => ("", path),
    }
}
