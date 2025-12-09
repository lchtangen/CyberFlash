<script setup lang="ts">
// GlassCard Component
defineProps<{
  noHover?: boolean;
  noPadding?: boolean;
}>();
</script>

<template>
  <div 
    class="relative overflow-hidden rounded-2xl border border-white/10 bg-surface/30 backdrop-blur-xl transition-all duration-300 ring-1 ring-white/5 shadow-2xl shadow-black/20"
    :class="[
      !noHover ? 'hover:bg-surface/40 hover:border-white/20 hover:shadow-primary/10 hover:scale-[1.002]' : '',
      !noPadding ? 'p-5' : ''
    ]"
  >
    <!-- Glow Effect -->
    <div v-if="!noHover" class="absolute inset-0 bg-gradient-to-br from-white/5 to-transparent opacity-0 hover:opacity-100 transition-opacity duration-500 pointer-events-none"></div>
    
    <div class="relative z-10">
      <slot />
    </div>
  </div>
</template>

<style scoped>
</style>
