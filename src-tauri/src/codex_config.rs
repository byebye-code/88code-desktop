use crate::config::{get_codex_auth_path, get_codex_config_path, write_json_file, write_text_file};
use serde::{Deserialize, Serialize};

/// Codex auth.json 的结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodexAuth {
    #[serde(rename = "OPENAI_API_KEY")]
    pub openai_api_key: String,
}

/// 生成 Codex config.toml 内容（使用用户提供的 base_url）
fn generate_config_toml(base_url: &str) -> String {
    format!(
        r#"model_provider = "88code"
model = "gpt-5-codex"
model_reasoning_effort = "high"
disable_response_storage = true

[model_providers.88code]
name = "88code"
base_url = "{}"
wire_api = "responses"
env_key = "key88"
requires_openai_auth = true
"#,
        base_url
    )
}

/// 配置 Codex
pub fn configure_codex(base_url: String, api_key: String) -> Result<(), String> {
    let auth_path = get_codex_auth_path();
    let config_path = get_codex_config_path();

    // 创建 auth.json
    let auth = CodexAuth {
        openai_api_key: api_key.clone(),
    };

    // 写入 auth.json
    write_json_file(&auth_path, &auth)?;

    // 生成并写入 config.toml (使用用户提供的 base_url)
    let config_content = generate_config_toml(&base_url);
    write_text_file(&config_path, &config_content)?;

    log::info!("Codex 配置成功");
    log::info!("  base_url: {}", base_url);
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
