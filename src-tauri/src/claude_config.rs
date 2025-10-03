use crate::config::{get_claude_settings_path, read_json_file, write_json_file};
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

    // 读取现有配置（如果存在），否则使用默认配置
    let mut settings: ClaudeSettings = if settings_path.exists() {
        read_json_file(&settings_path).unwrap_or_default()
    } else {
        ClaudeSettings::default()
    };

    // 更新环境变量
    settings
        .env
        .insert("ANTHROPIC_AUTH_TOKEN".to_string(), api_key);
    settings
        .env
        .insert("ANTHROPIC_BASE_URL".to_string(), base_url);
    settings.env.insert(
        "CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC".to_string(),
        "1".to_string(),
    );

    // 保存配置
    write_json_file(&settings_path, &settings)?;

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
