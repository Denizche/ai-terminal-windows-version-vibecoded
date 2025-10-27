use std::env;
use std::path::PathBuf;

#[cfg(windows)]
use winapi::um::processthreadsapi::GetCurrentProcessId;

/// Detects the current shell/terminal on Windows
pub fn detect_windows_shell() -> String {
    // Check if running in Windows Terminal
    if env::var("WT_SESSION").is_ok() {
        return "wt".to_string();
    }
    
    // Check if running in PowerShell
    if env::var("PSModulePath").is_ok() {
        return "powershell".to_string();
    }
    
    // Check if running in Git Bash
    if env::var("MSYSTEM").is_ok() {
        return "bash".to_string();
    }
    
    // Default to cmd
    "cmd".to_string()
}

/// Get the appropriate shell command for Windows
pub fn get_windows_shell_command() -> Vec<String> {
    let shell = detect_windows_shell();
    
    match shell.as_str() {
        "powershell" => vec!["powershell".to_string(), "-Command".to_string()],
        "bash" => vec!["bash".to_string(), "-c".to_string()],
        "wt" | "cmd" | _ => vec!["cmd".to_string(), "/C".to_string()],
    }
}

/// Convert Unix-style paths to Windows paths
pub fn convert_unix_path_to_windows(path: &str) -> String {
    if path.starts_with('/') {
        // Handle absolute Unix paths - convert /c/users/... to C:\users\...
        if path.len() > 2 && path.chars().nth(2) == Some('/') {
            let drive = path.chars().nth(1).unwrap().to_uppercase().collect::<String>();
            let rest = &path[3..].replace('/', "\\");
            format!("{}:\\{}", drive, rest)
        } else {
            // For other absolute paths, assume they're WSL paths
            path.replace('/', "\\")
        }
    } else {
        // Handle relative paths and Windows paths
        path.replace('/', "\\")
    }
}

/// Convert tilde (~) to Windows home directory
pub fn expand_windows_tilde(path: &str) -> String {
    if path.starts_with('~') {
        if let Some(home) = dirs::home_dir() {
            let home_str = home.to_string_lossy();
            if path == "~" {
                home_str.to_string()
            } else {
                format!("{}{}", home_str, &path[1..].replace('/', "\\"))
            }
        } else {
            path.to_string()
        }
    } else {
        path.to_string()
    }
}

/// Get current process ID (Windows-specific implementation)
#[cfg(windows)]
pub fn get_current_process_id() -> u32 {
    unsafe { GetCurrentProcessId() }
}

/// Check if a path is absolute on Windows
pub fn is_windows_absolute_path(path: &str) -> bool {
    // Windows absolute paths start with drive letter (C:) or UNC (\\)
    if path.len() >= 2 {
        let chars: Vec<char> = path.chars().collect();
        (chars[1] == ':' && chars[0].is_alphabetic()) || path.starts_with("\\\\")
    } else {
        false
    }
}

/// Normalize Windows path separators
pub fn normalize_windows_path(path: &str) -> String {
    path.replace('/', "\\")
}

/// Get Windows Terminal executable path if available
pub fn get_windows_terminal_path() -> Option<PathBuf> {
    // Check if Windows Terminal is installed
    if let Ok(output) = std::process::Command::new("where")
        .args(&["wt.exe"])
        .output()
    {
        if output.status.success() {
            let path_string = String::from_utf8_lossy(&output.stdout).trim().to_string();
            return Some(PathBuf::from(path_string));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_unix_path_to_windows() {
        assert_eq!(convert_unix_path_to_windows("/c/users/test"), "C:\\users\\test");
        assert_eq!(convert_unix_path_to_windows("./relative/path"), ".\\relative\\path");
        assert_eq!(convert_unix_path_to_windows("relative/path"), "relative\\path");
    }

    #[test]
    fn test_is_windows_absolute_path() {
        assert!(is_windows_absolute_path("C:\\Windows"));
        assert!(is_windows_absolute_path("D:\\"));
        assert!(is_windows_absolute_path("\\\\server\\share"));
        assert!(!is_windows_absolute_path("relative\\path"));
        assert!(!is_windows_absolute_path("./relative"));
    }

    #[test]
    fn test_normalize_windows_path() {
        assert_eq!(normalize_windows_path("path/to/file"), "path\\to\\file");
        assert_eq!(normalize_windows_path("path\\to\\file"), "path\\to\\file");
    }
}