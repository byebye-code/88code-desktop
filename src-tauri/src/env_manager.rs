use std::process::Command;

#[cfg(not(windows))]
use std::env;
#[cfg(not(windows))]
use std::fs::{self, OpenOptions};
#[cfg(not(windows))]
use std::io::{BufRead, BufReader, Write};
#[cfg(not(windows))]
use std::path::PathBuf;

/// 设置环境变量 key88 (跨平台)
pub fn set_key88_env(api_key: String) -> Result<(), String> {
    #[cfg(windows)]
    {
        set_windows_env("key88", &api_key)
    }

    #[cfg(not(windows))]
    {
        set_unix_env("key88", &api_key)
    }
}

/// Windows: 使用 setx 命令设置用户环境变量
#[cfg(windows)]
fn set_windows_env(key: &str, value: &str) -> Result<(), String> {
    let output = Command::new("setx")
        .arg(key)
        .arg(value)
        .output()
        .map_err(|e| format!("执行 setx 命令失败: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("设置环境变量失败: {}", stderr));
    }

    log::info!("Windows 环境变量设置成功: {}={}", key, value);
    Ok(())
}

/// Unix/macOS: 写入 shell 配置文件
#[cfg(not(windows))]
fn set_unix_env(key: &str, value: &str) -> Result<(), String> {
    let shell_config = detect_shell_config()?;
    append_env_to_shell_config(&shell_config, key, value)?;

    log::info!(
        "Unix 环境变量已添加到: {:?}, {}={}",
        shell_config,
        key,
        value
    );
    Ok(())
}

/// 检测当前使用的 shell 配置文件
#[cfg(not(windows))]
fn detect_shell_config() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or_else(|| "无法获取用户主目录".to_string())?;

    // 检查 SHELL 环境变量
    if let Ok(shell) = env::var("SHELL") {
        if shell.contains("zsh") {
            return Ok(home.join(".zshrc"));
        } else if shell.contains("bash") {
            // macOS 优先使用 .bash_profile
            #[cfg(target_os = "macos")]
            {
                let bash_profile = home.join(".bash_profile");
                if bash_profile.exists() {
                    return Ok(bash_profile);
                }
            }

            return Ok(home.join(".bashrc"));
        } else if shell.contains("fish") {
            return Ok(home.join(".config/fish/config.fish"));
        }
    }

    // 默认尝试 zsh (大多数现代系统)
    let zshrc = home.join(".zshrc");
    if zshrc.exists() {
        return Ok(zshrc);
    }

    // 回退到 bashrc
    Ok(home.join(".bashrc"))
}

/// 将环境变量添加到 shell 配置文件（避免重复）
#[cfg(not(windows))]
fn append_env_to_shell_config(config_path: &PathBuf, key: &str, value: &str) -> Result<(), String> {
    // 创建配置文件(如果不存在)
    if !config_path.exists() {
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("创建配置目录失败: {}", e))?;
        }
        fs::File::create(config_path).map_err(|e| format!("创建配置文件失败: {}", e))?;
    }

    // 读取现有内容，检查是否已存在该变量
    let file = fs::File::open(config_path).map_err(|e| format!("打开配置文件失败: {}", e))?;
    let reader = BufReader::new(file);

    let marker = format!("export {}=", key);
    let mut already_exists = false;

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.trim().starts_with(&marker) {
                already_exists = true;
                break;
            }
        }
    }

    // 如果已存在，先删除旧的，再添加新的
    if already_exists {
        remove_env_from_shell_config(config_path, key)?;
    }

    // 追加新的环境变量
    let mut file = OpenOptions::new()
        .append(true)
        .open(config_path)
        .map_err(|e| format!("打开配置文件失败: {}", e))?;

    let env_line = format!("\n# Added by 88code-desktop\nexport {}=\"{}\"\n", key, value);
    file.write_all(env_line.as_bytes())
        .map_err(|e| format!("写入配置文件失败: {}", e))?;

    Ok(())
}

/// 从 shell 配置文件中删除指定环境变量
#[cfg(not(windows))]
fn remove_env_from_shell_config(config_path: &PathBuf, key: &str) -> Result<(), String> {
    let content = fs::read_to_string(config_path)
        .map_err(|e| format!("读取配置文件失败: {}", e))?;

    let marker = format!("export {}=", key);
    let mut new_lines = Vec::new();
    let mut skip_next_blank = false;

    for line in content.lines() {
        if line.trim().starts_with(&marker) {
            skip_next_blank = true; // 跳过环境变量行
            continue;
        }

        if skip_next_blank && line.trim() == "" {
            skip_next_blank = false; // 跳过后面的空行
            continue;
        }

        if line.trim() == "# Added by 88code-desktop" {
            continue; // 跳过标记注释
        }

        new_lines.push(line);
    }

    fs::write(config_path, new_lines.join("\n"))
        .map_err(|e| format!("写入配置文件失败: {}", e))?;

    Ok(())
}
