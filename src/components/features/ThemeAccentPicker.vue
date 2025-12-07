<script setup lang="ts">
const colors = [
  { name: 'Blue', value: '#0A84FF' },
  { name: 'Purple', value: '#BF5AF2' },
  { name: 'Pink', value: '#FF375F' },
  { name: 'Red', value: '#FF453A' },
  { name: 'Orange', value: '#FF9F0A' },
  { name: 'Yellow', value: '#FFD60A' },
  { name: 'Green', value: '#30D158' },
  { name: 'Teal', value: '#64D2FF' },
];

const setColor = (color: string) => {
  document.documentElement.style.setProperty('--color-primary', color);
};
</script>

<template>
  <div class="flex flex-wrap gap-3">
    <button
      v-for="color in colors"
      :key="color.name"
      @click="setColor(color.value)"
      class="w-8 h-8 rounded-full border border-white/10 hover:scale-110 transition-transform relative group"
      :style="{ backgroundColor: color.value }"
    >
      <span class="absolute -bottom-6 left-1/2 -translate-x-1/2 text-xs text-white opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap bg-black/80 px-2 py-1 rounded pointer-events-none">
        {{ color.name }}
      </span>
    </button>
  </div>
</template>
