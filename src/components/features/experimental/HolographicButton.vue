<script setup lang="ts">
defineProps<{
  label?: string;
  icon?: string;
}>();
</script>

<template>
  <button class="group relative px-8 py-4 bg-transparent border-none outline-none cursor-pointer perspective-500">
    <!-- Back Layer -->
    <div class="absolute inset-0 bg-primary/20 rounded-xl transform translate-z-[-10px] transition-transform duration-200 group-hover:translate-z-[-15px] group-active:translate-z-[-5px]"></div>
    
    <!-- Middle Layer (Glow) -->
    <div class="absolute inset-0 bg-primary/10 blur-xl rounded-xl opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>

    <!-- Front Layer -->
    <div class="relative bg-surface/80 backdrop-blur-md border border-white/10 rounded-xl p-4 flex items-center gap-3 transform transition-transform duration-200 group-hover:-translate-y-1 group-active:translate-y-0 shadow-xl group-hover:shadow-primary/20">
      <span v-if="icon" class="material-symbols-rounded text-primary">{{ icon }}</span>
      <span class="font-bold text-white tracking-wider">{{ label || 'INITIATE' }}</span>
      
      <!-- Shine -->
      <div class="absolute inset-0 rounded-xl bg-gradient-to-tr from-white/10 to-transparent opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none"></div>
    </div>
  </button>
</template>

<style scoped>
.perspective-500 {
  perspective: 500px;
}
.translate-z-\[-10px\] {
  transform: translateZ(-10px);
}
.translate-z-\[-15px\] {
  transform: translateZ(-15px);
}
.translate-z-\[-5px\] {
  transform: translateZ(-5px);
}
</style>
