use crate::claude_config;
use crate::codex_config;
use crate::config;
use crate::env_manager;
use crate::vscode;

/// 默认的 Base URL
const DEFAULT_CLAUDE_BASE_URL: &str = "https://www.88code.org/api";
const DEFAULT_CODEX_BASE_URL: &str = "https://88code.org/openai/v1";

/// 配置 Claude Code
#[tauri::command]
pub async fn configure_claude_code(base_url: String, api_key: String) -> Result<String, String> {
    // 验证输入
    if api_key.trim().is_empty() {
        return Err("API 密钥不能为空".to_string());
    }

    // 如果 base_url 为空，使用 Claude 默认值
    let base_url = if base_url.trim().is_empty() {
        DEFAULT_CLAUDE_BASE_URL.to_string()
    } else {
        base_url.trim().to_string()
    };

    // 配置 Claude Code
    claude_config::configure_claude_code(base_url, api_key)?;

    Ok("Claude Code 配置成功！".to_string())
}

/// 配置 Codex 并设置环境变量
#[tauri::command]
pub async fn configure_codex(base_url: String, api_key: String) -> Result<String, String> {
    // 验证输入
    if api_key.trim().is_empty() {
        return Err("API 密钥不能为空".to_string());
    }

    // 如果 base_url 为空，使用 Codex 默认值
    let base_url = if base_url.trim().is_empty() {
        DEFAULT_CODEX_BASE_URL.to_string()
    } else {
        base_url.trim().to_string()
    };

    // 配置 Codex
    codex_config::configure_codex(base_url, api_key.clone())?;

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
    // 如果 api_key 为空，使用默认值 "key"
    let api_key = if api_key.trim().is_empty() {
        "key".to_string()
    } else {
        api_key.trim().to_string()
    };

    // VSCode Claude 扩展只需要 API Key，base_url 不做检查
    vscode::configure_vscode_claude(api_key, base_url)
}

/// 配置 VSCode Codex 扩展
#[tauri::command]
pub async fn configure_vscode_codex(base_url: String, api_key: String) -> Result<String, String> {
    if api_key.trim().is_empty() {
        return Err("API 密钥不能为空".to_string());
    }

    // 如果 base_url 为空，使用 Codex 默认值
    let base_url = if base_url.trim().is_empty() {
        DEFAULT_CODEX_BASE_URL.to_string()
    } else {
        base_url.trim().to_string()
    };

    vscode::configure_vscode_codex(base_url, api_key)
}

/// 获取 VSCode 配置路径
#[tauri::command]
pub async fn get_vscode_paths() -> Result<Vec<String>, String> {
    Ok(vscode::get_vscode_paths_info())
}

/// 高级配置 Claude Code（允许用户自定义完整配置内容）
#[tauri::command]
pub async fn configure_claude_advanced(config_content: String) -> Result<String, String> {
    claude_config::configure_claude_advanced(config_content)?;
    Ok("Claude Code 高级配置成功！".to_string())
}

/// 高级配置 Codex（允许用户自定义完整配置内容）
#[tauri::command]
pub async fn configure_codex_advanced(
    auth_json: String,
    config_toml: String,
    api_key: String,
) -> Result<String, String> {
    codex_config::configure_codex_advanced(auth_json, config_toml, api_key.clone())?;

    // 设置环境变量 key88
    env_manager::set_key88_env(api_key)?;

    #[cfg(windows)]
    {
        Ok("Codex 高级配置成功！环境变量 key88 已设置，请重启 Codex 以使环境变量生效。".to_string())
    }

    #[cfg(not(windows))]
    {
        Ok("Codex 高级配置成功！环境变量 key88 已添加到 shell 配置文件，请重启终端或运行 'source ~/.zshrc' (或相应的配置文件) 以使环境变量生效。".to_string())
    }
}

/// 删除 Claude Code 配置文件
#[tauri::command]
pub async fn delete_claude_config() -> Result<String, String> {
    let settings_path = config::get_claude_settings_path();

    if settings_path.exists() {
        std::fs::remove_file(&settings_path)
            .map_err(|e| format!("删除配置文件失败: {}", e))?;
        Ok(format!("已删除配置文件: {:?}", settings_path))
    } else {
        Ok("配置文件不存在".to_string())
    }
}

/// 删除 Codex 配置文件
#[tauri::command]
pub async fn delete_codex_config() -> Result<String, String> {
    let auth_path = config::get_codex_auth_path();
    let config_path = config::get_codex_config_path();

    let mut deleted = Vec::new();
    let mut not_exist = Vec::new();

    if auth_path.exists() {
        std::fs::remove_file(&auth_path)
            .map_err(|e| format!("删除 auth.json 失败: {}", e))?;
        deleted.push("auth.json");
    } else {
        not_exist.push("auth.json");
    }

    if config_path.exists() {
        std::fs::remove_file(&config_path)
            .map_err(|e| format!("删除 config.toml 失败: {}", e))?;
        deleted.push("config.toml");
    } else {
        not_exist.push("config.toml");
    }

    let mut msg = String::new();
    if !deleted.is_empty() {
        msg.push_str(&format!("已删除: {}", deleted.join(", ")));
    }
    if !not_exist.is_empty() {
        if !msg.is_empty() {
            msg.push_str("；");
        }
        msg.push_str(&format!("不存在: {}", not_exist.join(", ")));
    }

    Ok(msg)
}
