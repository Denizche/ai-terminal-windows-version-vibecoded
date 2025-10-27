use crate::command::types::command_manager::CommandManager;
use std::env;
use std::process::Command;
use tauri::{command, State};

pub fn get_shell_path() -> Option<String> {
    #[cfg(windows)]
    {
        // On Windows, we can try to get PATH from various shells
        use crate::utils::windows_utils::detect_windows_shell;
        
        let shell_type = detect_windows_shell();
        
        let (shell_exe, args, command) = match shell_type.as_str() {
            "powershell" => ("powershell", vec!["-Command"], "$env:PATH"),
            "bash" => ("bash", vec!["-c"], "echo $PATH"),
            _ => ("cmd", vec!["/C"], "echo %PATH%"),
        };
        
        let output = Command::new(shell_exe)
            .args(&args)
            .arg(command)
            .output()
            .ok()?;
        
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return Some(path);
            }
        }
        
        // Fallback to environment PATH
        env::var("PATH").ok()
    }
    
    #[cfg(unix)]
    {
        // Try to get the user's default shell from /etc/shells or fallback to common shells
        let shells = ["/bin/zsh", "/bin/bash", "/bin/sh"];
        let shell = shells.iter()
            .find(|shell| std::path::Path::new(shell).exists())
            .map(|s| *s)
            .unwrap_or("sh");

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

        // If the shell method fails, try to get PATH from the environment
        env::var("PATH").ok()
    }
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
        // Fallback if the session doesn't exist - create a new default state
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
    // Handle both Unix (/) and Windows (\) path separators
    let separator_pos = if cfg!(windows) {
        path.rfind('\\').or_else(|| path.rfind('/'))
    } else {
        path.rfind('/')
    };
    
    match separator_pos {
        Some(index) => {
            let (dir, file) = path.split_at(index + 1);
            (dir, file)
        }
        None => ("", path),
    }
}
