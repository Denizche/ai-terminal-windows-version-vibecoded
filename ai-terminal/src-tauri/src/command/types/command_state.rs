use std::process::Child;
use std::sync::{Arc, Mutex};

// Store the current working directory for each command
pub struct CommandState {
    pub current_dir: String,
    pub child_wait_handle: Option<Arc<Mutex<Child>>>, // For wait() and kill()
    pub child_stdin: Option<Arc<Mutex<std::process::ChildStdin>>>, // For writing
    pub pid: Option<u32>,
    pub is_ssh_session_active: bool, // Added for persistent SSH
    pub remote_current_dir: Option<String>, // New field for remote SSH path
}