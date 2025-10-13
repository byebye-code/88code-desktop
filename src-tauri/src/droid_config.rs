use crate::config::get_droid_config_path;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

const API_KEY_PLACEHOLDER: &str = "请替换为你的 API Key";
const DEFAULT_DROID_API_BASE_URL: &str = "https://www.88code.org/droid";
const DEFAULT_DROID_API_V1_BASE_URL: &str = "https://www.88code.org/droid/v1";

/// Droid 自定义模型配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomModel {
    pub model_display_name: String,
    pub model: String,
    pub base_url: String,
    pub api_key: String,
    pub provider: String,
    /// 保留未知字段
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl CustomModel {
    fn new(
        model_display_name: &str,
        model: &str,
        base_url: &str,
        api_key: &str,
        provider: &str,
    ) -> Self {
        Self {
            model_display_name: model_display_name.to_string(),
            model: model.to_string(),
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
            provider: provider.to_string(),
            extra: HashMap::new(),
        }
    }
}

/// Droid config.json 的结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DroidConfig {
    pub custom_models: Vec<CustomModel>,
    /// 保留未知字段
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl Default for DroidConfig {
    fn default() -> Self {
        Self {
            custom_models: vec![
                CustomModel::new(
                    "Sonnet 4.5 [88code]",
                    "claude-sonnet-4-5-20250929",
                    DEFAULT_DROID_API_BASE_URL,
                    API_KEY_PLACEHOLDER,
                    "anthropic",
                ),
                CustomModel::new(
                    "GPT-5-Codex [88code]",
                    "gpt-5-codex",
                    DEFAULT_DROID_API_V1_BASE_URL,
                    API_KEY_PLACEHOLDER,
                    "openai",
                ),
                CustomModel::new(
                    "GPT-5 [88code]",
                    "gpt-5",
                    DEFAULT_DROID_API_V1_BASE_URL,
                    API_KEY_PLACEHOLDER,
                    "openai",
                ),
            ],
            extra: HashMap::new(),
        }
    }
}

/// 配置 Droid（添加或更新自定义模型）
pub fn configure_droid(
    model_display_name: String,
    model: String,
    base_url: String,
    api_key: String,
    provider: String,
) -> Result<(), String> {
    let config_path = get_droid_config_path();

    // 当配置文件不存在时，先写入默认模型集合，方便用户直接替换 API Key
    if !config_path.exists() {
        let default_config = DroidConfig::default();
        crate::config::write_json_file(&config_path, &default_config)
            .map_err(|e| format!("写入配置文件失败: {}", e))?;
    }

    // 首次配置前创建备份
    crate::config::create_backup_if_not_exists(&config_path)?;

    // 读取现有配置，提取所有字段（使用Vec保持顺序）
    let mut custom_models = Vec::new();
    let mut extra_root = Vec::new();

    if config_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&config_path) {
            if let Ok(existing) = serde_json::from_str::<Value>(&content) {
                if let Some(obj) = existing.as_object() {
                    // 提取 custom_models
                    if let Some(models) = obj.get("custom_models").and_then(|v| v.as_array()) {
                        for model_value in models {
                            custom_models.push(model_value.clone());
                        }
                    }

                    // 提取根级别未知字段（保持顺序）
                    for (key, value) in obj {
                        if key != "custom_models" {
                            extra_root.push((key.clone(), value.clone()));
                        }
                    }
                }
            }
        }
    }

    // 重构现有模型为 Map（保持顺序）
    let mut models_vec: Vec<Map<String, Value>> = Vec::new();

    for model_value in &custom_models {
        if let Some(obj) = model_value.as_object() {
            let mut new_map = Map::new();

            // 按固定顺序添加已知字段
            if let Some(v) = obj.get("model_display_name") {
                new_map.insert("model_display_name".to_string(), v.clone());
            }
            if let Some(v) = obj.get("model") {
                new_map.insert("model".to_string(), v.clone());
            }
            if let Some(v) = obj.get("base_url") {
                new_map.insert("base_url".to_string(), v.clone());
            }
            if let Some(v) = obj.get("api_key") {
                new_map.insert("api_key".to_string(), v.clone());
            }
            if let Some(v) = obj.get("provider") {
                new_map.insert("provider".to_string(), v.clone());
            }

            // 添加其他未知字段
            for (key, value) in obj {
                if !matches!(key.as_str(), "model_display_name" | "model" | "base_url" | "api_key" | "provider") {
                    new_map.insert(key.clone(), value.clone());
                }
            }

            models_vec.push(new_map);
        }
    }

    // 检查是否已存在同名模型，如果存在则更新，否则添加
    let mut found = false;
    for model_map in &mut models_vec {
        if let Some(existing_name) = model_map.get("model_display_name").and_then(|v| v.as_str()) {
            if existing_name == model_display_name {
                // 更新现有模型，保持字段顺序
                model_map.insert("model_display_name".to_string(), Value::String(model_display_name.clone()));
                model_map.insert("model".to_string(), Value::String(model.clone()));
                model_map.insert("base_url".to_string(), Value::String(base_url.clone()));
                model_map.insert("api_key".to_string(), Value::String(api_key.clone()));
                model_map.insert("provider".to_string(), Value::String(provider.clone()));
                found = true;
                break;
            }
        }
    }

    // 如果不存在则添加新模型（按固定顺序）
    if !found {
        let mut new_model = Map::new();
        new_model.insert("model_display_name".to_string(), Value::String(model_display_name));
        new_model.insert("model".to_string(), Value::String(model));
        new_model.insert("base_url".to_string(), Value::String(base_url));
        new_model.insert("api_key".to_string(), Value::String(api_key));
        new_model.insert("provider".to_string(), Value::String(provider));
        models_vec.push(new_model);
    }

    // 构建最终的配置对象
    let mut root = Map::new();
    root.insert("custom_models".to_string(), Value::Array(
        models_vec.into_iter().map(Value::Object).collect()
    ));

    // 添加其他根级别字段（保持顺序）
    for (key, value) in extra_root {
        root.insert(key, value);
    }

    // 序列化为格式化的 JSON
    let json_str = serde_json::to_string_pretty(&Value::Object(root))
        .map_err(|e| format!("序列化 JSON 失败: {}", e))?;

    // 写入配置文件
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("创建配置目录失败: {}", e))?;
    }

    std::fs::write(&config_path, json_str)
        .map_err(|e| format!("写入配置文件失败: {}", e))?;

    log::info!("Droid 配置成功: {:?}", config_path);
    Ok(())
}

/// 读取当前 Droid 配置
pub fn get_droid_config() -> Result<DroidConfig, String> {
    let config_path = get_droid_config_path();

    if !config_path.exists() {
        let default_config = DroidConfig::default();
        crate::config::write_json_file(&config_path, &default_config)?;
        return Ok(default_config);
    }

    crate::config::read_json_file(&config_path)
}

/// 删除指定的自定义模型
pub fn delete_droid_model(model_display_name: String) -> Result<(), String> {
    let config_path = get_droid_config_path();

    if !config_path.exists() {
        return Err("配置文件不存在".to_string());
    }

    // 首次修改前创建备份
    crate::config::create_backup_if_not_exists(&config_path)?;

    // 读取现有配置
    let mut custom_models = Vec::new();
    let mut extra_root = Vec::new();

    if let Ok(content) = std::fs::read_to_string(&config_path) {
        if let Ok(existing) = serde_json::from_str::<Value>(&content) {
            if let Some(obj) = existing.as_object() {
                // 提取 custom_models，排除要删除的模型
                if let Some(models) = obj.get("custom_models").and_then(|v| v.as_array()) {
                    for model_value in models {
                        if let Some(obj) = model_value.as_object() {
                            if let Some(name) = obj.get("model_display_name").and_then(|v| v.as_str()) {
                                if name != model_display_name {
                                    custom_models.push(model_value.clone());
                                }
                            }
                        }
                    }
                }

                // 提取根级别未知字段
                for (key, value) in obj {
                    if key != "custom_models" {
                        extra_root.push((key.clone(), value.clone()));
                    }
                }
            }
        }
    }

    // 重构现有模型为 Map（保持顺序）
    let mut models_vec: Vec<Map<String, Value>> = Vec::new();

    for model_value in &custom_models {
        if let Some(obj) = model_value.as_object() {
            let mut new_map = Map::new();

            // 按固定顺序添加已知字段
            if let Some(v) = obj.get("model_display_name") {
                new_map.insert("model_display_name".to_string(), v.clone());
            }
            if let Some(v) = obj.get("model") {
                new_map.insert("model".to_string(), v.clone());
            }
            if let Some(v) = obj.get("base_url") {
                new_map.insert("base_url".to_string(), v.clone());
            }
            if let Some(v) = obj.get("api_key") {
                new_map.insert("api_key".to_string(), v.clone());
            }
            if let Some(v) = obj.get("provider") {
                new_map.insert("provider".to_string(), v.clone());
            }

            // 添加其他未知字段
            for (key, value) in obj {
                if !matches!(key.as_str(), "model_display_name" | "model" | "base_url" | "api_key" | "provider") {
                    new_map.insert(key.clone(), value.clone());
                }
            }

            models_vec.push(new_map);
        }
    }

    // 构建最终的配置对象
    let mut root = Map::new();
    root.insert("custom_models".to_string(), Value::Array(
        models_vec.into_iter().map(Value::Object).collect()
    ));

    // 添加其他根级别字段（保持顺序）
    for (key, value) in extra_root {
        root.insert(key, value);
    }

    // 序列化为格式化的 JSON
    let json_str = serde_json::to_string_pretty(&Value::Object(root))
        .map_err(|e| format!("序列化 JSON 失败: {}", e))?;

    std::fs::write(&config_path, json_str)
        .map_err(|e| format!("写入配置文件失败: {}", e))?;

    log::info!("Droid 模型已删除: {}", model_display_name);
    Ok(())
}
