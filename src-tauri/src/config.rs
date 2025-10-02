use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// 获取 Claude Code 配置目录路径
pub fn get_claude_config_dir() -> PathBuf {
    dirs::home_dir()
        .expect("无法获取用户主目录")
        .join(".claude")
}

/// 获取 Claude Code settings.json 文件路径
pub fn get_claude_settings_path() -> PathBuf {
    get_claude_config_dir().join("settings.json")
}

/// 获取 Codex 配置目录路径
pub fn get_codex_config_dir() -> PathBuf {
    dirs::home_dir()
        .expect("无法获取用户主目录")
        .join(".codex")
}

/// 获取 Codex auth.json 路径
pub fn get_codex_auth_path() -> PathBuf {
    get_codex_config_dir().join("auth.json")
}

/// 获取 Codex config.toml 路径
pub fn get_codex_config_path() -> PathBuf {
    get_codex_config_dir().join("config.toml")
}

/// 原子写入：写入临时文件后 rename 替换，避免半写状态
pub fn atomic_write(path: &Path, data: &[u8]) -> Result<(), String> {
    // 确保父目录存在
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {}", e))?;
    }

    // 生成临时文件路径（带纳秒时间戳）
    let parent = path.parent().ok_or_else(|| "无效的路径".to_string())?;
    let mut tmp = parent.to_path_buf();
    let file_name = path
        .file_name()
        .ok_or_else(|| "无效的文件名".to_string())?
        .to_string_lossy()
        .to_string();
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    tmp.push(format!("{}.tmp.{}", file_name, ts));

    // 写入临时文件
    {
        let mut f = fs::File::create(&tmp)
            .map_err(|e| format!("创建临时文件失败: {}", e))?;
        f.write_all(data)
            .map_err(|e| format!("写入临时文件失败: {}", e))?;
        f.flush()
            .map_err(|e| format!("刷新临时文件失败: {}", e))?;
    }

    // Unix 系统：复制原文件权限
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(meta) = fs::metadata(path) {
            let perm = meta.permissions().mode();
            let _ = fs::set_permissions(&tmp, fs::Permissions::from_mode(perm));
        }
    }

    // Windows 系统：先删除目标文件（rename 不支持覆盖）
    #[cfg(windows)]
    {
        if path.exists() {
            let _ = fs::remove_file(path);
        }
        fs::rename(&tmp, path).map_err(|e| format!("原子替换失败: {}", e))?;
    }

    // Unix/macOS 系统：直接 rename（原子操作）
    #[cfg(not(windows))]
    {
        fs::rename(&tmp, path).map_err(|e| format!("原子替换失败: {}", e))?;
    }

    Ok(())
}

/// 写入 JSON 配置文件
pub fn write_json_file<T: Serialize>(path: &Path, data: &T) -> Result<(), String> {
    // 序列化为格式化 JSON
    let json = serde_json::to_string_pretty(data)
        .map_err(|e| format!("序列化 JSON 失败: {}", e))?;

    // 调用原子写入
    atomic_write(path, json.as_bytes())
}

/// 原子写入文本文件（用于 TOML/纯文本）
pub fn write_text_file(path: &Path, data: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("创建目录失败: {}", e))?;
    }
    atomic_write(path, data.as_bytes())
}

/// 读取 JSON 文件并解析
pub fn read_json_file<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, String> {
    if !path.exists() {
        return Err(format!("文件不存在: {}", path.display()));
    }

    let content = fs::read_to_string(path)
        .map_err(|e| format!("读取文件失败: {}", e))?;

    serde_json::from_str(&content)
        .map_err(|e| format!("解析 JSON 失败: {}", e))
}

/// 获取当前系统的配置路径信息（用于前端显示）
#[derive(Serialize)]
pub struct ConfigPaths {
    pub claude_dir: String,
    pub claude_settings: String,
    pub codex_dir: String,
    pub codex_auth: String,
    pub codex_config: String,
}

pub fn get_config_paths_info() -> ConfigPaths {
    ConfigPaths {
        claude_dir: get_claude_config_dir().to_string_lossy().to_string(),
        claude_settings: get_claude_settings_path().to_string_lossy().to_string(),
        codex_dir: get_codex_config_dir().to_string_lossy().to_string(),
        codex_auth: get_codex_auth_path().to_string_lossy().to_string(),
        codex_config: get_codex_config_path().to_string_lossy().to_string(),
    }
}
