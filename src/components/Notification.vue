<template>
  <div v-if="notification" class="fixed top-6 right-6 z-50 animate-slide-in">
    <div :class="[
      'relative overflow-hidden text-white px-6 py-4 rounded-2xl shadow-2xl flex items-start gap-4 min-w-96 max-w-md backdrop-blur-sm border',
      bgColors[notification.type],
      borderColors[notification.type]
    ]">
      <!-- 图标区域 -->
      <div :class="['flex-shrink-0 p-2.5 rounded-xl', iconBgColors[notification.type]]">
        <CheckCircle v-if="notification.type === 'success'" :size="20" class="animate-scale-in" />
        <XCircle v-if="notification.type === 'error'" :size="20" class="animate-shake" />
        <Info v-if="notification.type === 'info'" :size="20" class="animate-scale-in" />
      </div>

      <!-- 内容区域 -->
      <div class="flex-1 pt-0.5">
        <h4 class="font-semibold text-sm mb-1">{{ titles[notification.type] }}</h4>
        <p class="text-sm opacity-95 leading-relaxed">{{ notification.message }}</p>
      </div>

      <!-- 关闭按钮 -->
      <button
        @click="handleClose"
        class="flex-shrink-0 hover:bg-white/20 rounded-lg p-1.5 transition-all duration-200 hover:scale-110 active:scale-95"
      >
        <X :size="16" />
      </button>

      <!-- 进度条 -->
      <div class="absolute bottom-0 left-0 right-0 h-1 bg-white/20">
        <div
          class="h-full bg-white/60 transition-all duration-100 ease-linear"
          :style="{ width: `${progress}%` }"
        ></div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch, onUnmounted } from 'vue';
import { CheckCircle, XCircle, Info, X } from 'lucide-vue-next';

const props = defineProps({
  notification: {
    type: Object,
    default: null
  }
});

const emit = defineEmits(['close']);

const progress = ref(100);

const bgColors = {
  success: 'bg-gradient-to-r from-green-600 to-emerald-600',
  error: 'bg-gradient-to-r from-red-600 to-rose-600',
  info: 'bg-gradient-to-r from-blue-600 to-indigo-600'
};

const borderColors = {
  success: 'border-green-400/30',
  error: 'border-red-400/30',
  info: 'border-blue-400/30'
};

const iconBgColors = {
  success: 'bg-white/20',
  error: 'bg-white/20',
  info: 'bg-white/20'
};

const titles = {
  success: '操作成功',
  error: '操作失败',
  info: '提示信息'
};

let timer = null;
let progressInterval = null;

const handleClose = () => {
  if (timer) clearTimeout(timer);
  if (progressInterval) clearInterval(progressInterval);
  emit('close');
};

watch(() => props.notification, (newVal) => {
  if (newVal) {
    // 清除之前的定时器
    if (timer) clearTimeout(timer);
    if (progressInterval) clearInterval(progressInterval);

    // 重置进度
    progress.value = 100;

    // 进度条动画
    const duration = 5000;
    const interval = 50;
    const decrement = (100 * interval) / duration;

    progressInterval = setInterval(() => {
      progress.value = Math.max(0, progress.value - decrement);
    }, interval);

    // 自动关闭定时器
    timer = setTimeout(() => {
      emit('close');
      if (progressInterval) clearInterval(progressInterval);
    }, duration);
  }
});

onUnmounted(() => {
  if (timer) clearTimeout(timer);
  if (progressInterval) clearInterval(progressInterval);
});
</script>

<style scoped>
/* 滑入动画 */
@keyframes slide-in {
  from {
    opacity: 0;
    transform: translateX(24rem);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

.animate-slide-in {
  animation: slide-in 0.4s cubic-bezier(0.16, 1, 0.3, 1);
}

/* 图标缩放动画 */
@keyframes scale-in {
  0% {
    transform: scale(0);
    opacity: 0;
  }
  50% {
    transform: scale(1.2);
  }
  100% {
    transform: scale(1);
    opacity: 1;
  }
}

.animate-scale-in {
  animation: scale-in 0.5s cubic-bezier(0.16, 1, 0.3, 1);
}

/* 错误图标抖动动画 */
@keyframes shake {
  0%, 100% {
    transform: translateX(0);
  }
  25% {
    transform: translateX(-4px);
  }
  75% {
    transform: translateX(4px);
  }
}

.animate-shake {
  animation: shake 0.4s ease-in-out, scale-in 0.5s cubic-bezier(0.16, 1, 0.3, 1);
}
</style>
