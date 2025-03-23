//! Prompt engineering and context management functionality for Ollama API

// Maximum context size - adjust as needed
pub const MAX_CONTEXT_SIZE: usize = 4000;

/// Trims the context to a reasonable size while preserving
/// the most important information
pub fn trim_context(prompt: &str) -> String {
    // If prompt is already smaller than limit, return as is
    if prompt.len() <= MAX_CONTEXT_SIZE {
        return prompt.to_string();
    }
    
    // Split the prompt into sections
    let parts: Vec<&str> = prompt.split("\n\n").collect();
    
    // Always keep system info and user query
    let mut essential_parts = Vec::new();
    let mut user_query = String::new();
    let mut system_info = String::new();
    
    // Find and extract essential parts
    for part in &parts {
        if part.to_lowercase().starts_with("system info:") {
            system_info = part.to_string();
        } else if part.to_lowercase().starts_with("user query:") {
            user_query = part.to_string();
        }
    }
    
    if !system_info.is_empty() {
        essential_parts.push(system_info);
    }
    
    // Include the most recent terminal output, but limit it
    if let Some(terminal_index) = prompt.to_lowercase().find("recent terminal output:") {
        let terminal_section = &prompt[terminal_index..];
        if let Some(end_index) = terminal_section.find("\n\n") {
            let terminal_content = &terminal_section[..end_index];
            
            // Get only the last few lines (max 10)
            let lines: Vec<&str> = terminal_content.lines().collect();
            let start_idx = if lines.len() > 12 { lines.len() - 10 } else { 2 }; // Skip header
            
            let mut trimmed_terminal = "Recent Terminal Output:\n".to_string();
            for line in &lines[start_idx..] {
                trimmed_terminal.push_str(line);
                trimmed_terminal.push('\n');
            }
            
            essential_parts.push(trimmed_terminal);
        }
    }
    
    // Always include recent chat
    if let Some(chat_index) = prompt.to_lowercase().find("recent chat history:") {
        let chat_section = &prompt[chat_index..];
        if let Some(end_index) = chat_section.find("\n\n") {
            let chat_content = &chat_section[..end_index];
            
            // Get only the last few chat messages (max 5)
            let lines: Vec<&str> = chat_content.lines().collect();
            let start_idx = if lines.len() > 7 { lines.len() - 5 } else { 2 }; // Skip header
            
            let mut trimmed_chat = "Recent Chat History:\n".to_string();
            for line in &lines[start_idx..] {
                trimmed_chat.push_str(line);
                trimmed_chat.push('\n');
            }
            
            essential_parts.push(trimmed_chat);
        }
    }
    
    // Always include user query last
    if !user_query.is_empty() {
        essential_parts.push(user_query);
    } else if let Some(query_index) = prompt.to_lowercase().find("user query:") {
        // Extract user query if not found earlier
        let query_content = &prompt[query_index..];
        if let Some(end_index) = query_content.find("\n\n") {
            essential_parts.push(query_content[..end_index].to_string());
        } else {
            essential_parts.push(query_content.to_string());
        }
    }
    
    // Include current directory if present
    if let Some(dir_index) = prompt.to_lowercase().find("current directory:") {
        let dir_content = &prompt[dir_index..];
        if let Some(end_index) = dir_content.find('\n') {
            essential_parts.push(dir_content[..end_index].to_string());
        } else {
            essential_parts.push(dir_content.to_string());
        }
    }
    
    // Combine essential parts with double newlines
    let result = essential_parts.join("\n\n");
    
    // Final safety check - if still too long, truncate
    if result.len() > MAX_CONTEXT_SIZE {
        let mut truncated = result;
        truncated.truncate(MAX_CONTEXT_SIZE);
        
        // Ensure we don't cut in the middle of the user query
        if let Some(query_index) = truncated.to_lowercase().rfind("user query:") {
            truncated.truncate(query_index);
            truncated.push_str("\n\n");
            
            // Add back the user query
            if let Some(query_index) = prompt.to_lowercase().find("user query:") {
                let query_content = &prompt[query_index..];
                if let Some(end_index) = query_content.find("\n\n") {
                    truncated.push_str(&query_content[..end_index]);
                } else {
                    truncated.push_str(query_content);
                }
            }
        }
        
        return truncated;
    }
    
    result
}

/// Extracts just the user query from a context-rich prompt
/// and adds minimal essential context
pub fn extract_user_query(prompt: &str) -> String {
    // Create a minimal context with essential information
    let mut minimal_context = Vec::new();
    
    // 1. Add essential system info if present
    if let Some(sys_info_index) = prompt.to_lowercase().find("system info:") {
        let sys_info = &prompt[sys_info_index..];
        if let Some(end_index) = sys_info.find('\n') {
            // Just take the first line of system info
            minimal_context.push(sys_info[..end_index].trim().to_string());
        }
    }
    
    // 2. Add current directory if present
    if let Some(dir_index) = prompt.to_lowercase().find("current directory:") {
        let dir_info = &prompt[dir_index..];
        if let Some(end_index) = dir_info.find('\n') {
            minimal_context.push(dir_info[..end_index].trim().to_string());
        }
    }
    
    // 3. Extract the last command if present
    if let Some(terminal_index) = prompt.to_lowercase().find("recent terminal output:") {
        let terminal_section = &prompt[terminal_index..];
        if let Some(end_index) = terminal_section.find("\n\n") {
            let terminal_content = &terminal_section[..end_index];
            
            // Get the last command line (starts with ">")
            let lines: Vec<&str> = terminal_content.lines().collect();
            for line in lines.iter().rev() {
                if line.trim().starts_with(">") {
                    minimal_context.push(format!("Last command: {}", line.trim()));
                    break;
                }
            }
        }
    }
    
    // 4. Extract user query (most important part)
    let user_query = if let Some(user_query_index) = prompt.to_lowercase().find("user query:") {
        let remaining = &prompt[user_query_index..];
        let query_content = if let Some(end_index) = remaining.find("\n\n") {
            &remaining[..end_index]
        } else {
            remaining
        };
        query_content.trim().to_string()
    } else {
        // Fallback if we can't find the user query section
        prompt.lines().last().unwrap_or("").trim().to_string()
    };
    
    // Always include the user query
    minimal_context.push(user_query);
    
    // Join with double newlines for better readability
    let result = minimal_context.join("\n\n");
    
    // Ensure we don't exceed a reasonable size for the minimal context
    if result.len() > 1000 {
        // If too long, prioritize the user query
        if let Some(user_query_index) = result.to_lowercase().rfind("user query:") {
            return result[user_query_index..].trim().to_string();
        } else {
            return result[(result.len() - 1000)..].trim().to_string();
        }
    }
    
    result
} 