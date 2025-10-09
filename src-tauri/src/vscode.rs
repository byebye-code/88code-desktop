use std::path::PathBuf;
use serde_json::{json, Value};
use std::fs;
use crate::config::{get_claude_config_dir, get_claude_settings_path, write_json_file};

/// 移除 JSON/JSONC 中的注释（简单实现）
/// 处理单行注释 // 和多行注释 /* */
fn strip_json_comments(content: &str) -> String {
    let mut result = String::new();
    let mut chars = content.chars().peekable();
    let mut in_string = false;
    let mut escape_next = false;

    while let Some(ch) = chars.next() {
        if escape_next {
            result.push(ch);
            escape_next = false;
            continue;
        }

        if ch == '\\' && in_string {
            result.push(ch);
            escape_next = true;
            continue;
        }

        if ch == '"' {
            in_string = !in_string;
            result.push(ch);
            continue;
        }

        if in_string {
            result.push(ch);
            continue;
        }

        // 处理注释（仅在非字符串中）
        if ch == '/' {
            if let Some(&next_ch) = chars.peek() {
                if next_ch == '/' {
                    // 单行注释，跳到行尾
                    chars.next(); // 消费第二个 /
                    for c in chars.by_ref() {
                        if c == '\n' {
                            result.push('\n');
                            break;
                        }
                    }
                    continue;
                } else if next_ch == '*' {
                    // 多行注释，跳到 */
                    chars.next(); // 消费 *
                    let mut found_end = false;
                    while let Some(c) = chars.next() {
                        if c == '*' {
                            if let Some(&n) = chars.peek() {
                                if n == '/' {
                                    chars.next(); // 消费 /
                                    found_end = true;
                                    break;
                                }
                            }
                        }
                    }
                    if found_end {
                        result.push(' '); // 用空格替代注释
                    }
                    continue;
                }
            }
        }

        result.push(ch);
    }

    result
}

/// 修复 JSON 中的尾部逗号（对象和数组最后一个元素后的逗号）
fn fix_json_trailing_commas(content: &str) -> String {
    let mut result = String::new();
    let mut chars = content.chars().peekable();
    let mut in_string = false;
    let mut escape_next = false;

    while let Some(ch) = chars.next() {
        if escape_next {
            result.push(ch);
            escape_next = false;
            continue;
        }

        if ch == '\\' && in_string {
            result.push(ch);
            escape_next = true;
            continue;
        }

        if ch == '"' {
            in_string = !in_string;
            result.push(ch);
            continue;
        }

        if in_string {
            result.push(ch);
            continue;
        }

        // 检查是否是尾部逗号（逗号后面只有空白字符，然后是 } 或 ]）
        if ch == ',' {
            result.push(ch);

            // 向前看，检查后面是否只有空白和 } 或 ]
            let mut temp_chars = chars.clone();
            let mut found_closing = false;
            let mut is_trailing = false;

            while let Some(&next) = temp_chars.peek() {
                if next.is_whitespace() {
                    temp_chars.next();
                    continue;
                }
                if next == '}' || next == ']' {
                    found_closing = true;
                    is_trailing = true;
                }
                break;
            }

            // 如果是尾部逗号，从result中移除最后一个字符（逗号）
            if is_trailing && found_closing {
                result.pop();
            }
        } else {
            result.push(ch);
        }
    }

    result
}

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
        } else {
            // 备用方案：尝试从环境变量直接读取
            if let Ok(appdata) = std::env::var("APPDATA") {
                let appdata_path = PathBuf::from(appdata);
                for prod in vscode_product_dirs() {
                    paths.push(appdata_path.join(prod).join("User").join("settings.json"));
                }
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

    // 首次配置前创建备份
    crate::config::create_backup_if_not_exists(&config_path)?;

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
pub fn configure_vscode_codex(base_url: String, api_key: String) -> Result<String, String> {
    // 查找或创建 settings.json 路径
    let settings_path = if let Some(path) = find_existing_settings() {
        // 首次配置前创建备份
        crate::config::create_backup_if_not_exists(&path)?;
        path
    } else {
        // 如果找不到现有配置，使用第一个候选路径（通常是 Code Stable）
        let candidates = candidate_settings_paths();
        if candidates.is_empty() {
            return Err(
                "无法确定 VSCode 配置目录路径。\n\
                 请检查：\n\
                 1. 是否已安装 VSCode\n\
                 2. Windows 系统环境变量 %APPDATA% 是否设置正确\n\
                 3. 可以尝试手动创建配置文件：%APPDATA%\\Code\\User\\settings.json"
                    .to_string(),
            );
        }
        let path = candidates[0].clone();

        // 确保父目录存在
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("创建 VSCode 配置目录失败: {}", e))?;
        }

        log::info!("VSCode settings.json 不存在，将创建新文件: {:?}", path);
        path
    };

    // 读取现有设置内容
    let original_content = if settings_path.exists() {
        fs::read_to_string(&settings_path)
            .map_err(|e| format!("读取 VSCode 设置失败: {}", e))?
    } else {
        String::from("{\n}")
    };

    // 验证 JSON 格式是否正确（移除注释和尾部逗号后验证）
    let cleaned_content = strip_json_comments(&original_content);
    let fixed_content = fix_json_trailing_commas(&cleaned_content);

    // 尝试解析以验证格式（仅用于验证，不用于重构）
    let _validation: Value = serde_json::from_str(&fixed_content).map_err(|e| {
        format!(
            "无法解析 VSCode settings.json 文件。\n\
             原因: {}\n\
             文件路径: {:?}\n\n\
             请检查配置文件格式是否正确。",
            e,
            settings_path
        )
    })?;

    // 在原文本基础上修改，完全保持原顺序
    let mut final_content = original_content.clone();

    // 检查是否已存在配置
    let has_api_base = final_content.contains(r#""chatgpt.apiBase""#);
    let has_config = final_content.contains(r#""chatgpt.config""#);

    if has_api_base {
        // 更新 apiBase 值（简单字符串替换，查找整个键值对）
        // 匹配模式："chatgpt.apiBase": "任意内容"
        let lines: Vec<&str> = final_content.lines().collect();
        let mut new_lines = Vec::new();

        for line in lines {
            if line.contains(r#""chatgpt.apiBase""#) && line.contains(':') {
                // 提取缩进
                let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
                // 检查是否有尾部逗号
                let has_comma = line.trim_end().ends_with(',');
                let comma = if has_comma { "," } else { "" };
                new_lines.push(format!(r#"{}"chatgpt.apiBase": "{}"{}"#, indent, base_url, comma));
            } else {
                new_lines.push(line.to_string());
            }
        }
        final_content = new_lines.join("\n");
    }

    if has_config {
        // 更新 config 对象（需要处理多行）
        let lines: Vec<&str> = final_content.lines().collect();
        let mut new_lines = Vec::new();
        let mut in_chatgpt_config = false;
        let mut config_indent = String::new();
        let mut brace_count = 0;
        let mut has_comma_after = false;

        for (i, line) in lines.iter().enumerate() {
            if line.contains(r#""chatgpt.config""#) && line.contains(':') {
                in_chatgpt_config = true;
                config_indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
                brace_count = 0;

                // 检查是否有逗号在闭合括号后
                let remaining_lines = &lines[i..];
                for future_line in remaining_lines {
                    if future_line.trim().starts_with('}') {
                        has_comma_after = future_line.trim_end().ends_with(',');
                        break;
                    }
                }

                // 写入新的配置
                new_lines.push(format!(r#"{}"chatgpt.config": {{"#, config_indent));
                new_lines.push(format!(r#"{}  "preferred_auth_method": "apikey""#, config_indent));
                let comma = if has_comma_after { "," } else { "" };
                new_lines.push(format!(r#"{}}}{}"#, config_indent, comma));
                continue;
            }

            if in_chatgpt_config {
                // 统计大括号
                for ch in line.chars() {
                    if ch == '{' {
                        brace_count += 1;
                    } else if ch == '}' {
                        brace_count -= 1;
                        if brace_count < 0 {
                            in_chatgpt_config = false;
                            break;
                        }
                    }
                }
                // 跳过原配置行
                continue;
            }

            new_lines.push(line.to_string());
        }
        final_content = new_lines.join("\n");
    }

    // 如果都不存在，在末尾添加
    if !has_api_base && !has_config {
        // 查找最后一个 }
        if let Some(last_brace_pos) = final_content.rfind('}') {
            // 检查倒数第二行的内容，判断是否需要逗号
            let before_brace = &final_content[..last_brace_pos];
            let lines: Vec<&str> = before_brace.lines().collect();

            // 获取缩进（从倒数第二行）
            let indent = if let Some(last_line) = lines.last() {
                last_line.chars().take_while(|c| c.is_whitespace()).collect::<String>()
            } else {
                String::from("  ")
            };

            // 检查是否需要逗号
            let needs_comma = if let Some(last_line) = lines.last() {
                let trimmed = last_line.trim();
                !trimmed.is_empty() && !trimmed.ends_with(',') && trimmed != "{"
            } else {
                false
            };

            let comma_prefix = if needs_comma { "," } else { "" };

            let insertion = format!(
                "{}\n{}\"{}\": \"{}\",\n{}\"chatgpt.config\": {{\n{}  \"preferred_auth_method\": \"apikey\"\n{}}}\n",
                comma_prefix,
                indent,
                "chatgpt.apiBase",
                base_url,
                indent,
                indent,
                indent
            );

            final_content.insert_str(last_brace_pos, &insertion);
        }
    } else if has_api_base && !has_config {
        // 只有 apiBase，添加 config
        if let Some(last_brace_pos) = final_content.rfind('}') {
            let before_brace = &final_content[..last_brace_pos];
            let lines: Vec<&str> = before_brace.lines().collect();
            let indent = if let Some(last_line) = lines.last() {
                last_line.chars().take_while(|c| c.is_whitespace()).collect::<String>()
            } else {
                String::from("  ")
            };
            let needs_comma = if let Some(last_line) = lines.last() {
                !last_line.trim_end().ends_with(',')
            } else {
                false
            };

            let insertion = format!(
                "{}\n{}\"chatgpt.config\": {{\n{}  \"preferred_auth_method\": \"apikey\"\n{}}}\n",
                if needs_comma { "," } else { "" },
                indent,
                indent,
                indent
            );
            final_content.insert_str(last_brace_pos, &insertion);
        }
    } else if !has_api_base && has_config {
        // 只有 config，添加 apiBase（在 config 之前）
        // 找到 chatgpt.config 的位置
        if let Some(config_pos) = final_content.find(r#""chatgpt.config""#) {
            // 找到这一行的开始
            let before_config = &final_content[..config_pos];
            if let Some(line_start) = before_config.rfind('\n') {
                let indent_line = &final_content[line_start + 1..config_pos];
                let indent = indent_line.chars().take_while(|c| c.is_whitespace()).collect::<String>();

                let insertion = format!(r#""chatgpt.apiBase": "{}",{}"#, base_url, '\n');
                final_content.insert_str(line_start + 1, &format!("{}{}", indent, insertion));
            }
        }
    }

    // 写入配置
    crate::config::atomic_write(&settings_path, final_content.as_bytes())?;

    log::info!("已配置 ChatGPT 扩展使用自定义服务: {}, 请确保环境变量 key88={}", base_url, api_key);

    Ok(format!(
        "VSCode 配置成功！路径: {}\n已配置 ChatGPT 扩展使用自定义服务: {}\n请重新加载 VSCode 窗口以使配置生效。",
        settings_path.display(),
        base_url
    ))
}

/// 获取 VSCode 配置路径信息
pub fn get_vscode_paths_info() -> Vec<String> {
    candidate_settings_paths()
        .into_iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect()
}
