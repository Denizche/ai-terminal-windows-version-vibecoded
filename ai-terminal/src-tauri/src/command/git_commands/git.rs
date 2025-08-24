use crate::command::types::command_manager::CommandManager;
use crate::utils::file_system_utils::get_shell_path;
use std::process::Command;
use tauri::{command, State};

pub fn new_git_command() -> Command {
    let mut cmd = Command::new("git");
    if let Some(path_val) = get_shell_path() {
        if let Ok(current_path) = std::env::var("PATH") {
            let new_path = format!("{}{}{}", path_val, std::path::MAIN_SEPARATOR, current_path);
            cmd.env("PATH", new_path);
        } else {
            cmd.env("PATH", path_val);
        }
    }
    cmd
}

#[command]
pub fn get_git_branch(
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
pub fn get_git_branches(
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
pub fn switch_branch(
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
pub fn git_fetch_and_pull(
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
pub fn git_commit_and_push(
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
pub fn get_github_remote_and_branch(
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
