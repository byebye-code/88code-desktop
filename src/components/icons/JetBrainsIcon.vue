<template>
  <svg
    :width="size"
    :height="size"
    viewBox="0 0 24 24"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
  >
    <defs>
      <linearGradient id="jetbrains-grad-1" x1="0" y1="22.5" x2="12" y2="0" gradientUnits="userSpaceOnUse">
        <stop offset="0.22" stop-color="#FC801D" />
        <stop offset="0.28" stop-color="#B07F61" />
        <stop offset="0.87" stop-color="#577BAE" />
        <stop offset="0.96" stop-color="#1E7CE6" />
      </linearGradient>
      <linearGradient id="jetbrains-grad-2" x1="10.5" y1="13.5" x2="24" y2="1.5" gradientUnits="userSpaceOnUse">
        <stop offset="0.09" stop-color="#FC801D" />
        <stop offset="0.23" stop-color="#B07F61" />
        <stop offset="0.87" stop-color="#577BAE" />
        <stop offset="1" stop-color="#1E7CE6" />
      </linearGradient>
      <linearGradient id="jetbrains-grad-3" x1="12.75" y1="4.5" x2="1.5" y2="18.75" gradientUnits="userSpaceOnUse">
        <stop offset="0" stop-color="#FE2857" />
        <stop offset="0.08" stop-color="#CB3979" />
        <stop offset="0.16" stop-color="#9E4997" />
        <stop offset="0.25" stop-color="#7557B2" />
        <stop offset="0.34" stop-color="#5362C8" />
        <stop offset="0.44" stop-color="#386CDA" />
        <stop offset="0.54" stop-color="#2373E8" />
        <stop offset="0.66" stop-color="#1478F2" />
        <stop offset="0.79" stop-color="#0B7BF8" />
        <stop offset="1" stop-color="#087CFA" />
      </linearGradient>
    </defs>
    <path d="M0 0l4.56 4.44L0 7.68zM22.32 7.2L24 0l-7.2 2.4z" fill="url(#jetbrains-grad-1)" />
    <path d="M9.84 24l7.68-4.08-3.6-2.4L9.84 24zM24 0l-8.4 7.68 4.56 4.08z" fill="url(#jetbrains-grad-2)" />
    <path d="M0 0l11.52 10.8 4.08-3.12L9.6 0zM15.6 11.76L24 24l-11.76-8.4z" fill="url(#jetbrains-grad-3)" />
    <path d="M4.8 4.8h14.4v14.4H4.8z" :fill="color" />
    <path d="M6.72 17.04h6.48v1.08H6.72z" fill="#fff" />
    <path d="M7.2 7.68h2.4L6.96 11.4l2.88 3.6H7.2l-2.16-2.88L6.96 9.6l-1.92-1.92h2.16zm5.76 3.84v2.88h-.96v-7.2l3.6-.24v.96l-2.64.24v2.4h2.4v.96h-2.4z" fill="#fff" />
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
    default: '#000000'
  }
});
</script>
