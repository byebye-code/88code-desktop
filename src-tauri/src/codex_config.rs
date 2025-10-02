use crate::config::{get_codex_auth_path, get_codex_config_path, write_json_file, write_text_file};
use serde::{Deserialize, Serialize};

/// Codex auth.json 的结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodexAuth {
    #[serde(rename = "OPENAI_API_KEY")]
    pub openai_api_key: String,
}

/// 固定的 Codex config.toml 模板
const CODEX_CONFIG_TEMPLATE: &str = r#"model_provider = "88code"
model = "gpt-5-codex"
model_reasoning_effort = "high"
disable_response_storage = true

[model_providers.88code]
name = "88code"
base_url = "https://88code.org/openai/v1"
wire_api = "responses"
env_key = "key88"
requires_openai_auth = true
"#;

/// 配置 Codex
pub fn configure_codex(api_key: String) -> Result<(), String> {
    let auth_path = get_codex_auth_path();
    let config_path = get_codex_config_path();

    // 创建 auth.json
    let auth = CodexAuth {
        openai_api_key: api_key.clone(),
    };

    // 写入 auth.json
    write_json_file(&auth_path, &auth)?;

    // 写入 config.toml (使用固定模板)
    write_text_file(&config_path, CODEX_CONFIG_TEMPLATE)?;

    log::info!("Codex 配置成功");
    log::info!("  auth.json: {:?}", auth_path);
    log::info!("  config.toml: {:?}", config_path);

    Ok(())
}

/// 读取当前 Codex 配置
pub fn get_codex_auth() -> Result<Option<CodexAuth>, String> {
    let auth_path = get_codex_auth_path();

    if !auth_path.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(&auth_path)
        .map_err(|e| format!("读取 auth.json 失败: {}", e))?;

    let auth: CodexAuth = serde_json::from_str(&content)
        .map_err(|e| format!("解析 auth.json 失败: {}", e))?;

    Ok(Some(auth))
}
