# 88code Desktop

88code Claude Code 和 Codex 配置工具

## 功能特性

### 1. Claude Code 配置
- **自动配置模式**: 快速配置 Base URL 和 API 密钥
- **高级配置模式**: 自定义完整 JSON 配置内容
- 自动配置 `~/.claude/settings.json` 文件
- 支持配置续写，保留现有配置字段
- 跨平台支持（Windows/macOS/Linux）

### 2. Code 终端配置
- 配置环境变量 `ANTHROPIC_BASE_URL` 和 `ANTHROPIC_AUTH_TOKEN`
- Windows: 永久设置系统环境变量
- Linux/macOS: 写入 shell 配置文件（.zshrc/.bashrc）
- 支持清空配置功能

### 3. Codex 配置
- **自动配置模式**: 快速配置 API 密钥和 Base URL
- **高级配置模式**: 自定义 auth.json 和 config.toml 内容
- 自动配置 `~/.codex/auth.json` 和 `config.toml`
- 支持配置续写，保留现有配置字段
- **永久设置环境变量 key88**

### 4. VSCode 扩展配置
- **Claude 扩展**: 配置 `~/.claude/config.json`
- **Codex (ChatGPT) 扩展**: 配置 VSCode settings.json
- 自动检测 VSCode 安装路径（支持 Stable/Insiders/VSCodium）

### 🆕 5. 配置备份功能 (v1.1.0)
- **首次配置自动备份**: 在修改配置前自动创建 .bak 备份文件
- **智能备份保护**: 仅首次创建备份，已存在的 .bak 文件不会被覆盖
- **全面覆盖**: 所有配置文件（Claude/Codex/VSCode/终端）均支持备份
- **备份位置**:
  - `~/.claude/settings.json.bak`
  - `~/.claude/config.json.bak`
  - `~/.codex/auth.json.bak`
  - `~/.codex/config.toml.bak`
  - `[VSCode配置目录]/settings.json.bak`

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
## 使用
windows可在release中下载exe直接运行

linux和mac可本地自行build

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
2. **自动备份**: 首次配置前会自动创建 .bak 备份文件，保护原始配置
3. **配置续写**: 配置文件已存在时，会智能合并现有配置，不会丢失其他字段
4. 配置使用原子写入机制，确保配置文件完整性
5. API 密钥以密码形式输入，配置成功后会自动清空输入框
6. 高级配置支持 JSON/TOML 格式验证，确保配置正确性

## 项目结构

```
88code-desktop/
├── src/                                 # Vue 3 前端代码
│   ├── components/                      # Vue 组件
│   │   ├── icons/                       # 图标组件
│   │   │   ├── ClaudeIcon.vue           # Claude 图标
│   │   │   ├── CodexIcon.vue            # Codex 图标
│   │   │   ├── VSCodeIcon.vue           # VSCode 图标
│   │   │   ├── JetBrainsIcon.vue        # JetBrains 图标
│   │   │   └── TerminalIcon.vue         # 终端图标
│   │   ├── Sidebar.vue                  # 侧边栏导航
│   │   ├── TabButton.vue                # 标签按钮组件
│   │   ├── ClaudeConfigPanel.vue        # Claude Code 配置面板
│   │   ├── CodexConfigPanel.vue         # Codex 配置面板
│   │   ├── AdvancedConfigModal.vue      # 高级配置模态框
│   │   └── Notification.vue             # 通知组件
│   ├── assets/                          # 静态资源
│   │   └── vue.svg                      # Vue logo
│   ├── App.vue                          # 主应用组件
│   ├── main.js                          # Vue 应用入口
│   ├── types.ts                         # TypeScript 类型定义
│   └── index.css                        # Tailwind CSS 全局样式
├── src-tauri/                           # Rust 后端代码
│   ├── src/
│   │   ├── config.rs                    # 配置路径管理、原子写入、备份功能
│   │   ├── claude_config.rs             # Claude Code 配置逻辑
│   │   ├── codex_config.rs              # Codex 配置逻辑
│   │   ├── vscode.rs                    # VSCode 扩展配置逻辑
│   │   ├── env_manager.rs               # 环境变量管理（终端配置）
│   │   ├── commands.rs                  # Tauri 命令定义
│   │   ├── lib.rs                       # 库主模块
│   │   └── main.rs                      # 应用入口
│   ├── icons/                           # 应用图标资源
│   ├── capabilities/                    # Tauri 权限配置
│   ├── Cargo.toml                       # Rust 依赖配置
│   └── tauri.conf.json                  # Tauri 应用配置
├── public/                              # 静态资源
│   ├── tauri.svg
│   └── vite.svg
├── package.json                         # Node.js 依赖配置
├── vite.config.js                       # Vite 构建配置
├── tailwind.config.js                   # Tailwind CSS 配置
├── index.html                           # HTML 入口
└── README.md                            # 项目文档
```

## 许可证

MIT

## 版本历史

### v1.1.0 (2025-10-05)


- 🆕 新增配置备份功能，首次配置自动创建 .bak 备份文件

- ✨ 智能备份保护，已存在的备份不会被覆盖

- 📦 全面支持所有配置文件的备份（Claude/Codex/VSCode/终端）

- 🔧 优化配置续写逻辑，确保不破坏原有配置

### v1.0.0 (2025-10-04)

- ✅ Claude Code 自动配置
- ✅ Codex 自动配置
- ✅ Code 终端环境变量配置
- ✅ VSCode 扩展配置
- ✅ 高级配置模式
- ✅ 官方图标
- ✅ 跨平台支持（Windows/macOS/Linux）

### v0.1.0 (2025-10-01)

- ✅ Claude Code 自动配置
- ✅ Codex 自动配置
- ✅ Code 终端环境变量配置
- ✅ VSCode 扩展配置
- ✅ 跨平台支持（Windows/macOS/Linux）

---

© 2025 88code
