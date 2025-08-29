extern crate fix_path_env;

use ai_terminal_lib::command::types::command_manager::CommandManager;
use ai_terminal_lib::{command, ollama, utils};
use std::env;

fn main() {
    let _ = fix_path_env::fix();

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
            command::core::terminate_command::terminate_command,
            utils::operating_system_utils::get_current_pid,
            command::autocomplete::autocomplete_command::autocomplete,
            utils::file_system_utils::get_working_directory,
            utils::file_system_utils::get_home_directory,
            ollama::model_request::request::ask_ai,
            ollama::model_request::request::get_models,
            ollama::model_request::request::switch_model,
            ollama::model_request::request::get_host,
            ollama::model_request::request::set_host,
            command::git_commands::git::get_git_branch,
            command::git_commands::git::get_git_branches,
            command::git_commands::git::switch_branch,
            utils::operating_system_utils::get_system_environment_variables,
            command::git_commands::git::git_fetch_and_pull,
            command::git_commands::git::git_commit_and_push,
            command::git_commands::git::get_github_remote_and_branch,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
