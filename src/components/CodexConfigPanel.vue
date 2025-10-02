<template>
  <div class="flex-1 p-8 bg-gradient-to-br from-green-50 via-white to-emerald-50 overflow-y-auto">
    <div class="max-w-4xl mx-auto">
      <!-- 头部 -->
      <div class="flex items-center gap-4 mb-8">
        <div class="p-3 bg-gradient-to-br from-green-500 to-emerald-600 rounded-2xl shadow-lg">
          <Code2 class="text-white" :size="32" />
        </div>
        <div>
          <h2 class="text-3xl font-bold bg-gradient-to-r from-green-600 to-emerald-600 bg-clip-text text-transparent">
            Codex 配置
          </h2>
          <p class="text-gray-600 text-sm mt-1">
            配置您的 Codex 客户端并设置环境变量
          </p>
        </div>
      </div>

      <!-- 标签页 -->
      <div class="bg-gray-100 p-1.5 rounded-xl mb-6 inline-flex gap-1">
        <TabButton :active="activeTab === 'client'" @click="activeTab = 'client'">
          <Terminal :size="16" class="inline mr-2" />客户端配置
        </TabButton>
        <TabButton :active="activeTab === 'vscode'" @click="activeTab = 'vscode'">
          <Code :size="16" class="inline mr-2" />VSCode 配置
        </TabButton>
        <TabButton :active="activeTab === 'jetbrains'" @click="activeTab = 'jetbrains'">
          <Braces :size="16" class="inline mr-2" />JetBrains 配置
        </TabButton>
      </div>

      <!-- 客户端配置 -->
      <div v-show="activeTab === 'client'" class="animate-fade-in">
        <div class="bg-white rounded-2xl shadow-xl p-8 mb-6 border border-gray-100 hover:shadow-2xl transition-shadow duration-300">
          <div class="mb-6">
            <label class="block text-sm font-semibold text-gray-700 mb-3">
              Base URL
            </label>
            <input
              v-model="clientConfig.baseUrl"
              type="text"
              class="w-full px-4 py-3 border-2 border-gray-200 rounded-xl focus:ring-2 focus:ring-green-500 focus:border-transparent outline-none transition-all duration-200"
              placeholder="https://88code.org/openai/v1"
            />
          </div>

          <div class="mb-8">
            <label class="block text-sm font-semibold text-gray-700 mb-3">
              API 密钥
            </label>
            <input
              v-model="clientConfig.apiKey"
              type="password"
              class="w-full px-4 py-3 border-2 border-gray-200 rounded-xl focus:ring-2 focus:ring-green-500 focus:border-transparent outline-none transition-all duration-200"
              placeholder="输入您的 API 密钥"
            />
          </div>

          <button
            @click="handleClientConfigure"
            :disabled="isLoading.client"
            class="w-full bg-gradient-to-r from-green-600 to-emerald-600 hover:from-green-700 hover:to-emerald-700 disabled:from-gray-400 disabled:to-gray-400 text-white font-semibold py-4 rounded-xl transition-all duration-300 transform hover:scale-[1.02] active:scale-[0.98] shadow-lg hover:shadow-xl flex items-center justify-center gap-3"
          >
            <Settings :size="20" />
            {{ isLoading.client ? '配置中...' : '自动配置客户端' }}
          </button>
        </div>

        <div v-if="configPaths" class="bg-green-50 border-2 border-green-200 rounded-xl p-5 mb-4">
          <h3 class="text-sm font-semibold text-green-900 mb-3 flex items-center gap-2">
            <FolderOpen :size="16" />配置文件路径
          </h3>
          <div class="space-y-2">
            <div class="text-xs text-green-700 bg-white rounded-lg p-3">
              <span class="font-semibold">auth.json:</span>
              <span class="font-mono ml-2">{{ configPaths.codex_auth }}</span>
            </div>
            <div class="text-xs text-green-700 bg-white rounded-lg p-3">
              <span class="font-semibold">config.toml:</span>
              <span class="font-mono ml-2">{{ configPaths.codex_config }}</span>
            </div>
          </div>
        </div>

        <div class="bg-yellow-50 border-2 border-yellow-200 rounded-xl p-5">
          <h3 class="text-sm font-semibold text-yellow-900 mb-3 flex items-center gap-2">
            <AlertTriangle :size="16" />重要提示
          </h3>
          <ul class="text-xs text-yellow-800 space-y-2">
            <li class="flex items-start gap-2">
              <span class="text-yellow-500 mt-0.5">•</span>
              <span>配置完成后会自动设置环境变量 <code class="bg-yellow-100 px-1.5 py-0.5 rounded font-mono">key88</code></span>
            </li>
            <li class="flex items-start gap-2">
              <span class="text-yellow-500 mt-0.5">•</span>
              <span>Windows 用户需要重启 Codex 才能使环境变量生效</span>
            </li>
            <li class="flex items-start gap-2">
              <span class="text-yellow-500 mt-0.5">•</span>
              <span>Linux/macOS 用户需要重新打开终端或运行 <code class="bg-yellow-100 px-1.5 py-0.5 rounded font-mono">source ~/.zshrc</code></span>
            </li>
          </ul>
        </div>
      </div>

      <!-- VSCode 配置 -->
      <div v-show="activeTab === 'vscode'" class="animate-fade-in">
        <!-- 提示信息 -->
        <div class="bg-blue-50 border-2 border-blue-200 rounded-xl p-5 mb-6">
          <h3 class="text-sm font-semibold text-blue-900 mb-3 flex items-center gap-2">
            <Info :size="16" />配置说明
          </h3>
          <p class="text-xs text-blue-800">
            <strong>此配置可能已不需要。</strong>如果您在 VSCode 中使用 Codex 插件已经正常工作，可以不执行此自动配置。此功能主要用于配置 VSCode 的 <strong>ChatGPT 扩展</strong>使用 88code 服务。
          </p>
        </div>

        <div class="bg-white rounded-2xl shadow-xl p-8 mb-6 border border-gray-100 hover:shadow-2xl transition-shadow duration-300">
          <div class="mb-6">
            <label class="block text-sm font-semibold text-gray-700 mb-3">
              Base URL
            </label>
            <input
              v-model="vscodeConfig.baseUrl"
              type="text"
              class="w-full px-4 py-3 border-2 border-gray-200 rounded-xl focus:ring-2 focus:ring-green-500 focus:border-transparent outline-none transition-all duration-200"
              placeholder="https://88code.org/openai/v1"
            />
          </div>

          <div class="mb-8">
            <label class="block text-sm font-semibold text-gray-700 mb-3">
              API 密钥
            </label>
            <input
              v-model="vscodeConfig.apiKey"
              type="password"
              class="w-full px-4 py-3 border-2 border-gray-200 rounded-xl focus:ring-2 focus:ring-green-500 focus:border-transparent outline-none transition-all duration-200"
              placeholder="输入您的 API 密钥"
            />
          </div>

          <button
            @click="handleVSCodeConfigure"
            :disabled="isLoading.vscode"
            class="w-full bg-gradient-to-r from-green-600 to-emerald-600 hover:from-green-700 hover:to-emerald-700 disabled:from-gray-400 disabled:to-gray-400 text-white font-semibold py-4 rounded-xl transition-all duration-300 transform hover:scale-[1.02] active:scale-[0.98] shadow-lg hover:shadow-xl flex items-center justify-center gap-3"
          >
            <Code :size="20" />
            {{ isLoading.vscode ? '配置中...' : '自动配置 VSCode' }}
          </button>
        </div>

        <div class="bg-green-50 border-2 border-green-200 rounded-xl p-5">
          <h3 class="text-sm font-semibold text-green-900 mb-3 flex items-center gap-2">
            <Info :size="16" />配置详情
          </h3>
          <ul class="text-xs text-green-700 space-y-2">
            <li class="flex items-start gap-2">
              <span class="text-green-500 mt-0.5">•</span>
              <span>将配置 VSCode 的 <strong>ChatGPT 扩展</strong></span>
            </li>
            <li class="flex items-start gap-2">
              <span class="text-green-500 mt-0.5">•</span>
              <span>在 VSCode settings.json 中写入 <code class="bg-green-100 px-1.5 py-0.5 rounded font-mono">chatgpt.apiBase</code> 和 <code class="bg-green-100 px-1.5 py-0.5 rounded font-mono">chatgpt.config</code></span>
            </li>
            <li class="flex items-start gap-2">
              <span class="text-green-500 mt-0.5">•</span>
              <span>API Key 通过环境变量 <code class="bg-green-100 px-1.5 py-0.5 rounded font-mono">key88</code> 传递</span>
            </li>
            <li class="flex items-start gap-2">
              <span class="text-green-500 mt-0.5">•</span>
              <span>配置后需要重新加载 VSCode 窗口（Ctrl+Shift+P → Reload Window）</span>
            </li>
          </ul>
        </div>
      </div>

      <!-- JetBrains 配置（占位） -->
      <div v-show="activeTab === 'jetbrains'" class="animate-fade-in">
        <div class="bg-white rounded-2xl shadow-xl p-12 text-center border border-gray-100">
          <div class="inline-block p-4 bg-yellow-100 rounded-full mb-4">
            <Braces :size="48" class="text-yellow-600" />
          </div>
          <h3 class="text-xl font-bold text-gray-900 mb-2">JetBrains 配置</h3>
          <p class="text-gray-600 mb-6">此功能正在开发中，敬请期待...</p>
          <div class="inline-flex items-center gap-2 text-sm text-yellow-700 bg-yellow-50 px-4 py-2 rounded-lg">
            <AlertCircle :size="16" />
            <span>预计在下一个版本中推出</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { Code2, Settings, Code, Terminal, Braces, FolderOpen, Info, AlertCircle, AlertTriangle } from 'lucide-vue-next';
import TabButton from './TabButton.vue';

const props = defineProps({
  configPaths: {
    type: Object,
    default: null
  }
});

const emit = defineEmits(['success', 'error']);

const activeTab = ref('client');

const clientConfig = ref({
  baseUrl: 'https://88code.org/openai/v1',
  apiKey: ''
});

const vscodeConfig = ref({
  baseUrl: 'https://88code.org/openai/v1',
  apiKey: ''
});

const isLoading = ref({
  client: false,
  vscode: false
});

const handleClientConfigure = async () => {
  if (!clientConfig.value.apiKey.trim()) {
    emit('error', '请输入 API 密钥');
    return;
  }

  if (!clientConfig.value.baseUrl.trim()) {
    emit('error', '请输入 Base URL');
    return;
  }

  isLoading.value.client = true;

  try {
    const result = await invoke('configure_codex', {
      baseUrl: clientConfig.value.baseUrl.trim(),
      apiKey: clientConfig.value.apiKey.trim(),
    });

    emit('success', result);
    clientConfig.value.apiKey = '';
  } catch (error) {
    emit('error', error);
  } finally {
    isLoading.value.client = false;
  }
};

const handleVSCodeConfigure = async () => {
  if (!vscodeConfig.value.apiKey.trim()) {
    emit('error', '请输入 API 密钥');
    return;
  }

  if (!vscodeConfig.value.baseUrl.trim()) {
    emit('error', '请输入 Base URL');
    return;
  }

  isLoading.value.vscode = true;

  try {
    const result = await invoke('configure_vscode_codex', {
      baseUrl: vscodeConfig.value.baseUrl.trim(),
      apiKey: vscodeConfig.value.apiKey.trim(),
    });

    emit('success', result);
    vscodeConfig.value.apiKey = '';
  } catch (error) {
    emit('error', error);
  } finally {
    isLoading.value.vscode = false;
  }
};
</script>

<style scoped>
@keyframes fade-in {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.animate-fade-in {
  animation: fade-in 0.3s ease-out;
}
</style>
