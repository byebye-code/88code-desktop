// 配置路径信息
export interface ConfigPaths {
  claude_dir: string;
  claude_settings: string;
  codex_dir: string;
  codex_auth: string;
  codex_config: string;
}

// Claude 配置
export interface ClaudeSettings {
  env: {
    ANTHROPIC_AUTH_TOKEN: string;
    ANTHROPIC_BASE_URL: string;
    CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC: string;
  };
  permissions: {
    allow: string[];
    deny: string[];
  };
}

// Codex 认证配置
export interface CodexAuth {
  OPENAI_API_KEY: string;
}

// 通知类型
export type NotificationType = "success" | "error" | "info";

export interface Notification {
  type: NotificationType;
  message: string;
}

// 面板类型
export type PanelType = "claude" | "codex";
