<template>
  <div class="w-72 bg-gradient-to-br from-gray-900 via-gray-800 to-gray-900 text-white flex flex-col shadow-2xl">
    <!-- 头部 -->
    <div class="p-6 border-b border-gray-700/50">
      <div class="flex items-center gap-3 mb-2">
        <div class="p-2.5 bg-gradient-to-br from-purple-500 to-pink-600 rounded-xl shadow-lg">
          <Sparkles :size="24" class="text-white" />
        </div>
        <div>
          <h1 class="text-xl font-bold bg-gradient-to-r from-purple-400 to-pink-400 bg-clip-text text-transparent">
            88code Desktop
          </h1>
        </div>
      </div>
      <p class="text-sm text-gray-400 ml-1">一键配置开发环境</p>
    </div>

    <!-- 导航 -->
    <nav class="flex-1 p-4 space-y-2">
      <button
        @click="$emit('panel-change', 'claude')"
        :class="[
          'w-full flex items-center gap-3 px-4 py-3.5 rounded-xl transition-all duration-300 group relative overflow-hidden',
          activePanel === 'claude'
            ? 'bg-gradient-to-r from-blue-600 to-indigo-600 text-white shadow-lg shadow-blue-500/30 scale-[1.02]'
            : 'text-gray-300 hover:bg-gray-800/60 hover:text-white hover:scale-[1.01]'
        ]"
      >
        <div :class="[
          'p-2 rounded-lg transition-all duration-300',
          activePanel === 'claude'
            ? 'bg-white/20'
            : 'bg-gray-700/50 group-hover:bg-gray-700'
        ]">
          <Cloud :size="18" />
        </div>
        <div class="flex-1 text-left">
          <span class="font-semibold text-sm">Claude Code</span>
          <p class="text-xs opacity-80 mt-0.5">AI 编程助手</p>
        </div>
        <div v-if="activePanel === 'claude'" class="absolute left-0 top-1/2 -translate-y-1/2 w-1 h-8 bg-white rounded-r-full"></div>
      </button>

      <button
        @click="$emit('panel-change', 'codex')"
        :class="[
          'w-full flex items-center gap-3 px-4 py-3.5 rounded-xl transition-all duration-300 group relative overflow-hidden',
          activePanel === 'codex'
            ? 'bg-gradient-to-r from-green-600 to-emerald-600 text-white shadow-lg shadow-green-500/30 scale-[1.02]'
            : 'text-gray-300 hover:bg-gray-800/60 hover:text-white hover:scale-[1.01]'
        ]"
      >
        <div :class="[
          'p-2 rounded-lg transition-all duration-300',
          activePanel === 'codex'
            ? 'bg-white/20'
            : 'bg-gray-700/50 group-hover:bg-gray-700'
        ]">
          <Code2 :size="18" />
        </div>
        <div class="flex-1 text-left">
          <span class="font-semibold text-sm">Codex</span>
          <p class="text-xs opacity-80 mt-0.5">代码生成工具</p>
        </div>
        <div v-if="activePanel === 'codex'" class="absolute left-0 top-1/2 -translate-y-1/2 w-1 h-8 bg-white rounded-r-full"></div>
      </button>
    </nav>

    <!-- 底部信息 -->
    <div class="p-4 border-t border-gray-700/50 bg-gray-900/50">
      <div class="flex items-center justify-between text-xs text-gray-400">
        <div class="flex items-center gap-2">
          <div class="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
          <span>运行中</span>
        </div>
        <span class="font-mono">v0.1.0</span>
      </div>
      <p class="text-xs text-gray-500 mt-2 text-center">© 2025 88code.org</p>
    </div>
  </div>
</template>

<script setup>
import { Cloud, Code2, Sparkles } from 'lucide-vue-next';

defineProps({
  activePanel: {
    type: String,
    required: true
  }
});

defineEmits(['panel-change']);
</script>
