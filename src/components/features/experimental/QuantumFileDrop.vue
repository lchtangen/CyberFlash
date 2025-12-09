<script setup lang="ts">
import { ref } from 'vue';

const isHovering = ref(false);
</script>

<template>
  <div 
    class="relative w-full h-48 rounded-2xl border-2 border-dashed transition-all duration-300 flex flex-col items-center justify-center overflow-hidden group cursor-pointer"
    :class="isHovering ? 'border-primary bg-primary/10' : 'border-white/20 bg-white/5 hover:border-white/40'"
    @dragenter.prevent="isHovering = true"
    @dragleave.prevent="isHovering = false"
    @drop.prevent="isHovering = false"
  >
    <!-- Background Grid -->
    <div class="absolute inset-0 bg-[url('data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjAiIGhlaWdodD0iMjAiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+PGNpcmNsZSBjeD0iMSIgY3k9IjEiIHI9IjEiIGZpbGw9InJnYmEoMjU1LDI1NSwyNTUsMC4xKSIvPjwvc3ZnPg==')] opacity-20"></div>

    <!-- Icon -->
    <div class="relative z-10 w-16 h-16 rounded-full bg-surface border border-white/10 flex items-center justify-center mb-4 group-hover:scale-110 transition-transform duration-300 shadow-xl">
      <span class="material-symbols-rounded text-3xl text-white/80 group-hover:text-primary transition-colors">cloud_upload</span>
      
      <!-- Orbiting Particle -->
      <div class="absolute inset-0 rounded-full border border-primary/30 animate-spin-slow opacity-0 group-hover:opacity-100"></div>
    </div>

    <!-- Text -->
    <div class="relative z-10 text-center">
      <h3 class="text-white font-bold text-lg group-hover:text-primary transition-colors">Drop Firmware Here</h3>
      <p class="text-white/50 text-sm mt-1">Supports .zip, .img, .payload.bin</p>
    </div>

    <!-- Quantum Glow -->
    <div class="absolute inset-0 bg-primary/20 blur-3xl opacity-0 group-hover:opacity-50 transition-opacity duration-500 pointer-events-none"></div>
  </div>
</template>

<style scoped>
.animate-spin-slow {
  animation: spin 3s linear infinite;
}
@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
</style>
