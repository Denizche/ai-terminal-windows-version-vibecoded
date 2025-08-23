use crate::command::utils::path_utils::get_shell_path;
use std::process::Command;

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
