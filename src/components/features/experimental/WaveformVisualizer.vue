<script setup lang="ts">
import { onMounted, ref } from 'vue';

const bars = ref<number[]>(new Array(20).fill(10));

onMounted(() => {
  setInterval(() => {
    bars.value = bars.value.map(() => Math.max(10, Math.random() * 100));
  }, 100);
});
</script>

<template>
  <div class="w-full h-full flex items-center justify-center gap-1 bg-black/20 rounded-xl p-4 backdrop-blur-sm border border-white/5">
    <div 
      v-for="(height, index) in bars" 
      :key="index"
      class="w-1.5 bg-gradient-to-t from-primary/20 to-primary rounded-full transition-all duration-100 ease-linear"
      :style="{ height: `${height}%` }"
    ></div>
  </div>
</template>
