use crate::claude_config;
use crate::codex_config;
use crate::config;
use crate::env_manager;
use crate::vscode;

/// 配置 Claude Code
#[tauri::command]
pub async fn configure_claude_code(base_url: String, api_key: String) -> Result<String, String> {
    // 验证输入
    if api_key.trim().is_empty() {
        return Err("API 密钥不能为空".to_string());
    }

    if base_url.trim().is_empty() {
        return Err("Base URL 不能为空".to_string());
    }

    // 配置 Claude Code
    claude_config::configure_claude_code(base_url, api_key)?;

    Ok("Claude Code 配置成功！".to_string())
}

/// 配置 Codex 并设置环境变量
#[tauri::command]
pub async fn configure_codex(api_key: String) -> Result<String, String> {
    // 验证输入
    if api_key.trim().is_empty() {
        return Err("API 密钥不能为空".to_string());
    }

    // 配置 Codex
    codex_config::configure_codex(api_key.clone())?;

    // 设置环境变量 key88
    env_manager::set_key88_env(api_key)?;

    #[cfg(windows)]
    {
        Ok("Codex 配置成功！环境变量 key88 已设置，请重启 Codex 以使环境变量生效。".to_string())
    }

    #[cfg(not(windows))]
    {
        Ok("Codex 配置成功！环境变量 key88 已添加到 shell 配置文件，请重启终端或运行 'source ~/.zshrc' (或相应的配置文件) 以使环境变量生效。".to_string())
    }
}

/// 获取配置路径信息
#[tauri::command]
pub async fn get_config_paths() -> Result<config::ConfigPaths, String> {
    Ok(config::get_config_paths_info())
}

/// 读取当前 Claude Code 配置
#[tauri::command]
pub async fn get_current_claude_config() -> Result<Option<claude_config::ClaudeSettings>, String> {
    match claude_config::get_claude_config() {
        Ok(settings) => Ok(Some(settings)),
        Err(_) => Ok(None),
    }
}

/// 读取当前 Codex 配置
#[tauri::command]
pub async fn get_current_codex_auth() -> Result<Option<codex_config::CodexAuth>, String> {
    codex_config::get_codex_auth()
}

/// 配置 VSCode Claude 扩展
#[tauri::command]
pub async fn configure_vscode_claude(base_url: String, api_key: String) -> Result<String, String> {
    if api_key.trim().is_empty() {
        return Err("API 密钥不能为空".to_string());
    }

    if base_url.trim().is_empty() {
        return Err("Base URL 不能为空".to_string());
    }

    vscode::configure_vscode_claude(api_key, base_url)
}

/// 配置 VSCode Codex 扩展
#[tauri::command]
pub async fn configure_vscode_codex(api_key: String) -> Result<String, String> {
    if api_key.trim().is_empty() {
        return Err("API 密钥不能为空".to_string());
    }

    vscode::configure_vscode_codex(api_key)
}

/// 获取 VSCode 配置路径
#[tauri::command]
pub async fn get_vscode_paths() -> Result<Vec<String>, String> {
    Ok(vscode::get_vscode_paths_info())
}
