use std::process::Command;
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

// New method to spawn a command with streaming output
fn spawn_streaming_command(&mut self, command: String, command_index: usize) {
    let (tx, rx) = mpsc::channel();
    
    let command_clone = command.clone();
    let current_dir = self.current_dir.clone();
    
    // Create a channel for user input
    let (input_tx, input_rx) = mpsc::channel::<String>();
    let input_tx_clone = input_tx.clone();
    
    // Send an initial output to force display refresh
    tx.send(format!("Running command: {}", command)).ok();
    
    // Detect if this is a directory listing command
    let is_ls_command = command.trim() == "ls" || command.trim().starts_with("ls ");
    let buffer_size = if is_ls_command { 500 } else { 1 };
    
    // Helper function to process output streams
    fn handle_stream(stream: impl BufRead, tx: mpsc::Sender<String>, is_ls_command: bool, buffer_size: usize) {
        let mut buffer = Vec::with_capacity(buffer_size);
        
        for line in stream.lines() {
            if let Ok(line) = line {
                if line.is_empty() {
                    continue;
                }
                
                // For ls commands, buffer the output to reduce UI updates
                if is_ls_command {
                    buffer.push(line);
                    if buffer.len() >= buffer_size {
                        // Send batch of lines
                        for l in buffer.drain(..) {
                            tx.send(l).ok();
                        }
                        // Small delay between batches to let UI catch up
                        std::thread::sleep(std::time::Duration::from_millis(10));
                    }
                } else {
                    // For other commands, send each line immediately
                    tx.send(line).ok();
                    // Brief pause to allow UI to catch up
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
            }
        }
        
        // Send any remaining buffered lines
        for l in buffer {
            tx.send(l).ok();
        }
    }
    
    // Check if this is a sudo command, but don't immediately enable password mode
    thread::spawn(move || {
        let parts: Vec<&str> = command_clone.split_whitespace().collect();
        
        let mut cmd = if parts[0] == "sudo" {
            let mut cmd = Command::new("sudo");
            
            // First check if sudo needs a password with -n flag
            let needs_password = {
                let mut check_cmd = Command::new("sudo");
                check_cmd.arg("-n"); // Non-interactive - will fail if password is needed
                check_cmd.arg("true");
                !check_cmd.status().map(|s| s.success()).unwrap_or(false)
            };
            
            // If password is needed, send a message to enable password mode
            if needs_password {
                tx.send("[sudo] password required:".to_string()).ok();
            }
            
            // Configure sudo command
            cmd.arg("-S"); // Read from stdin
            if parts.len() > 1 {
                cmd.args(&parts[1..]);
            }
            cmd
        } else {
            let mut cmd = Command::new(parts[0]);
            if parts.len() > 1 {
                cmd.args(&parts[1..]);
            }
            cmd
        };

        cmd.current_dir(&current_dir)
           .stdout(Stdio::piped())
           .stderr(Stdio::piped())
           .stdin(Stdio::piped());
           
        match cmd.spawn() {
            Ok(mut child) => {
                let stdout = child.stdout.take().expect("Failed to open stdout");
                let stderr = child.stderr.take().expect("Failed to open stderr");
                let stdin = child.stdin.take().expect("Failed to open stdin");
                
                // Thread to handle user input
                thread::spawn(move || {
                    let mut stdin = stdin;
                    while let Ok(input) = input_rx.recv() {
                        writeln!(stdin, "{}", input).ok();
                        stdin.flush().ok();
                    }
                });

                // Thread for stdout
                let stdout_tx = tx.clone();
                thread::spawn(move || {
                    handle_stream(BufReader::new(stdout), stdout_tx, is_ls_command, buffer_size);
                });

                // Thread for stderr
                let stderr_tx = tx.clone();
                thread::spawn(move || {
                    handle_stream(BufReader::new(stderr), stderr_tx, false, 1);
                });

                // Wait for the command to finish
                let status_tx = tx.clone();
                thread::spawn(move || {
                    // Wait for the process to complete
                    match child.wait() {
                        Ok(status) => {
                            // Send completion message
                            status_tx.send(format!("__COMMAND_COMPLETE__{}", status.success())).ok();
                        },
                        Err(_) => {
                            // Error waiting for process
                            status_tx.send("__COMMAND_COMPLETE__false".to_string()).ok();
                        }
                    }
                });
            }
            Err(e) => {
                tx.send(format!("Failed to execute command: {}", e)).ok();
                tx.send("__COMMAND_COMPLETE__false".to_string()).ok();
            }
        }
    });
    
    self.command_receiver = Some((
        Arc::new(Mutex::new(rx)),
        command_index,
        command,
        Vec::new(),
        input_tx_clone
    ));
}

// New method to poll for command output
pub fn poll_command_output(&mut self) -> Option<IcedCommand<Message>> {
    // Check if there's an active command
    if let Some((rx, command_index, command, output_lines, _input_tx)) = &self.command_receiver {
        // Try to receive a message without taking ownership
        let result = {
            let rx_lock = rx.lock().unwrap();
            rx_lock.try_recv()
        };
        
        match result {
            Ok(line) => {
                // Check for password prompts - consolidated pattern matching
                let password_patterns = ["[sudo] password", "Password:", "password:", "password for", "password di", "password per"];
                let is_password_prompt = password_patterns.iter().any(|pattern| line.contains(pattern));
                
                if is_password_prompt {
                    self.password_mode = true;
                    self.output.push(line.clone());
                    return Some(scrollable_container::scroll_to_bottom());
                }
                
                // Process command output
                if line.starts_with("__COMMAND_COMPLETE__") {
                    // Command is done
                    let success = line.strip_prefix("__COMMAND_COMPLETE__").unwrap() == "true";
                    if *command_index < self.command_status.len() {
                        self.command_status[*command_index] = if success {
                            CommandStatus::Success
                        } else {
                            CommandStatus::Failure
                        };
                    }
                    
                    // Store context and clean up
                    self.last_terminal_context = Some((command.clone(), output_lines.clone()));
                    self.password_mode = false;
                    self.command_receiver = None;
                    
                    // Force UI update
                    return Some(scrollable_container::scroll_to_bottom());
                } else if !line.is_empty() {
                    // Regular output, add to terminal
                    self.output.push(line.clone());
                    
                    // Update our stored output lines
                    if let Some((_, _, _, lines, _)) = &mut self.command_receiver {
                        lines.push(line);
                    }
                    
                    // Force UI update
                    return Some(scrollable_container::scroll_to_bottom());
                }
                
                None
            }
            Err(mpsc::TryRecvError::Empty) => None,
            Err(_) => {
                // Channel closed unexpectedly
                if *command_index < self.command_status.len() {
                    self.command_status[*command_index] = CommandStatus::Failure;
                }
                
                self.output.push("Error: Command execution terminated unexpectedly".to_string());
                self.command_receiver = None;
                self.password_mode = false;
                
                // Force UI update
                return Some(scrollable_container::scroll_to_bottom());
            }
        }
    } else {
        None
    }
} 