<script setup lang="ts">
defineProps<{
  variant?: 'default' | 'primary' | 'success' | 'warning' | 'error' | 'info';
  label?: string;
  icon?: string;
  pulse?: boolean;
}>();
</script>

<template>
  <span 
    class="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-[10px] font-bold uppercase tracking-wider border shadow-sm backdrop-blur-md"
    :class="[
      variant === 'primary' ? 'bg-primary/10 text-primary border-primary/20' : '',
      variant === 'success' ? 'bg-success/10 text-success border-success/20' : '',
      variant === 'warning' ? 'bg-warning/10 text-warning border-warning/20' : '',
      variant === 'error' ? 'bg-error/10 text-error border-error/20' : '',
      variant === 'info' ? 'bg-blue-400/10 text-blue-400 border-blue-400/20' : '',
      (!variant || variant === 'default') ? 'bg-white/10 text-white/70 border-white/10' : ''
    ]"
  >
    <span v-if="pulse" class="relative flex h-2 w-2">
      <span class="animate-ping absolute inline-flex h-full w-full rounded-full opacity-75" 
        :class="[
          variant === 'primary' ? 'bg-primary' : '',
          variant === 'success' ? 'bg-success' : '',
          variant === 'warning' ? 'bg-warning' : '',
          variant === 'error' ? 'bg-error' : '',
          (!variant || variant === 'default') ? 'bg-white' : ''
        ]"
      ></span>
      <span class="relative inline-flex rounded-full h-2 w-2"
        :class="[
          variant === 'primary' ? 'bg-primary' : '',
          variant === 'success' ? 'bg-success' : '',
          variant === 'warning' ? 'bg-warning' : '',
          variant === 'error' ? 'bg-error' : '',
          (!variant || variant === 'default') ? 'bg-white' : ''
        ]"
      ></span>
    </span>
    <span v-if="icon" class="material-symbols-rounded text-sm">{{ icon }}</span>
    <slot>{{ label }}</slot>
  </span>
</template>
