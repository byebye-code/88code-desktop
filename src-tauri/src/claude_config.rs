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

    // 读取现有配置JSON并提取未知字段
    let mut extra_env = HashMap::new();
    let mut extra_root = HashMap::new();
    let mut existing_permissions: Option<Value> = None;

    if settings_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&settings_path) {
            if let Ok(existing) = serde_json::from_str::<Value>(&content) {
                if let Some(obj) = existing.as_object() {
                    // 提取env中的未知字段
                    if let Some(env) = obj.get("env").and_then(|v| v.as_object()) {
                        for (key, value) in env {
                            if !matches!(key.as_str(),
                                "ANTHROPIC_AUTH_TOKEN" | "ANTHROPIC_BASE_URL" |
                                "CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC"
                            ) {
                                extra_env.insert(key.clone(), value.clone());
                            }
                        }
                    }

                    // 保存permissions
                    existing_permissions = obj.get("permissions").cloned();

                    // 提取根级别未知字段
                    for (key, value) in obj {
                        if !matches!(key.as_str(), "env" | "permissions") {
                            extra_root.insert(key.clone(), value.clone());
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

    // 添加额外env字段
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

    // 3. 其他根级别字段
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

    // 读取现有配置
    let mut final_config = if settings_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&settings_path) {
            serde_json::from_str::<Value>(&content).unwrap_or(new_config.clone())
        } else {
            new_config.clone()
        }
    } else {
        new_config.clone()
    };

    // 合并新配置到现有配置
    if let (Some(final_obj), Some(new_obj)) = (final_config.as_object_mut(), new_config.as_object()) {
        // 合并env字段
        if let Some(new_env) = new_obj.get("env").and_then(|v| v.as_object()) {
            let final_env = final_obj.entry("env")
                .or_insert_with(|| serde_json::json!({}))
                .as_object_mut()
                .unwrap();

            for (key, value) in new_env {
                final_env.insert(key.clone(), value.clone());
            }
        }

        // 更新permissions（如果有）
        if let Some(perms) = new_obj.get("permissions") {
            final_obj.insert("permissions".to_string(), perms.clone());
        }

        // 合并其他根级别字段
        for (key, value) in new_obj {
            if key != "env" && key != "permissions" {
                final_obj.insert(key.clone(), value.clone());
            }
        }
    }

    // 写入配置（格式化输出）
    let json_str = serde_json::to_string_pretty(&final_config)
        .map_err(|e| format!("序列化配置失败: {}", e))?;

    if let Some(parent) = settings_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("创建配置目录失败: {}", e))?;
    }

    std::fs::write(&settings_path, json_str)
        .map_err(|e| format!("写入配置文件失败: {}", e))?;

    log::info!("Claude Code 高级配置成功: {:?}", settings_path);
    Ok(())
}
