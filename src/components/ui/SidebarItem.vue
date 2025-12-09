<script setup lang="ts">
defineProps<{
  icon: string;
  label: string;
  active?: boolean;
  variant?: 'default' | 'primary' | 'success' | 'warning' | 'error' | 'info';
  description?: string;
  color?: string; // Optional custom color class (e.g., 'text-purple-400')
}>();

defineEmits<{
  (e: 'click'): void;
}>();
</script>

<template>
  <button 
    @click="$emit('click')"
    class="w-full text-left px-3 py-2 rounded-xl transition-all duration-300 flex items-center gap-3 group relative overflow-hidden"
    :class="[
      active 
        ? (variant === 'primary' ? 'bg-primary text-white shadow-lg shadow-primary/20' : 'bg-surface/50 backdrop-blur-xl text-white shadow-lg shadow-black/20 border border-white/10') 
        : 'text-text-secondary hover:bg-white/5 hover:text-white hover:translate-x-1'
    ]"
  >
    <!-- Active Indicator (Left Bar) -->
    <div 
      v-if="active && variant !== 'primary'" 
      class="absolute left-0 top-1/2 -translate-y-1/2 w-1 h-5 rounded-r-full transition-colors duration-300"
      :class="color ? color.replace('text-', 'bg-') : 'bg-primary'"
    ></div>

    <!-- Icon -->
    <div 
      class="w-8 h-8 rounded-lg flex items-center justify-center transition-all duration-300"
      :class="[
        active 
          ? (variant === 'primary' ? 'bg-white/20 scale-110' : 'bg-white/10 scale-110') 
          : 'bg-white/5 group-hover:bg-white/10 group-hover:scale-105'
      ]"
    >
      <span 
        class="material-symbols-rounded text-[18px] transition-colors duration-300" 
        :class="[
          { 'animate-pulse': variant === 'primary' && active },
          active && variant !== 'primary' && !color ? 'text-primary' : '',
          color ? color : ''
        ]"
      >{{ icon }}</span>
    </div>

    <!-- Text -->
    <div class="flex flex-col leading-none">
      <span class="text-sm font-medium tracking-wide">{{ label }}</span>
      <span v-if="description" class="text-[10px] opacity-70 mt-0.5 font-normal">{{ description }}</span>
    </div>

    <!-- Hover Glow -->
    <div class="absolute inset-0 bg-gradient-to-r from-white/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none"></div>
  </button>
</template>
