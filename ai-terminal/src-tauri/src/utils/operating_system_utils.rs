use crate::command::types::command_manager::CommandManager;
use tauri::State;

// Add a helper function to get the OS information
pub fn get_operating_system() -> String {
    #[cfg(target_os = "windows")]
    return "Windows".to_string();

    #[cfg(target_os = "macos")]
    return "macOS".to_string();

    #[cfg(target_os = "linux")]
    return "Linux".to_string();

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    return "Unknown".to_string();
}

#[tauri::command]
pub fn get_system_environment_variables() -> Result<Vec<(String, String)>, String> {
    let env_vars: Vec<(String, String)> = std::env::vars().collect();
    Ok(env_vars)
}

#[tauri::command]
pub fn get_current_pid(
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
