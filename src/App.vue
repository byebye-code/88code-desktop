<template>
  <div class="flex h-screen bg-gray-100">
    <Sidebar :activePanel="activePanel" @panel-change="handlePanelChange" />

    <ClaudeConfigPanel
      v-if="activePanel === 'claude'"
      :configPaths="configPaths"
      @success="handleSuccess"
      @error="handleError"
    />

    <CodexConfigPanel
      v-if="activePanel === 'codex'"
      :configPaths="configPaths"
      @success="handleSuccess"
      @error="handleError"
    />

    <Notification :notification="notification" @close="notification = null" />
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import Sidebar from './components/Sidebar.vue';
import ClaudeConfigPanel from './components/ClaudeConfigPanel.vue';
import CodexConfigPanel from './components/CodexConfigPanel.vue';
import Notification from './components/Notification.vue';

const activePanel = ref('claude');
const configPaths = ref(null);
const notification = ref(null);

const handlePanelChange = (panel) => {
  activePanel.value = panel;
};

const showNotification = (type, message) => {
  notification.value = { type, message };
};

const handleSuccess = (message) => {
  showNotification('success', message);
};

const handleError = (message) => {
  showNotification('error', message);
};

onMounted(async () => {
  try {
    const paths = await invoke('get_config_paths');
    configPaths.value = paths;
  } catch (error) {
    console.error('加载配置路径失败:', error);
  }
});
</script>
