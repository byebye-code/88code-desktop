<template>
  <svg
    :width="size"
    :height="size"
    viewBox="0 0 24 24"
    :fill="color"
    xmlns="http://www.w3.org/2000/svg"
  >
    <title>Droid</title>
    <!-- Factory Droid 图标 - 机器人样式 -->
    <path
      d="M12 2C11.4477 2 11 2.44772 11 3V4H7C5.89543 4 5 4.89543 5 6V19C5 20.1046 5.89543 21 7 21H17C18.1046 21 19 20.1046 19 19V6C19 4.89543 18.1046 4 17 4H13V3C13 2.44772 12.5523 2 12 2ZM7 6H17V19H7V6ZM9 9C9 8.44772 9.44772 8 10 8C10.5523 8 11 8.44772 11 9C11 9.55228 10.5523 10 10 10C9.44772 10 9 9.55228 9 9ZM13 9C13 8.44772 13.4477 8 14 8C14.5523 8 15 8.44772 15 9C15 9.55228 14.5523 10 14 10C13.4477 10 13 9.55228 13 9ZM9 13H15V15H9V13Z"
    />
  </svg>
</template>

<script setup>
defineProps({
  size: {
    type: [Number, String],
    default: 24
  },
  color: {
    type: String,
    default: '#6366F1' // Factory 主题色 - 蓝紫色
  }
});
</script>
