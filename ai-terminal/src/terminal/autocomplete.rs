use crate::config::{COMMON_COMMANDS, PATH_COMMANDS};
use crate::model::App;
use std::fs;

impl App {
    // Get autocomplete suggestions based on current input
    pub fn get_autocomplete_suggestions(&mut self) -> Vec<String> {
        let input = self.input.clone();
        println!("[autocomplete] Getting autocomplete suggestions for input: '{}'", input);
        let mut suggestions = Vec::new();

        // If input is empty, return empty suggestions
        if input.is_empty() {
            println!("[autocomplete] Input empty, returning no suggestions");
            return suggestions;
        }

        // Split input into parts
        let parts: Vec<&str> = input.split_whitespace().collect();
        println!("[autocomplete] Input parts: {:?}", parts);

        // Check if we're trying to autocomplete a path (for cd, ls, etc.)
        if parts.len() >= 2 && PATH_COMMANDS.contains(&parts[0]) {
            let command = parts[0];
            let path_part = if parts.len() > 1 {
                // Get the last part which is being typed
                parts.last().unwrap()
            } else {
                ""
            };
            
            println!("[autocomplete] Path completion for command '{}' with path part: '{}'", command, path_part);

            // For cd command, only suggest directories
            if command == "cd" {
                suggestions = self
                    .get_path_suggestions(path_part)
                    .into_iter()
                    .filter(|s| s.ends_with('/'))
                    .collect();
            } else {
                // For other commands, suggest both files and directories
                suggestions = self.get_path_suggestions(path_part);
            }

            // Format suggestions to include the command and any intermediate arguments
            if parts.len() > 2 {
                let prefix = parts[..parts.len() - 1].join(" ") + " ";
                println!("[autocomplete] Multi-part command, using prefix: '{}'", prefix);
                suggestions = suggestions
                    .into_iter()
                    .map(|s| format!("{}{}", prefix, s))
                    .collect();
            } else if parts.len() == 2 {
                let prefix = format!("{} ", command);
                println!("[autocomplete] Two-part command, using prefix: '{}'", prefix);
                suggestions = suggestions
                    .into_iter()
                    .map(|s| format!("{}{}", prefix, s))
                    .collect();
            }
        } else if !input.contains(' ') {
            println!("[autocomplete] Command completion for: '{}'", input);
            // We're at the beginning of a command (no space yet)
            // Common Unix commands for autocompletion
            for cmd in COMMON_COMMANDS.iter() {
                if cmd.starts_with(&input) {
                    println!("[autocomplete] Found common command match: '{}'", cmd);
                    suggestions.push(cmd.to_string());
                }
            }

            // Also add commands from history
            for cmd in &self.command_history {
                let cmd_part = cmd.split_whitespace().next().unwrap_or("");
                if cmd_part.starts_with(&input) && !suggestions.contains(&cmd_part.to_string()) {
                    println!("[autocomplete] Found history match: '{}'", cmd_part);
                    suggestions.push(cmd_part.to_string());
                }
            }
        }

        suggestions.sort();
        println!("[autocomplete] Returning {} suggestions: {:?}", suggestions.len(), suggestions);
        suggestions
    }

    // Get path suggestions for cd command
    pub fn get_path_suggestions(&self, path_part: &str) -> Vec<String> {
        println!("[autocomplete] Getting path suggestions for: '{}'", path_part);
        let mut suggestions = Vec::new();

        // Determine the directory to search in and the prefix to match
        let (search_dir, prefix) = if path_part.is_empty() {
            // If no path specified, suggest directories in current directory
            println!("[autocomplete] Empty path, searching in current directory: {:?}", self.current_dir);
            (self.current_dir.clone(), "".to_string())
        } else if path_part == "~" {
            // Suggest home directory
            if let Some(home) = dirs_next::home_dir() {
                println!("[autocomplete] Home path, searching in: {:?}", home);
                (home, "~".to_string())
            } else {
                println!("[autocomplete] Home directory not found");
                return suggestions;
            }
        } else if path_part.starts_with("~/") {
            // Suggest in home directory with subdirectory
            if let Some(home) = dirs_next::home_dir() {
                let subdir = path_part.trim_start_matches("~/");
                let last_slash = subdir.rfind('/').unwrap_or(0);
                let (dir_part, file_prefix) = if last_slash == 0 {
                    (subdir, "")
                } else {
                    subdir.split_at(last_slash)
                };
                
                println!("[autocomplete] Home subdirectory path: ~/{}, dir_part: '{}', file_prefix: '{}'", 
                    subdir, dir_part, file_prefix);

                let search_path = if dir_part.is_empty() {
                    home.clone()
                } else {
                    home.join(dir_part)
                };

                (
                    search_path,
                    format!(
                        "~/{}{}",
                        dir_part,
                        if !dir_part.is_empty() && !dir_part.ends_with('/') {
                            "/"
                        } else {
                            ""
                        }
                    ),
                )
            } else {
                println!("[autocomplete] Home directory not found");
                return suggestions;
            }
        } else if path_part.starts_with('/') {
            // Absolute path
            let last_slash = path_part.rfind('/').unwrap_or(0);
            let (dir_part, file_prefix) = path_part.split_at(last_slash + 1);
            
            println!("[autocomplete] Absolute path: '{}', dir_part: '{}', file_prefix: '{}'", 
                path_part, dir_part, file_prefix);
            
            (std::path::PathBuf::from(dir_part), dir_part.to_string())
        } else {
            // Relative path
            let last_slash = path_part.rfind('/').unwrap_or(0);
            let (dir_part, file_prefix) = if last_slash == 0 {
                ("", path_part)
            } else {
                path_part.split_at(last_slash + 1)
            };
            
            println!("[autocomplete] Relative path: '{}', dir_part: '{}', file_prefix: '{}'", 
                path_part, dir_part, file_prefix);

            let search_path = if dir_part.is_empty() {
                self.current_dir.clone()
            } else {
                self.current_dir.join(dir_part)
            };

            (search_path, dir_part.to_string())
        };

        println!("[autocomplete] Will search in directory: {:?} with prefix: '{}'", search_dir, prefix);

        // Get the part after the last slash to match against
        let match_prefix = if let Some(last_slash) = path_part.rfind('/') {
            &path_part[last_slash + 1..]
        } else {
            path_part
        };
        
        println!("[autocomplete] Matching entries against prefix: '{}'", match_prefix);

        // Read the directory and find matching entries
        if let Ok(entries) = fs::read_dir(&search_dir) {
            for entry in entries.flatten() {
                if let Ok(file_name) = entry.file_name().into_string() {
                    // Check if the file name starts with our prefix
                    if file_name.starts_with(match_prefix) {
                        if let Ok(file_type) = entry.file_type() {
                            let suggestion = if file_type.is_dir() {
                                // Add trailing slash for directories
                                format!("{}{}/", prefix, file_name)
                            } else {
                                // Regular file
                                format!("{}{}", prefix, file_name)
                            };
                            println!("[autocomplete] Adding suggestion: '{}'", suggestion);
                            suggestions.push(suggestion);
                        }
                    }
                }
            }
        }

        // Add special directories if they match
        if ".".starts_with(match_prefix) {
            println!("[autocomplete] Adding special directory: './'");
            suggestions.push(format!("{}./", prefix));
        }
        if "..".starts_with(match_prefix) {
            println!("[autocomplete] Adding special directory: '../'");
            suggestions.push(format!("{}../", prefix));
        }

        println!("[autocomplete] Found {} path suggestions", suggestions.len());
        suggestions
    }
}