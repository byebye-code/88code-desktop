<template>
  <div v-if="isOpen" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
    <div class="bg-white rounded-2xl shadow-2xl w-full max-w-3xl max-h-[90vh] overflow-hidden">
      <!-- 头部 -->
      <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-indigo-500 to-purple-600">
        <div class="flex items-center justify-between">
          <h3 class="text-xl font-bold text-white flex items-center gap-2">
            <Settings2 :size="24" />
            高级配置
          </h3>
          <button
            @click="handleClose"
            class="p-1 hover:bg-white/20 rounded-lg transition-colors"
          >
            <X :size="24" class="text-white" />
          </button>
        </div>
      </div>

      <!-- 内容 -->
      <div class="p-6 overflow-y-auto max-h-[calc(90vh-140px)]">
        <div class="space-y-6">
          <!-- 提示信息 -->
          <div class="bg-amber-50 border border-amber-200 rounded-xl p-4">
            <div class="flex gap-3">
              <AlertTriangle class="text-amber-600 flex-shrink-0" :size="20" />
              <div class="text-sm text-amber-800">
                <p class="font-semibold mb-1">高级配置说明</p>
                <p>高级配置允许您完全自定义配置文件内容。修改这些设置需要了解配置文件格式。</p>
              </div>
            </div>
          </div>

          <!-- Base URL -->
          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">
              Base URL
            </label>
            <input
              v-model="advancedConfig.baseUrl"
              type="text"
              class="w-full px-4 py-3 border-2 border-gray-200 rounded-xl focus:ring-2 focus:ring-indigo-500 focus:border-transparent outline-none transition-all duration-200"
              :placeholder="defaultBaseUrl"
            />
          </div>

          <!-- API 密钥 -->
          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">
              API 密钥
            </label>
            <div class="relative">
              <input
                v-model="advancedConfig.apiKey"
                :type="showApiKey ? 'text' : 'password'"
                class="w-full px-4 py-3 pr-12 border-2 border-gray-200 rounded-xl focus:ring-2 focus:ring-indigo-500 focus:border-transparent outline-none transition-all duration-200"
                placeholder="输入您的 API 密钥"
              />
              <button
                @click="showApiKey = !showApiKey"
                class="absolute right-3 top-1/2 -translate-y-1/2 p-2 text-gray-500 hover:text-gray-700 transition-colors"
              >
                <Eye v-if="!showApiKey" :size="18" />
                <EyeOff v-else :size="18" />
              </button>
            </div>
          </div>

          <!-- Claude配置文件内容 -->
          <div v-if="props.type === 'claude'">
            <div class="flex items-center justify-between mb-2">
              <label class="text-sm font-semibold text-gray-700">
                自定义配置内容（settings.json）
              </label>
              <div class="flex gap-2">
                <button
                  @click="useDefaultTemplate"
                  class="px-3 py-1 text-xs bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-lg transition-colors flex items-center gap-1"
                >
                  <FileText :size="14" />
                  使用默认模板
                </button>
                <button
                  @click="formatJson"
                  title="格式化JSON"
                  class="px-3 py-1 text-xs bg-blue-100 hover:bg-blue-200 text-blue-700 rounded-lg transition-colors flex items-center gap-1"
                >
                  <Code2 :size="14" />
                  格式化
                </button>
              </div>
            </div>
            <textarea
              v-model="displayConfigContent"
              class="w-full px-4 py-3 border-2 border-gray-200 rounded-xl focus:ring-2 focus:ring-indigo-500 focus:border-transparent outline-none transition-all duration-200 font-mono text-sm"
              rows="12"
              placeholder="输入自定义的配置文件内容（JSON 格式）"
            ></textarea>
            <p v-if="jsonError" class="mt-2 text-sm text-red-600 flex items-center gap-1">
              <AlertCircle :size="16" />
              {{ jsonError }}
            </p>
            <p v-else class="mt-2 text-sm text-green-600 flex items-center gap-1">
              <CheckCircle :size="16" />
              JSON 格式正确
            </p>
          </div>

          <!-- Codex配置文件内容（两个文件） -->
          <div v-if="props.type === 'codex'" class="space-y-4">
            <!-- auth.json -->
            <div>
              <div class="flex items-center justify-between mb-2">
                <label class="text-sm font-semibold text-gray-700">
                  auth.json（API密钥配置）
                </label>
                <div class="flex gap-2">
                  <button
                    @click="useDefaultAuthTemplate"
                    class="px-3 py-1 text-xs bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-lg transition-colors flex items-center gap-1"
                  >
                    <FileText :size="14" />
                    使用默认模板
                  </button>
                  <button
                    @click="formatAuthJson"
                    title="格式化JSON"
                    class="px-3 py-1 text-xs bg-blue-100 hover:bg-blue-200 text-blue-700 rounded-lg transition-colors flex items-center gap-1"
                  >
                    <Code2 :size="14" />
                    格式化
                  </button>
                </div>
              </div>
              <textarea
                v-model="displayAuthContent"
                class="w-full px-4 py-3 border-2 border-gray-200 rounded-xl focus:ring-2 focus:ring-2 focus:ring-indigo-500 focus:border-transparent outline-none transition-all duration-200 font-mono text-sm"
                rows="4"
                placeholder='{"OPENAI_API_KEY": "你的API密钥"}'
              ></textarea>
              <p v-if="authJsonError" class="mt-2 text-sm text-red-600 flex items-center gap-1">
                <AlertCircle :size="16" />
                {{ authJsonError }}
              </p>
              <p v-else class="mt-2 text-sm text-green-600 flex items-center gap-1">
                <CheckCircle :size="16" />
                JSON 格式正确
              </p>
            </div>

            <!-- config.toml -->
            <div>
              <div class="flex items-center justify-between mb-2">
                <label class="text-sm font-semibold text-gray-700">
                  config.toml（服务配置）
                </label>
                <button
                  @click="useDefaultCodexTemplate"
                  class="px-3 py-1 text-xs bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-lg transition-colors flex items-center gap-1"
                >
                  <FileText :size="14" />
                  使用默认模板
                </button>
              </div>
              <textarea
                v-model="displayConfigToml"
                class="w-full px-4 py-3 border-2 border-gray-200 rounded-xl focus:ring-2 focus:ring-indigo-500 focus:border-transparent outline-none transition-all duration-200 font-mono text-sm"
                rows="10"
                placeholder="输入 config.toml 内容"
              ></textarea>
            </div>
          </div>
        </div>
      </div>

      <!-- 底部按钮 -->
      <div class="px-6 py-4 border-t border-gray-200 bg-gray-50">
        <div class="flex justify-end gap-3">
          <button
            @click="handleClose"
            class="px-6 py-2 bg-white border-2 border-gray-300 text-gray-700 font-medium rounded-xl hover:bg-gray-50 transition-colors"
          >
            取消
          </button>
          <button
            @click="handleApply"
            :disabled="hasError || !advancedConfig.apiKey"
            class="px-6 py-2 bg-gradient-to-r from-indigo-600 to-purple-600 text-white font-medium rounded-xl hover:from-indigo-700 hover:to-purple-700 transition-all disabled:from-gray-400 disabled:to-gray-400 disabled:cursor-not-allowed"
          >
            应用配置
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch, computed } from 'vue';
import { Settings2, X, Eye, EyeOff, AlertTriangle, FileText, AlertCircle, CheckCircle, Code2 } from 'lucide-vue-next';

const props = defineProps({
  isOpen: {
    type: Boolean,
    default: false
  },
  type: {
    type: String,
    required: true // 'claude' 或 'codex'
  }
});

const emit = defineEmits(['close', 'apply']);

const advancedConfig = ref({
  baseUrl: '',
  apiKey: '',
  configContent: '',
  authContent: '',  // Codex auth.json内容
  configToml: ''    // Codex config.toml内容
});

const showApiKey = ref(false);
const jsonError = ref('');
const authJsonError = ref('');

// 默认 Base URL
const defaultBaseUrl = computed(() => {
  return props.type === 'claude'
    ? 'https://www.88code.org/api'
    : 'https://88code.org/openai/v1';
});

// 判断是否有错误
const hasError = computed(() => {
  if (props.type === 'claude') {
    return !!jsonError.value;
  } else {
    return !!authJsonError.value;
  }
});

// 监听打开状态，初始化配置
watch(() => props.isOpen, (isOpen) => {
  if (isOpen) {
    if (props.type === 'claude') {
      advancedConfig.value = {
        baseUrl: defaultBaseUrl.value,
        apiKey: '',
        configContent: getDefaultTemplate(),
        authContent: '',
        configToml: ''
      };
    } else if (props.type === 'codex') {
      advancedConfig.value = {
        baseUrl: defaultBaseUrl.value,
        apiKey: '',
        configContent: '',
        authContent: getDefaultAuthJson(),
        configToml: getDefaultConfigToml()
      };
    }
  }
});

// 计算最终显示的配置内容（Claude），自动替换占位符
const displayConfigContent = computed({
  get() {
    if (props.type !== 'claude') return advancedConfig.value.configContent;

    let content = advancedConfig.value.configContent;
    if (advancedConfig.value.apiKey) {
      content = content.replace(/你的API密钥/g, advancedConfig.value.apiKey);
    }
    if (advancedConfig.value.baseUrl) {
      content = content.replace(/你的Base URL/g, advancedConfig.value.baseUrl);
    }
    return content;
  },
  set(value) {
    advancedConfig.value.configContent = value;
  }
});

// 计算最终显示的auth.json内容（Codex），自动替换占位符
const displayAuthContent = computed({
  get() {
    if (props.type !== 'codex') return advancedConfig.value.authContent;

    let content = advancedConfig.value.authContent;
    if (advancedConfig.value.apiKey) {
      content = content.replace(/你的API密钥/g, advancedConfig.value.apiKey);
    }
    return content;
  },
  set(value) {
    advancedConfig.value.authContent = value;
  }
});

// 计算最终显示的config.toml内容（Codex），自动替换占位符
const displayConfigToml = computed({
  get() {
    if (props.type !== 'codex') return advancedConfig.value.configToml;

    let content = advancedConfig.value.configToml;
    if (advancedConfig.value.baseUrl) {
      content = content.replace(/你的Base URL/g, advancedConfig.value.baseUrl);
    }
    return content;
  },
  set(value) {
    advancedConfig.value.configToml = value;
  }
});

// 监听 JSON 内容变化，验证格式（Claude）
watch(displayConfigContent, (content) => {
  if (props.type !== 'claude') return;

  if (!content) {
    jsonError.value = '请输入配置内容';
    return;
  }

  try {
    JSON.parse(content);
    jsonError.value = '';
  } catch (error) {
    jsonError.value = 'JSON 格式错误: ' + error.message;
  }
});

// 监听 auth.json 内容变化，验证格式（Codex）
watch(displayAuthContent, (content) => {
  if (props.type !== 'codex') return;

  if (!content) {
    authJsonError.value = '请输入auth.json内容';
    return;
  }

  try {
    JSON.parse(content);
    authJsonError.value = '';
  } catch (error) {
    authJsonError.value = 'JSON 格式错误: ' + error.message;
  }
});

// 获取默认模板（Claude）
function getDefaultTemplate() {
  return JSON.stringify({
    env: {
      "ANTHROPIC_AUTH_TOKEN": "你的API密钥",
      "ANTHROPIC_BASE_URL": "你的Base URL",
      "CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC": "1"
    },
    permissions: {
      allow: [],
      deny: []
    }
  }, null, 2);
}

// 获取默认auth.json（Codex）
function getDefaultAuthJson() {
  return JSON.stringify({
    "OPENAI_API_KEY": "你的API密钥"
  }, null, 2);
}

// 获取默认config.toml（Codex）
function getDefaultConfigToml() {
  return `model_provider = "88code"
model = "gpt-5-codex"
model_reasoning_effort = "high"
disable_response_storage = true

[model_providers.88code]
name = "88code"
base_url = "你的Base URL"
wire_api = "responses"
env_key = "key88"
requires_openai_auth = true`;
}

// 使用默认模板
function useDefaultTemplate() {
  advancedConfig.value.configContent = getDefaultTemplate();
}

// 使用默认Codex模板
function useDefaultCodexTemplate() {
  advancedConfig.value.configToml = getDefaultConfigToml();
}

// 格式化auth.json
function formatAuthJson() {
  try {
    const parsed = JSON.parse(advancedConfig.value.authContent);
    advancedConfig.value.authContent = JSON.stringify(parsed, null, 2);
    authJsonError.value = '';
  } catch (error) {
    authJsonError.value = 'JSON 格式错误，请先修正格式再格式化';
  }
}

// 使用默认auth.json模板
function useDefaultAuthTemplate() {
  advancedConfig.value.authContent = getDefaultAuthJson();
}

// 格式化 JSON
function formatJson() {
  try {
    const parsed = JSON.parse(advancedConfig.value.configContent);
    advancedConfig.value.configContent = JSON.stringify(parsed, null, 2);
    jsonError.value = '';
  } catch (error) {
    jsonError.value = 'JSON 格式错误，请先修正格式再格式化';
  }
}

// 处理关闭
function handleClose() {
  emit('close');
}

// 处理应用配置
function handleApply() {
  if (!advancedConfig.value.apiKey) {
    return;
  }

  if (props.type === 'claude') {
    if (jsonError.value) return;

    // 使用已经替换过的内容
    emit('apply', {
      baseUrl: advancedConfig.value.baseUrl,
      apiKey: advancedConfig.value.apiKey,
      configContent: displayConfigContent.value
    });
  } else if (props.type === 'codex') {
    if (authJsonError.value) return;

    // 使用已经替换过的内容
    emit('apply', {
      baseUrl: advancedConfig.value.baseUrl,
      apiKey: advancedConfig.value.apiKey,
      configContent: JSON.stringify({
        authJson: displayAuthContent.value,
        configToml: displayConfigToml.value
      })
    });
  }
}
</script>
