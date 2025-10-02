// 88code-desktop modules
mod claude_config;
mod codex_config;
mod commands;
mod config;
mod env_manager;
mod vscode;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::configure_claude_code,
            commands::configure_codex,
            commands::get_config_paths,
            commands::get_current_claude_config,
            commands::get_current_codex_auth,
            commands::configure_vscode_claude,
            commands::configure_vscode_codex,
            commands::get_vscode_paths,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
