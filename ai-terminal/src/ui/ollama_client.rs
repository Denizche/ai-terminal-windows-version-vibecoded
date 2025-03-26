use crate::model::App as AppState;

pub fn create_ollama_context(state: &AppState, query: &str) -> String {
    format!(
        "System Info: {}\n\nRecent Terminal Output:\n{}\n\nRecent Chat History:\n{}\n\nUser query: {}\n\nCurrent directory: {}",
        state.os_info,
        state.output.iter().rev().take(20).map(String::as_str).collect::<Vec<_>>().join("\n"),
        state.ai_output.iter().rev().take(10).map(String::as_str).collect::<Vec<_>>().join("\n"),
        query,
        state.current_dir.display()
    )
} 