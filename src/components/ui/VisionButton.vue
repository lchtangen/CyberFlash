<script setup lang="ts">
import { computed } from 'vue';

interface Props {
  variant?: 'primary' | 'secondary' | 'danger' | 'warning' | 'ghost';
  size?: 'sm' | 'md' | 'lg';
  disabled?: boolean;
  loading?: boolean;
  icon?: string;
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'primary',
  size: 'md',
  disabled: false,
  loading: false,
});

const classes = computed(() => {
  const base = 'relative inline-flex items-center justify-center font-medium transition-all duration-200 active:scale-95 disabled:opacity-50 disabled:pointer-events-none overflow-hidden';
  
  const variants = {
    primary: 'bg-primary text-white hover:brightness-110 shadow-lg shadow-primary/20 border border-white/10',
    secondary: 'bg-surface/50 text-white hover:bg-surface/70 border border-white/10 backdrop-blur-md',
    danger: 'bg-error/10 text-error hover:bg-error/20 border border-error/20 hover:border-error/40',
    warning: 'bg-warning/10 text-warning hover:bg-warning/20 border border-warning/20 hover:border-warning/40',
    ghost: 'bg-transparent text-text-secondary hover:text-white hover:bg-white/5',
  };
  
  const sizes = {
    sm: 'px-3 py-1.5 text-xs rounded-lg gap-1.5',
    md: 'px-5 py-2.5 text-sm rounded-xl gap-2',
    lg: 'px-8 py-4 text-base rounded-2xl gap-3',
  };

  return [base, variants[props.variant], sizes[props.size]];
});
</script>

<template>
  <button :class="classes" :disabled="disabled || loading">
    <!-- Loading Spinner -->
    <span v-if="loading" class="absolute inset-0 flex items-center justify-center bg-inherit">
      <svg class="animate-spin h-5 w-5" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
      </svg>
    </span>

    <!-- Icon (if provided) -->
    <span v-if="icon && !loading" class="material-symbols-rounded text-[1.2em]">{{ icon }}</span>
    
    <!-- Content -->
    <span :class="{ 'opacity-0': loading }">
      <slot />
    </span>
  </button>
</template>
