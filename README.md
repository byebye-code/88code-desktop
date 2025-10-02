# 88code Desktop

88code Claude Code 和 Codex 自动配置工具

## 功能特性

### 1. Claude Code 自动配置
- 用户输入 Base URL 和 API 密钥
- 自动配置 `~/.claude/settings.json` 文件
- 跨平台支持（Windows/macOS/Linux）

### 2. Codex 自动配置
- 用户输入 API 密钥
- 自动配置 `~/.codex/auth.json` 和 `config.toml`
- **永久设置环境变量 key88**

## 技术栈

- **前端**: Vue 3.5.13 (Composition API) + Tailwind CSS 4
- **后端**: Rust + Tauri 2.8
- **构建工具**: Vite 6
- **包管理器**: pnpm

## 开发

### 安装依赖

```bash
pnpm install
```

### 开发模式

```bash
pnpm tauri dev
```

### 构建生产版本

```bash
pnpm tauri build
```

## 配置路径

### Windows
- Claude Code: `C:\Users\<用户名>\.claude\settings.json`
- Codex: `C:\Users\<用户名>\.codex\`

### macOS/Linux
- Claude Code: `~/.claude/settings.json`
- Codex: `~/.codex/`

## 使用说明

### Claude Code 配置

1. 在左侧导航栏选择"Claude Code 配置"
2. 输入 Base URL（默认：https://www.88code.org/api）
3. 输入 API 密钥
4. 点击"自动配置"按钮
5. 配置成功后会显示通知

### Codex 配置

1. 在左侧导航栏选择"Codex 配置"
2. 输入 API 密钥
3. 点击"自动配置"按钮
4. 配置成功后：
   - Windows 用户需要重启 Codex 才能使环境变量生效
   - Linux/macOS 用户需要重启终端或运行 `source ~/.zshrc`

## 环境变量说明

Codex 配置会自动设置环境变量 `key88=<您的API密钥>`

- **Windows**: 使用 `setx` 命令设置用户环境变量
- **Linux/macOS**: 写入 shell 配置文件（.zshrc/.bashrc）

## 注意事项

1. 首次配置会自动创建配置目录和文件
2. 如果配置文件已存在，会合并现有配置（Claude Code）或覆盖（Codex）
3. 配置使用原子写入机制，确保配置文件完整性
4. API 密钥以密码形式输入，配置成功后会自动清空输入框

## 项目结构

```
88code-desktop/
├── src/                          # Vue 3 前端代码
│   ├── components/               # Vue 组件
│   │   ├── Sidebar.vue          # 侧边栏导航
│   │   ├── ClaudeConfigPanel.vue # Claude 配置面板
│   │   ├── CodexConfigPanel.vue  # Codex 配置面板
│   │   └── Notification.vue      # 通知组件
│   ├── App.vue                   # 主应用
│   ├── main.js                   # 入口文件
│   ├── types.ts                  # TypeScript 类型定义
│   └── index.css                 # 全局样式
├── src-tauri/                    # Rust 后端代码
│   └── src/
│       ├── config.rs             # 配置路径管理和原子写入
│       ├── claude_config.rs      # Claude 配置逻辑
│       ├── codex_config.rs       # Codex 配置逻辑
│       ├── env_manager.rs        # 环境变量管理
│       ├── commands.rs           # Tauri 命令
│       └── lib.rs                # 主模块
├── package.json                  # 前端依赖
├── Cargo.toml                    # Rust 依赖
└── README.md                     # 项目说明
```

## 许可证

MIT

## 版本

0.1.0

---

© 2025 88code
