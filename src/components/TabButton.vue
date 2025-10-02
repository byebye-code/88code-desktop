<template>
  <button
    @click="$emit('click')"
    :class="[
      'px-6 py-3 font-medium text-sm rounded-lg transition-all duration-300',
      'focus:outline-none focus:ring-2 focus:ring-offset-2',
      active
        ? 'bg-white text-blue-600 shadow-md focus:ring-blue-500'
        : 'text-gray-600 hover:text-gray-900 hover:bg-gray-100'
    ]"
  >
    <slot />
  </button>
</template>

<script setup>
defineProps({
  active: {
    type: Boolean,
    default: false
  }
});

defineEmits(['click']);
</script>
