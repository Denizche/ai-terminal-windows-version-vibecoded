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
