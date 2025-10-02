<template>
  <div class="flex-1 p-8 bg-gradient-to-br from-blue-50 via-white to-indigo-50 overflow-y-auto">
    <div class="max-w-4xl mx-auto">
      <!-- 头部 -->
      <div class="flex items-center gap-4 mb-8">
        <div class="p-3 bg-gradient-to-br from-blue-500 to-blue-600 rounded-2xl shadow-lg">
          <Cloud class="text-white" :size="32" />
        </div>
        <div>
          <h2 class="text-3xl font-bold bg-gradient-to-r from-blue-600 to-indigo-600 bg-clip-text text-transparent">
            Claude Code 配置
          </h2>
          <p class="text-gray-600 text-sm mt-1">
            配置您的 Claude Code 客户端连接到 88code 服务
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
              class="w-full px-4 py-3 border-2 border-gray-200 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-all duration-200"
              placeholder="https://www.88code.org/api"
            />
          </div>

          <div class="mb-8">
            <label class="block text-sm font-semibold text-gray-700 mb-3">
              API 密钥
            </label>
            <input
              v-model="clientConfig.apiKey"
              type="password"
              class="w-full px-4 py-3 border-2 border-gray-200 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-all duration-200"
              placeholder="输入您的 API 密钥"
            />
          </div>

          <button
            @click="handleClientConfigure"
            :disabled="isLoading.client"
            class="w-full bg-gradient-to-r from-blue-600 to-indigo-600 hover:from-blue-700 hover:to-indigo-700 disabled:from-gray-400 disabled:to-gray-400 text-white font-semibold py-4 rounded-xl transition-all duration-300 transform hover:scale-[1.02] active:scale-[0.98] shadow-lg hover:shadow-xl flex items-center justify-center gap-3"
          >
            <Settings :size="20" />
            {{ isLoading.client ? '配置中...' : '自动配置客户端' }}
          </button>
        </div>

        <div v-if="configPaths" class="bg-blue-50 border-2 border-blue-200 rounded-xl p-5">
          <h3 class="text-sm font-semibold text-blue-900 mb-3 flex items-center gap-2">
            <FolderOpen :size="16" />配置文件路径
          </h3>
          <p class="text-xs text-blue-700 font-mono break-all bg-white rounded-lg p-3">
            {{ configPaths.claude_settings }}
          </p>
        </div>
      </div>

      <!-- VSCode 配置 -->
      <div v-show="activeTab === 'vscode'" class="animate-fade-in">
        <!-- 警告提示 -->
        <div class="bg-amber-50 border-2 border-amber-200 rounded-xl p-5 mb-6">
          <h3 class="text-sm font-semibold text-amber-900 mb-3 flex items-center gap-2">
            <AlertTriangle :size="16" />配置前提
          </h3>
          <p class="text-xs text-amber-800">
            <strong>请先完成客户端配置！</strong>VSCode 配置依赖于客户端配置文件，需要先在"客户端配置"标签页完成配置后，才能进行 VSCode 配置。
          </p>
        </div>

        <div class="bg-white rounded-2xl shadow-xl p-8 mb-6 border border-gray-100 hover:shadow-2xl transition-shadow duration-300">
          <div class="mb-8">
            <label class="block text-sm font-semibold text-gray-700 mb-3">
              API 密钥
            </label>
            <input
              v-model="vscodeConfig.apiKey"
              type="password"
              class="w-full px-4 py-3 border-2 border-gray-200 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-all duration-200"
              placeholder="输入您的 API 密钥"
            />
          </div>

          <button
            @click="handleVSCodeConfigure"
            :disabled="isLoading.vscode"
            class="w-full bg-gradient-to-r from-blue-600 to-indigo-600 hover:from-blue-700 hover:to-indigo-700 disabled:from-gray-400 disabled:to-gray-400 text-white font-semibold py-4 rounded-xl transition-all duration-300 transform hover:scale-[1.02] active:scale-[0.98] shadow-lg hover:shadow-xl flex items-center justify-center gap-3"
          >
            <Code :size="20" />
            {{ isLoading.vscode ? '配置中...' : '自动配置 VSCode' }}
          </button>
        </div>

        <div class="bg-blue-50 border-2 border-blue-200 rounded-xl p-5">
          <h3 class="text-sm font-semibold text-blue-900 mb-3 flex items-center gap-2">
            <Info :size="16" />VSCode 配置说明
          </h3>
          <ul class="text-xs text-blue-700 space-y-2">
            <li class="flex items-start gap-2">
              <span class="text-blue-500 mt-0.5">•</span>
              <span>将在 <code class="bg-blue-100 px-1.5 py-0.5 rounded font-mono">~/.claude/config.json</code> 中写入配置</span>
            </li>
            <li class="flex items-start gap-2">
              <span class="text-blue-500 mt-0.5">•</span>
              <span>写入内容：<code class="bg-blue-100 px-1.5 py-0.5 rounded font-mono">{"primaryApiKey": "您的密钥"}</code></span>
            </li>
            <li class="flex items-start gap-2">
              <span class="text-blue-500 mt-0.5">•</span>
              <span>配置后需要重新加载 VSCode 窗口（Ctrl+Shift+P → Reload Window）</span>
            </li>
          </ul>
        </div>
      </div>

      <!-- JetBrains 配置���占位） -->
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
import { Cloud, Settings, Code, Terminal, Braces, FolderOpen, Info, AlertCircle } from 'lucide-vue-next';
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
  baseUrl: 'https://www.88code.org/api',
  apiKey: ''
});

const vscodeConfig = ref({
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
    const result = await invoke('configure_claude_code', {
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

  isLoading.value.vscode = true;

  try {
    const result = await invoke('configure_vscode_claude', {
      baseUrl: '', // baseUrl 在后端不使用，传空字符串
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
