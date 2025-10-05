use crate::config::{get_codex_auth_path, get_codex_config_path, write_json_file};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// 辅助函数：格式化 TOML 值为字符串
fn format_toml_value(key: &str, value: &toml::Value) -> String {
    match value {
        toml::Value::String(s) => format!("{} = \"{}\"\n", key, s),
        toml::Value::Integer(i) => format!("{} = {}\n", key, i),
        toml::Value::Float(f) => format!("{} = {}\n", key, f),
        toml::Value::Boolean(b) => format!("{} = {}\n", key, b),
        toml::Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(|v| {
                match v {
                    toml::Value::String(s) => format!("\"{}\"", s),
                    toml::Value::Integer(i) => i.to_string(),
                    toml::Value::Float(f) => f.to_string(),
                    toml::Value::Boolean(b) => b.to_string(),
                    _ => String::new(),
                }
            }).collect();
            format!("{} = [{}]\n", key, items.join(", "))
        },
        _ => String::new(),
    }
}

/// Codex auth.json 的结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodexAuth {
    #[serde(rename = "OPENAI_API_KEY")]
    pub openai_api_key: String,
    /// 保留未知字段，防止版本更新时丢失新字段
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// 配置 Codex
pub fn configure_codex(base_url: String, api_key: String) -> Result<(), String> {
    let auth_path = get_codex_auth_path();
    let config_path = get_codex_config_path();

    // 读取现有 auth.json 的原始JSON（保留所有字段）
    let mut auth_value: Value = if auth_path.exists() {
        match std::fs::read_to_string(&auth_path) {
            Ok(content) => {
                serde_json::from_str(&content).unwrap_or_else(|e| {
                    log::warn!("解析现有 auth.json 失败: {}，将创建新配置", e);
                    serde_json::json!({})
                })
            }
            Err(e) => {
                log::warn!("读取现有 auth.json 失败: {}，将创建新配置", e);
                serde_json::json!({})
            }
        }
    } else {
        serde_json::json!({})
    };

    // 确保是对象
    if !auth_value.is_object() {
        auth_value = serde_json::json!({});
    }

    // 更新 OPENAI_API_KEY
    if let Some(obj) = auth_value.as_object_mut() {
        obj.insert("OPENAI_API_KEY".to_string(), Value::String(api_key.clone()));
    }

    // 写入 auth.json
    let auth_json_str = serde_json::to_string_pretty(&auth_value)
        .map_err(|e| format!("序列化 auth.json 失败: {}", e))?;

    if let Some(parent) = auth_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("创建 Codex 配置目录失败: {}", e))?;
    }

    std::fs::write(&auth_path, auth_json_str)
        .map_err(|e| format!("写入 auth.json 失败: {}", e))?;

    // 读取现有 config.toml（如果存在），提取未知字段
    let existing_toml_content = if config_path.exists() {
        std::fs::read_to_string(&config_path).unwrap_or_default()
    } else {
        String::new()
    };

    // 解析现有配置，提取未知字段
    let mut extra_fields = Vec::new();
    let mut extra_providers = Vec::new();
    let mut extra_88code_fields = Vec::new();

    if !existing_toml_content.is_empty() {
        if let Ok(existing) = toml::from_str::<toml::Value>(&existing_toml_content) {
            if let Some(table) = existing.as_table() {
                // 收集根级别的未知字段
                for (key, value) in table {
                    if !matches!(key.as_str(),
                        "model_provider" | "model" | "model_reasoning_effort" |
                        "disable_response_storage" | "model_providers"
                    ) {
                        extra_fields.push((key.clone(), value.clone()));
                    }
                }

                // 收集其他 provider 和 88code 的未知字段
                if let Some(providers) = table.get("model_providers").and_then(|v| v.as_table()) {
                    for (provider_name, provider_value) in providers {
                        if provider_name == "88code" {
                            // 收集 88code 的未知字段
                            if let Some(code88) = provider_value.as_table() {
                                for (key, value) in code88 {
                                    if !matches!(key.as_str(),
                                        "name" | "base_url" | "wire_api" | "env_key" | "requires_openai_auth"
                                    ) {
                                        extra_88code_fields.push((key.clone(), value.clone()));
                                    }
                                }
                            }
                        } else {
                            // 保留其他 provider
                            extra_providers.push((provider_name.clone(), provider_value.clone()));
                        }
                    }
                }
            }
        }
    }

    // 按照固定顺序生成配置内容
    let mut toml_str = format!(
        r#"model_provider = "88code"
model = "gpt-5-codex"
model_reasoning_effort = "high"
disable_response_storage = true
"#
    );

    // 添加额外的根级别字段
    for (key, value) in extra_fields {
        toml_str.push_str(&format_toml_value(&key, &value));
    }

    // 添加 model_providers.88code
    toml_str.push_str(&format!(
        r#"
[model_providers.88code]
name = "88code"
base_url = "{}"
wire_api = "responses"
env_key = "key88"
requires_openai_auth = true
"#,
        base_url
    ));

    // 添加 88code 的额外字段
    for (key, value) in extra_88code_fields {
        toml_str.push_str(&format_toml_value(&key, &value));
    }

    // 添加其他 provider
    for (provider_name, provider_value) in extra_providers {
        toml_str.push_str(&format!("\n[model_providers.{}]\n", provider_name));
        if let Some(table) = provider_value.as_table() {
            for (key, value) in table {
                toml_str.push_str(&format_toml_value(key, value));
            }
        }
    }

    // 写入 config.toml
    std::fs::write(&config_path, toml_str)
        .map_err(|e| format!("写入 config.toml 失败: {}", e))?;

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

/// 高级配置 Codex（直接写入用户提供的完整配置内容）
pub fn configure_codex_advanced(
    auth_json: String,
    config_toml: String,
    api_key: String,
) -> Result<(), String> {
    let auth_path = get_codex_auth_path();
    let config_path = get_codex_config_path();

    // 验证并解析 auth.json
    let new_auth: CodexAuth = serde_json::from_str(&auth_json)
        .map_err(|e| format!("auth.json 格式错误: {}", e))?;

    // 读取现有 auth.json（如果存在）
    let existing_auth = if auth_path.exists() {
        get_codex_auth()?.unwrap_or_else(|| CodexAuth {
            openai_api_key: api_key.clone(),
            extra: HashMap::new(),
        })
    } else {
        CodexAuth {
            openai_api_key: api_key.clone(),
            extra: HashMap::new(),
        }
    };

    // 合并 auth.json：更新 API key，保留 extra 字段
    let mut merged_auth = existing_auth;
    merged_auth.openai_api_key = new_auth.openai_api_key;
    for (key, value) in new_auth.extra {
        merged_auth.extra.insert(key, value);
    }

    // 写入 auth.json
    write_json_file(&auth_path, &merged_auth)?;

    // 解析新的config.toml
    let new_toml: toml::Value = toml::from_str(&config_toml)
        .map_err(|e| format!("config.toml 格式错误: {}", e))?;

    // 读取现有config.toml并提取未知字段
    let mut extra_fields = Vec::new();
    let mut extra_providers = Vec::new();
    let mut extra_88code_fields = Vec::new();

    // 提取标准字段的值（优先使用新配置，如果没有则用旧配置）
    let mut model_provider = "88code".to_string();
    let mut model = "gpt-5-codex".to_string();
    let mut model_reasoning_effort = "high".to_string();
    let mut disable_response_storage = true;
    let mut base_url = String::new();
    let mut wire_api = "responses".to_string();
    let mut env_key = "key88".to_string();
    let mut provider_name = "88code".to_string();
    let mut requires_openai_auth = true;

    // 从新配置中提取值
    if let Some(table) = new_toml.as_table() {
        if let Some(v) = table.get("model_provider").and_then(|v| v.as_str()) {
            model_provider = v.to_string();
        }
        if let Some(v) = table.get("model").and_then(|v| v.as_str()) {
            model = v.to_string();
        }
        if let Some(v) = table.get("model_reasoning_effort").and_then(|v| v.as_str()) {
            model_reasoning_effort = v.to_string();
        }
        if let Some(v) = table.get("disable_response_storage").and_then(|v| v.as_bool()) {
            disable_response_storage = v;
        }

        // 提取未知根字段
        for (key, value) in table {
            if !matches!(key.as_str(),
                "model_provider" | "model" | "model_reasoning_effort" |
                "disable_response_storage" | "model_providers"
            ) {
                extra_fields.push((key.clone(), value.clone()));
            }
        }

        // 提取provider配置
        if let Some(providers) = table.get("model_providers").and_then(|v| v.as_table()) {
            for (prov_name, prov_value) in providers {
                if prov_name == "88code" {
                    if let Some(prov_table) = prov_value.as_table() {
                        if let Some(v) = prov_table.get("name").and_then(|v| v.as_str()) {
                            provider_name = v.to_string();
                        }
                        if let Some(v) = prov_table.get("base_url").and_then(|v| v.as_str()) {
                            base_url = v.to_string();
                        }
                        if let Some(v) = prov_table.get("wire_api").and_then(|v| v.as_str()) {
                            wire_api = v.to_string();
                        }
                        if let Some(v) = prov_table.get("env_key").and_then(|v| v.as_str()) {
                            env_key = v.to_string();
                        }
                        if let Some(v) = prov_table.get("requires_openai_auth").and_then(|v| v.as_bool()) {
                            requires_openai_auth = v;
                        }

                        // 提取88code未知字段
                        for (key, value) in prov_table {
                            if !matches!(key.as_str(),
                                "name" | "base_url" | "wire_api" | "env_key" | "requires_openai_auth"
                            ) {
                                extra_88code_fields.push((key.clone(), value.clone()));
                            }
                        }
                    }
                } else {
                    extra_providers.push((prov_name.clone(), prov_value.clone()));
                }
            }
        }
    }

    // 读取现有配置，提取额外字段
    if config_path.exists() {
        if let Ok(existing_content) = std::fs::read_to_string(&config_path) {
            if let Ok(existing) = toml::from_str::<toml::Value>(&existing_content) {
                if let Some(table) = existing.as_table() {
                    // 从现有配置补充缺失的未知字段
                    for (key, value) in table {
                        if !matches!(key.as_str(),
                            "model_provider" | "model" | "model_reasoning_effort" |
                            "disable_response_storage" | "model_providers"
                        ) {
                            if !extra_fields.iter().any(|(k, _)| k == key) {
                                extra_fields.push((key.clone(), value.clone()));
                            }
                        }
                    }

                    // 从现有配置补充providers相关字段
                    if let Some(providers) = table.get("model_providers").and_then(|v| v.as_table()) {
                        // 补充88code的未知字段
                        if let Some(code88) = providers.get("88code").and_then(|v| v.as_table()) {
                            for (key, value) in code88 {
                                if !matches!(key.as_str(),
                                    "name" | "base_url" | "wire_api" | "env_key" | "requires_openai_auth"
                                ) {
                                    if !extra_88code_fields.iter().any(|(k, _)| k == key) {
                                        extra_88code_fields.push((key.clone(), value.clone()));
                                    }
                                }
                            }
                        }

                        // 补充其他provider
                        for (prov_name, prov_value) in providers {
                            if prov_name != "88code" && !extra_providers.iter().any(|(n, _)| n == prov_name) {
                                extra_providers.push((prov_name.clone(), prov_value.clone()));
                            }
                        }
                    }
                }
            }
        }
    }

    // 按固定顺序构建TOML字符串
    let mut toml_str = format!(
        r#"model_provider = "{}"
model = "{}"
model_reasoning_effort = "{}"
disable_response_storage = {}
"#,
        model_provider, model, model_reasoning_effort, disable_response_storage
    );

    // 添加额外根字段
    for (key, value) in extra_fields {
        toml_str.push_str(&format_toml_value(&key, &value));
    }

    // 添加88code provider
    toml_str.push_str(&format!(
        r#"
[model_providers.88code]
name = "{}"
base_url = "{}"
wire_api = "{}"
env_key = "{}"
requires_openai_auth = {}
"#,
        provider_name, base_url, wire_api, env_key, requires_openai_auth
    ));

    // 添加88code额外字段
    for (key, value) in extra_88code_fields {
        toml_str.push_str(&format_toml_value(&key, &value));
    }

    // 添加其他provider
    for (prov_name, prov_value) in extra_providers {
        toml_str.push_str(&format!("\n[model_providers.{}]\n", prov_name));
        if let Some(table) = prov_value.as_table() {
            for (key, value) in table {
                toml_str.push_str(&format_toml_value(key, value));
            }
        }
    }

    // 写入config.toml
    std::fs::write(&config_path, toml_str)
        .map_err(|e| format!("写入 config.toml 失败: {}", e))?;

    log::info!("Codex 高级配置成功");
    log::info!("  auth.json: {:?}", auth_path);
    log::info!("  config.toml: {:?}", config_path);

    Ok(())
}
