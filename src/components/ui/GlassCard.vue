<script setup lang="ts">
defineProps<{
  noHover?: boolean;
  noPadding?: boolean;
}>();
</script>

<template>
  <div 
    class="relative overflow-hidden rounded-2xl border border-white/10 bg-surface/30 backdrop-blur-xl transition-all duration-300"
    :class="[
      !noHover ? 'hover:bg-surface/40 hover:border-white/20 hover:shadow-lg' : '',
      !noPadding ? 'p-6' : ''
    ]"
  >
    <slot />
  </div>
</template>
