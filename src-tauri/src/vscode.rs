use std::path::PathBuf;
use serde_json::{json, Value};
use std::fs;
use crate::config::{get_claude_config_dir, get_claude_settings_path, write_json_file};

/// 枚举可能的 VS Code 发行版配置目录名称
fn vscode_product_dirs() -> Vec<&'static str> {
    vec![
        "Code",            // VS Code Stable
        "Code - Insiders", // VS Code Insiders
        "VSCodium",        // VSCodium
        "Code - OSS",      // OSS 发行版
    ]
}

/// 获取 VS Code 用户 settings.json 的候选路径列表（按优先级排序）
pub fn candidate_settings_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    #[cfg(target_os = "macos")]
    {
        if let Some(home) = dirs::home_dir() {
            for prod in vscode_product_dirs() {
                paths.push(
                    home.join("Library")
                        .join("Application Support")
                        .join(prod)
                        .join("User")
                        .join("settings.json"),
                );
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        // Windows: %APPDATA%\Code\User\settings.json
        if let Some(roaming) = dirs::config_dir() {
            for prod in vscode_product_dirs() {
                paths.push(roaming.join(prod).join("User").join("settings.json"));
            }
        }
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        // Linux: ~/.config/Code/User/settings.json
        if let Some(config) = dirs::config_dir() {
            for prod in vscode_product_dirs() {
                paths.push(config.join(prod).join("User").join("settings.json"));
            }
        }
    }

    paths
}

/// 返回第一个存在的 settings.json 路径
pub fn find_existing_settings() -> Option<PathBuf> {
    for p in candidate_settings_paths() {
        if let Ok(meta) = std::fs::metadata(&p) {
            if meta.is_file() {
                return Some(p);
            }
        }
    }
    None
}

/// 配置 VSCode Claude 扩展
/// 功能：在 ~/.claude/config.json 中写入 {"primaryApiKey": "key"}
pub fn configure_vscode_claude(api_key: String, _base_url: String) -> Result<String, String> {
    // 1. 检查客户端配置是否存在
    let settings_path = get_claude_settings_path();
    if !settings_path.exists() {
        return Err("请先配置 Claude Code 客户端！需要先完成客户端配置才能配置 VSCode。".to_string());
    }

    // 2. 获取 ~/.claude/config.json 路径
    let config_dir = get_claude_config_dir();
    let config_path = config_dir.join("config.json");

    // 3. 创建配置内容
    let config_content = json!({
        "primaryApiKey": api_key
    });

    // 4. 写入配置文件
    write_json_file(&config_path, &config_content)?;

    Ok(format!(
        "VSCode 配置成功！已写入: {}\n请重新加载 VSCode 窗口以使配置生效。",
        config_path.display()
    ))
}

/// 配置 VSCode Codex 扩展（配置 ChatGPT 扩展）
/// 功能：在 VSCode settings.json 中写入 ChatGPT 扩展配置
pub fn configure_vscode_codex(api_key: String) -> Result<String, String> {
    let settings_path = find_existing_settings()
        .ok_or_else(|| "未找到 VSCode settings.json 文件。请确保已安装 VSCode 并至少打开过一次。".to_string())?;

    // 读取现有设置（支持 JSONC 格式的简单处理）
    let content = fs::read_to_string(&settings_path)
        .map_err(|e| format!("读取 VSCode 设置失败: {}", e))?;

    let mut settings: Value = serde_json::from_str(&content).unwrap_or(json!({}));

    // 更新 ChatGPT 扩展配置
    if let Some(obj) = settings.as_object_mut() {
        // 设置 API Base URL（88code 的 Codex 服务）
        obj.insert(
            "chatgpt.apiBase".to_string(),
            Value::String("https://88code.org/openai/v1".to_string()),
        );

        // 设置认证方式为 apikey
        let mut config_obj = serde_json::Map::new();
        config_obj.insert(
            "preferred_auth_method".to_string(),
            Value::String("apikey".to_string()),
        );
        obj.insert("chatgpt.config".to_string(), Value::Object(config_obj));

        // 注意：API Key 通过环境变量 key88 传递，不直接写入 settings.json
        log::info!("已配置 ChatGPT 扩展使用 88code 服务，请确保环境变量 key88={}", api_key);
    }

    // 写入配置
    let json_str = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("序列化设置失败: {}", e))?;

    crate::config::atomic_write(&settings_path, json_str.as_bytes())?;

    Ok(format!(
        "VSCode 配置成功！路径: {}\n已配置 ChatGPT 扩展使用 88code 服务。\n请重新加载 VSCode 窗口以使配置生效。",
        settings_path.display()
    ))
}

/// 获取 VSCode 配置路径信息
pub fn get_vscode_paths_info() -> Vec<String> {
    candidate_settings_paths()
        .into_iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect()
}
