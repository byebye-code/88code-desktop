<template>
  <div class="flex-1 p-8 bg-gradient-to-br from-green-50 via-white to-emerald-50 overflow-y-auto">
    <div class="max-w-4xl mx-auto">
      <!-- 头部 -->
      <div class="flex items-center gap-4 mb-8">
        <div class="p-3 bg-gradient-to-br from-green-500 to-emerald-600 rounded-2xl shadow-lg">
          <DroidIcon :size="32" color="white" />
        </div>
        <div>
          <h2 class="text-3xl font-bold bg-gradient-to-r from-green-600 to-emerald-600 bg-clip-text text-transparent">
            Droid 配置
          </h2>
          <p class="text-gray-600 text-sm mt-1">
            一键配置 Droid（Factory）所有预设模型
          </p>
        </div>
      </div>

      <!-- 使用提示 -->
      <div class="bg-green-50 border border-green-200 rounded-xl p-4 mb-6">
        <div class="flex gap-3">
          <Info class="text-green-600 flex-shrink-0" :size="20" />
          <div class="text-sm text-green-800">
            <p class="font-semibold mb-1">配置说明</p>
            <p>只需输入一个 API 密钥，软件将自动配置所有预设模型到 <code class="bg-green-100 px-1.5 py-0.5 rounded">~/.factory/config.json</code>。高级用户可使用<strong>高级配置</strong>自定义。</p>
          </div>
        </div>
      </div>

      <!-- 当前模型列表 -->
      <div v-if="currentModels && currentModels.length > 0" class="mb-6">
        <div class="flex items-center justify-between mb-4">
          <h3 class="text-lg font-semibold text-gray-900 flex items-center gap-2">
            <List :size="20" />
            已配置的模型（{{ currentModels.length }}）
          </h3>
          <div class="flex gap-2">
            <button
              @click="openJsonEditor"
              class="px-3 py-1.5 text-xs bg-gradient-to-r from-indigo-500 to-purple-600 text-white rounded-lg hover:from-indigo-600 hover:to-purple-700 transition-all duration-200 flex items-center gap-1.5"
            >
              <Code :size="14" />
              高级配置
            </button>
            <button
              @click="handleExportConfig"
              class="px-3 py-1.5 text-xs bg-blue-100 hover:bg-blue-200 text-blue-700 rounded-lg transition-colors flex items-center gap-1.5"
              title="导出配置为 JSON"
            >
              <Download :size="14" />
              导出
            </button>
          </div>
        </div>
        <div class="space-y-3">
          <div
            v-for="model in currentModels"
            :key="model.model_display_name"
            class="bg-white border border-gray-200 rounded-xl p-4 hover:shadow-md transition-shadow"
          >
            <div class="flex items-start justify-between">
              <div class="flex-1">
                <h4 class="font-semibold text-gray-900 mb-1">{{ model.model_display_name }}</h4>
                <p class="text-xs text-gray-500 font-mono mb-1">模型: {{ model.model }}</p>
                <p class="text-xs text-gray-500 font-mono mb-1">Base URL: {{ model.base_url }}</p>
                <p class="text-xs text-gray-500">Provider: {{ model.provider }}</p>
                <p v-if="model.api_key && model.api_key !== '请替换为你的 API Key'" class="text-xs text-green-600 mt-1">✓ API Key 已配置</p>
                <p v-else class="text-xs text-amber-600 mt-1">⚠ API Key 未配置</p>
              </div>
              <button
                @click="handleDeleteModel(model.model_display_name)"
                class="px-3 py-1.5 text-xs bg-red-100 hover:bg-red-200 text-red-700 rounded-lg transition-colors flex items-center gap-1.5"
                title="删除此模型"
              >
                <Trash2 :size="14" />
                删除
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- 简化配置表单 - 仅 API 密钥 -->
      <div class="bg-white rounded-2xl shadow-xl p-8 mb-6 border border-gray-100 hover:shadow-2xl transition-shadow duration-300">
        <h3 class="text-lg font-semibold text-gray-900 mb-6 flex items-center gap-2">
          <Settings :size="20" />
          一键配置所有模型
        </h3>

        <div class="mb-8">
          <label class="block text-sm font-semibold text-gray-700 mb-3">
            API 密钥 <span class="text-red-500">*</span>
          </label>
          <input
            v-model="apiKey"
            type="password"
            class="w-full px-4 py-3 border-2 border-gray-200 rounded-xl focus:ring-2 focus:ring-green-500 focus:border-transparent outline-none transition-all duration-200"
            placeholder="输入您的 88code API 密钥"
          />
          <p class="text-xs text-gray-500 mt-2">
            此密钥将应用于所有预设模型（Sonnet 4.5、GPT-5、GPT-5-Codex）
          </p>
        </div>

        <button
          @click="handleConfigureAll"
          :disabled="isLoading"
          class="w-full bg-gradient-to-r from-green-600 to-emerald-600 hover:from-green-700 hover:to-emerald-700 disabled:from-gray-400 disabled:to-gray-400 text-white font-semibold py-4 rounded-xl transition-all duration-300 transform hover:scale-[1.02] active:scale-[0.98] shadow-lg hover:shadow-xl flex items-center justify-center gap-3"
        >
          <Settings :size="20" />
          {{ isLoading ? '配置中...' : '一键配置所有模型' }}
        </button>
      </div>

      <!-- 配置文件路径 -->
      <div v-if="configPaths" class="bg-green-50 border-2 border-green-200 rounded-xl p-5">
        <div class="flex items-center justify-between mb-3">
          <h3 class="text-sm font-semibold text-green-900 flex items-center gap-2">
            <FolderOpen :size="16" />配置文件路径
          </h3>
          <button
            @click="handleDeleteConfig"
            class="px-3 py-1.5 text-xs bg-red-100 hover:bg-red-200 text-red-700 rounded-lg transition-colors flex items-center gap-1.5"
            title="删除整个配置文件"
          >
            <Trash2 :size="14" />
            删除配置文件
          </button>
        </div>
        <p class="text-xs text-green-700 font-mono break-all bg-white rounded-lg p-3">
          {{ configPaths.droid_config }}
        </p>
      </div>
    </div>

    <!-- JSON 编辑器模态框 -->
    <div v-if="isJsonEditorOpen" class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div class="bg-white rounded-2xl shadow-2xl w-full max-w-4xl max-h-[90vh] overflow-hidden">
        <!-- 头部 -->
        <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-green-500 to-emerald-600">
          <div class="flex items-center justify-between">
            <h3 class="text-xl font-bold text-white flex items-center gap-2">
              <Code :size="24" />
              高级配置 - JSON 编辑器
            </h3>
            <button
              @click="closeJsonEditor"
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
                  <p>直接编辑完整的 config.json 文件内容。修改后将<strong>完全替换</strong>现有配置，请谨慎操作。首次修改时会自动创建备份文件。</p>
                </div>
              </div>
            </div>

            <!-- API 密钥输入 -->
            <div>
              <label class="block text-sm font-semibold text-gray-700 mb-3">
                API 密钥
              </label>
              <input
                v-model="advancedApiKey"
                type="password"
                class="w-full px-4 py-3 border-2 border-gray-200 rounded-xl focus:ring-2 focus:ring-green-500 focus:border-transparent outline-none transition-all duration-200"
                placeholder="输入 API 密钥，将自动同步到下方 JSON 配置中"
              />
              <p class="text-xs text-gray-500 mt-2">
                输入的 API 密钥会自动替换 JSON 中所有的 "你的API密钥" 占位符
              </p>
            </div>

            <!-- JSON 编辑器 -->
            <div>
              <div class="flex items-center justify-between mb-2">
                <label class="text-sm font-semibold text-gray-700">
                  config.json 完整内容
                </label>
                <div class="flex gap-2">
                  <button
                    @click="useDefaultJsonTemplate"
                    class="px-3 py-1 text-xs bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-lg transition-colors flex items-center gap-1"
                  >
                    <FileText :size="14" />
                    使用默认模板
                  </button>
                  <button
                    @click="formatConfigJson"
                    title="格式化 JSON"
                    class="px-3 py-1 text-xs bg-blue-100 hover:bg-blue-200 text-blue-700 rounded-lg transition-colors flex items-center gap-1"
                  >
                    <Code2 :size="14" />
                    格式化
                  </button>
                </div>
              </div>
              <textarea
                v-model="jsonEditorContent"
                class="w-full px-4 py-3 border-2 border-gray-200 rounded-xl focus:ring-2 focus:ring-green-500 focus:border-transparent outline-none transition-all duration-200 font-mono text-sm"
                rows="20"
                placeholder='{"custom_models": [...]}'
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
          </div>
        </div>

        <!-- 底部按钮 -->
        <div class="px-6 py-4 border-t border-gray-200 bg-gray-50">
          <div class="flex justify-end gap-3">
            <button
              @click="closeJsonEditor"
              class="px-6 py-2 bg-white border-2 border-gray-300 text-gray-700 font-medium rounded-xl hover:bg-gray-50 transition-colors"
            >
              取消
            </button>
            <button
              @click="handleApplyJsonConfig"
              :disabled="!!jsonError || !jsonEditorContent"
              class="px-6 py-2 bg-gradient-to-r from-green-600 to-emerald-600 text-white font-medium rounded-xl hover:from-green-700 hover:to-emerald-700 transition-all disabled:from-gray-400 disabled:to-gray-400 disabled:cursor-not-allowed"
            >
              应用配置
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, watch, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { Settings, FolderOpen, Info, Trash2, List, Code, Download, X, AlertTriangle, FileText, AlertCircle, CheckCircle, Code2 } from 'lucide-vue-next';
import DroidIcon from './icons/DroidIcon.vue';

const props = defineProps({
  configPaths: {
    type: Object,
    default: null
  }
});

const emit = defineEmits(['success', 'error']);

const apiKey = ref('');
const currentModels = ref([]);
const isLoading = ref(false);

// JSON 编辑器相关
const isJsonEditorOpen = ref(false);
const jsonEditorContentRaw = ref(''); // 原始 JSON 内容（带占位符）
const jsonError = ref('');
const advancedApiKey = ref(''); // 高级配置中的 API 密钥

// 计算属性：实时替换 API 密钥
const jsonEditorContent = computed({
  get() {
    if (advancedApiKey.value && jsonEditorContentRaw.value) {
      // 实时替换所有 "你的API密钥" 为实际的密钥
      return jsonEditorContentRaw.value.replace(/"你的API密钥"/g, `"${advancedApiKey.value}"`);
    }
    return jsonEditorContentRaw.value;
  },
  set(value) {
    jsonEditorContentRaw.value = value;
  }
});

// 预设的所有模型配置
const DEFAULT_MODELS = [
  {
    model_display_name: "Sonnet 4.5 [88code]",
    model: "claude-sonnet-4-5-20250929",
    base_url: "https://www.88code.org/droid",
    provider: "anthropic"
  },
  {
    model_display_name: "GPT-5-Codex [88code]",
    model: "gpt-5-codex",
    base_url: "https://www.88code.org/droid/v1",
    provider: "openai"
  },
  {
    model_display_name: "GPT-5 [88code]",
    model: "gpt-5",
    base_url: "https://www.88code.org/droid/v1",
    provider: "openai"
  }
];

// 监听 JSON 内容变化，验证格式
watch(jsonEditorContentRaw, (content) => {
  if (!content) {
    jsonError.value = '请输入配置内容';
    return;
  }

  // 验证时使用替换后的内容
  const contentToValidate = advancedApiKey.value && content
    ? content.replace(/"你的API密钥"/g, `"${advancedApiKey.value}"`)
    : content;

  try {
    JSON.parse(contentToValidate);
    jsonError.value = '';
  } catch (error) {
    jsonError.value = 'JSON 格式错误: ' + error.message;
  }
});

// 监听 API 密钥变化，重新验证 JSON
watch(advancedApiKey, () => {
  if (!jsonEditorContentRaw.value) return;

  const contentToValidate = advancedApiKey.value
    ? jsonEditorContentRaw.value.replace(/"你的API密钥"/g, `"${advancedApiKey.value}"`)
    : jsonEditorContentRaw.value;

  try {
    JSON.parse(contentToValidate);
    jsonError.value = '';
  } catch (error) {
    jsonError.value = 'JSON 格式错误: ' + error.message;
  }
});

// 加载当前配置
const loadCurrentConfig = async () => {
  try {
    const config = await invoke('get_current_droid_config');
    if (config && config.custom_models) {
      currentModels.value = config.custom_models;
    }
  } catch (error) {
    console.error('加载 Droid 配置失败:', error);
  }
};

onMounted(() => {
  loadCurrentConfig();
});

// 一键配置所有模型
const handleConfigureAll = async () => {
  if (!apiKey.value.trim()) {
    emit('error', '请输入 API 密钥');
    return;
  }

  isLoading.value = true;

  try {
    // 先删除现有配置
    try {
      await invoke('delete_droid_config');
    } catch (e) {
      // 忽略删除失败（可能文件不存在）
    }

    // 逐个添加预设模型
    for (const model of DEFAULT_MODELS) {
      await invoke('configure_droid', {
        modelDisplayName: model.model_display_name,
        model: model.model,
        baseUrl: model.base_url,
        apiKey: apiKey.value.trim(),
        provider: model.provider
      });
    }

    emit('success', '所有模型配置成功！');

    // 重新加载配置
    await loadCurrentConfig();

    // 清空密钥输入
    apiKey.value = '';
  } catch (error) {
    emit('error', '配置失败: ' + error);
  } finally {
    isLoading.value = false;
  }
};

// 导出配置
const handleExportConfig = () => {
  if (!currentModels.value || currentModels.value.length === 0) {
    emit('error', '当前没有可导出的配置');
    return;
  }

  const config = {
    custom_models: currentModels.value
  };

  const jsonStr = JSON.stringify(config, null, 2);
  const blob = new Blob([jsonStr], { type: 'application/json' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = 'droid-config.json';
  a.click();
  URL.revokeObjectURL(url);
  
  emit('success', '配置已导出为 droid-config.json');
};

// 打开 JSON 编辑器
const openJsonEditor = async () => {
  try {
    const config = await invoke('get_current_droid_config');
    if (config) {
      // 将 api_key 替换为占位符
      const configCopy = JSON.parse(JSON.stringify(config));
      if (configCopy.custom_models) {
        configCopy.custom_models.forEach(model => {
          if (model.api_key) {
            model.api_key = '你的API密钥';
          }
        });
      }
      jsonEditorContentRaw.value = JSON.stringify(configCopy, null, 2);
    } else {
      jsonEditorContentRaw.value = getDefaultJsonTemplate();
    }
  } catch (error) {
    jsonEditorContentRaw.value = getDefaultJsonTemplate();
  }
  isJsonEditorOpen.value = true;
};

// 关闭 JSON 编辑器
const closeJsonEditor = () => {
  isJsonEditorOpen.value = false;
  jsonEditorContentRaw.value = '';
  jsonError.value = '';
  advancedApiKey.value = '';
};

// 获取默认 JSON 模板
const getDefaultJsonTemplate = () => {
  return JSON.stringify({
    custom_models: DEFAULT_MODELS.map(model => ({
      ...model,
      api_key: "你的API密钥"
    }))
  }, null, 2);
};

// 使用默认 JSON 模板
const useDefaultJsonTemplate = () => {
  jsonEditorContentRaw.value = getDefaultJsonTemplate();
};

// 格式化 JSON
const formatConfigJson = () => {
  if (jsonError.value) {
    emit('error', 'JSON 格式错误，请先修正格式再格式化');
    return;
  }

  try {
    // 格式化时使用替换后的内容
    const contentToFormat = advancedApiKey.value && jsonEditorContentRaw.value
      ? jsonEditorContentRaw.value.replace(/"你的API密钥"/g, `"${advancedApiKey.value}"`)
      : jsonEditorContentRaw.value;
    const parsed = JSON.parse(contentToFormat);
    // 格式化后恢复占位符
    const formatted = JSON.stringify(parsed, null, 2);
    jsonEditorContentRaw.value = advancedApiKey.value
      ? formatted.replace(new RegExp(`"${advancedApiKey.value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}"`, 'g'), '"你的API密钥"')
      : formatted;
  } catch (error) {
    emit('error', 'JSON 格式错误: ' + error.message);
  }
};

// 应用 JSON 配置（完全替换配置文件）
const handleApplyJsonConfig = async () => {
  if (jsonError.value || !jsonEditorContentRaw.value) {
    return;
  }

  try {
    // 使用替换后的内容进行解析
    const contentToApply = advancedApiKey.value && jsonEditorContentRaw.value
      ? jsonEditorContentRaw.value.replace(/"你的API密钥"/g, `"${advancedApiKey.value}"`)
      : jsonEditorContentRaw.value;
    const config = JSON.parse(contentToApply);
    
    // 验证配置结构
    if (!config.custom_models || !Array.isArray(config.custom_models)) {
      emit('error', '配置格式错误: 必须包含 custom_models 数组');
      return;
    }

    // 验证每个模型的必填字段
    for (const model of config.custom_models) {
      if (!model.model_display_name || !model.base_url || !model.provider) {
        emit('error', '配置格式错误: 每个模型必须包含 model_display_name、base_url 和 provider');
        return;
      }
    }

    // 先删除配置文件
    await invoke('delete_droid_config');

    // 逐个添加模型
    for (const model of config.custom_models) {
      await invoke('configure_droid', {
        modelDisplayName: model.model_display_name,
        model: model.model || '',
        baseUrl: model.base_url,
        apiKey: model.api_key || '你的API密钥',
        provider: model.provider
      });
    }

    emit('success', '配置已成功应用！');
    
    // 重新加载配置
    await loadCurrentConfig();
    
    // 关闭编辑器
    closeJsonEditor();
  } catch (error) {
    emit('error', '应用配置失败: ' + error);
  }
};

const handleDeleteModel = async (modelDisplayName) => {
  if (!confirm(`确定要删除模型 "${modelDisplayName}" 吗？`)) {
    return;
  }

  try {
    const result = await invoke('delete_droid_model', {
      modelDisplayName: modelDisplayName
    });
    emit('success', result);

    // 重新加载配置
    await loadCurrentConfig();
  } catch (error) {
    emit('error', error);
  }
};

const handleDeleteConfig = async () => {
  if (!confirm('确定要删除 Droid 配置文件吗？\n\n这将清空所有自定义模型配置。此操作不可恢复。')) {
    return;
  }

  try {
    const result = await invoke('delete_droid_config');
    emit('success', result);

    // 清空当前模型列表
    currentModels.value = [];
  } catch (error) {
    emit('error', error);
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
