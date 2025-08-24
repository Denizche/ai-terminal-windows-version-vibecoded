use std::process::Command;

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
