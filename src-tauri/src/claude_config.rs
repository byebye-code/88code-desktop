use crate::config::{get_claude_settings_path, read_json_file};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Claude Code settings.json 的结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeSettings {
    pub env: HashMap<String, String>,
    #[serde(default)]
    pub permissions: Permissions,
    /// 保留未知字段，防止版本更新时丢失新字段
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Permissions {
    #[serde(default)]
    pub allow: Vec<String>,
    #[serde(default)]
    pub deny: Vec<String>,
}

impl Default for ClaudeSettings {
    fn default() -> Self {
        Self {
            env: HashMap::new(),
            permissions: Permissions::default(),
            extra: HashMap::new(),
        }
    }
}

/// 配置 Claude Code
pub fn configure_claude_code(base_url: String, api_key: String) -> Result<(), String> {
    let settings_path = get_claude_settings_path();

    // 读取现有配置JSON并提取未知字段（使用Vec保持顺序）
    let mut extra_env = Vec::new();
    let mut extra_root = Vec::new();
    let mut existing_permissions: Option<Value> = None;

    if settings_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&settings_path) {
            if let Ok(existing) = serde_json::from_str::<Value>(&content) {
                if let Some(obj) = existing.as_object() {
                    // 提取env中的未知字段（保持顺序）
                    if let Some(env) = obj.get("env").and_then(|v| v.as_object()) {
                        for (key, value) in env {
                            if !matches!(key.as_str(),
                                "ANTHROPIC_AUTH_TOKEN" | "ANTHROPIC_BASE_URL" |
                                "CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC"
                            ) {
                                extra_env.push((key.clone(), value.clone()));
                            }
                        }
                    }

                    // 保存permissions
                    existing_permissions = obj.get("permissions").cloned();

                    // 提取根级别未知字段（保持顺序）
                    for (key, value) in obj {
                        if !matches!(key.as_str(), "env" | "permissions") {
                            extra_root.push((key.clone(), value.clone()));
                        }
                    }
                }
            }
        }
    }

    // 按固定顺序构建JSON字符串
    let mut json_str = String::from("{\n");

    // 1. env对象（固定顺序）
    json_str.push_str("  \"env\": {\n");
    json_str.push_str(&format!("    \"ANTHROPIC_AUTH_TOKEN\": \"{}\",\n",
        api_key.replace("\\", "\\\\").replace("\"", "\\\"")));
    json_str.push_str(&format!("    \"ANTHROPIC_BASE_URL\": \"{}\",\n",
        base_url.replace("\\", "\\\\").replace("\"", "\\\"")));
    json_str.push_str("    \"CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC\": \"1\"");

    // 添加额外env字段（Vec保持原顺序）
    for (key, value) in &extra_env {
        json_str.push_str(",\n");
        json_str.push_str(&format!("    \"{}\": {}", key,
            serde_json::to_string(value).unwrap_or_default()));
    }
    json_str.push_str("\n  },\n");

    // 2. permissions对象
    json_str.push_str("  \"permissions\": ");
    if let Some(perms) = existing_permissions {
        json_str.push_str(&serde_json::to_string_pretty(&perms).unwrap_or_default().replace("\n", "\n  "));
    } else {
        json_str.push_str("{\n    \"allow\": [],\n    \"deny\": []\n  }");
    }

    // 3. 其他根级别字段（Vec保持原顺序）
    for (key, value) in &extra_root {
        json_str.push_str(",\n");
        json_str.push_str(&format!("  \"{}\": {}", key,
            serde_json::to_string_pretty(value).unwrap_or_default().replace("\n", "\n  ")));
    }

    json_str.push_str("\n}\n");

    // 写入配置文件
    if let Some(parent) = settings_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("创建配置目录失败: {}", e))?;
    }

    std::fs::write(&settings_path, json_str)
        .map_err(|e| format!("写入配置文件失败: {}", e))?;

    log::info!("Claude Code 配置成功: {:?}", settings_path);
    Ok(())
}

/// 读取当前 Claude Code 配置
pub fn get_claude_config() -> Result<ClaudeSettings, String> {
    let settings_path = get_claude_settings_path();

    if !settings_path.exists() {
        return Ok(ClaudeSettings::default());
    }

    read_json_file(&settings_path)
}

/// 高级配置 Claude Code（直接写入用户提供的完整配置内容）
pub fn configure_claude_advanced(config_content: String) -> Result<(), String> {
    let settings_path = get_claude_settings_path();

    // 验证JSON格式
    let new_config: Value = serde_json::from_str(&config_content)
        .map_err(|e| format!("配置内容格式错误: {}", e))?;

    // 读取现有配置，提取字段（使用Vec保持顺序）
    let mut extra_env = Vec::new();
    let mut extra_root = Vec::new();
    let mut existing_permissions: Option<Value> = None;

    if settings_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&settings_path) {
            if let Ok(existing) = serde_json::from_str::<Value>(&content) {
                if let Some(obj) = existing.as_object() {
                    // 提取env中的所有字段（保持顺序）
                    if let Some(env) = obj.get("env").and_then(|v| v.as_object()) {
                        for (key, value) in env {
                            extra_env.push((key.clone(), value.clone()));
                        }
                    }

                    // 保存permissions
                    existing_permissions = obj.get("permissions").cloned();

                    // 提取根级别字段（保持顺序）
                    for (key, value) in obj {
                        if !matches!(key.as_str(), "env" | "permissions") {
                            extra_root.push((key.clone(), value.clone()));
                        }
                    }
                }
            }
        }
    }

    // 从新配置中提取要更新的字段
    let new_env = new_config.get("env").and_then(|v| v.as_object());
    let new_permissions = new_config.get("permissions");
    let new_obj = new_config.as_object();

    // 合并env字段（更新已存在的，添加新的）
    if let Some(new_env) = new_env {
        for (key, value) in new_env {
            // 更新或添加
            if let Some(pos) = extra_env.iter().position(|(k, _)| k == key) {
                extra_env[pos] = (key.clone(), value.clone());
            } else {
                extra_env.push((key.clone(), value.clone()));
            }
        }
    }

    // 更新permissions
    if let Some(perms) = new_permissions {
        existing_permissions = Some(perms.clone());
    }

    // 合并根级别字段
    if let Some(new_obj) = new_obj {
        for (key, value) in new_obj {
            if key != "env" && key != "permissions" {
                // 更新或添加
                if let Some(pos) = extra_root.iter().position(|(k, _)| k == key) {
                    extra_root[pos] = (key.clone(), value.clone());
                } else {
                    extra_root.push((key.clone(), value.clone()));
                }
            }
        }
    }

    // 按固定顺序构建JSON字符串
    let mut json_str = String::from("{\n");

    // 1. env对象（保持原顺序）
    json_str.push_str("  \"env\": {\n");
    for (i, (key, value)) in extra_env.iter().enumerate() {
        if i > 0 {
            json_str.push_str(",\n");
        }
        json_str.push_str(&format!("    \"{}\": {}", key,
            serde_json::to_string(value).unwrap_or_default()));
    }
    json_str.push_str("\n  }");

    // 2. permissions对象
    if let Some(perms) = existing_permissions {
        json_str.push_str(",\n  \"permissions\": ");
        json_str.push_str(&serde_json::to_string_pretty(&perms).unwrap_or_default().replace("\n", "\n  "));
    }

    // 3. 其他根级别字段（保持原顺序）
    for (key, value) in &extra_root {
        json_str.push_str(",\n");
        json_str.push_str(&format!("  \"{}\": {}", key,
            serde_json::to_string_pretty(value).unwrap_or_default().replace("\n", "\n  ")));
    }

    json_str.push_str("\n}\n");

    // 写入配置文件
    if let Some(parent) = settings_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("创建配置目录失败: {}", e))?;
    }

    std::fs::write(&settings_path, json_str)
        .map_err(|e| format!("写入配置文件失败: {}", e))?;

    log::info!("Claude Code 高级配置成功: {:?}", settings_path);
    Ok(())
}
