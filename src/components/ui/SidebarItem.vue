<script setup lang="ts">
defineProps<{
  icon: string;
  label: string;
  active?: boolean;
  variant?: 'default' | 'primary';
  description?: string;
}>();

defineEmits<{
  (e: 'click'): void;
}>();
</script>

<template>
  <button 
    @click="$emit('click')"
    class="w-full text-left px-3 py-2 rounded-lg transition-all duration-200 flex items-center gap-3 group relative overflow-hidden"
    :class="[
      active 
        ? (variant === 'primary' ? 'bg-primary text-white shadow-lg shadow-primary/20' : 'bg-white/10 text-white border border-white/5') 
        : 'text-text-secondary hover:bg-white/5 hover:text-white'
    ]"
  >
    <!-- Active Indicator (Left Bar) -->
    <div 
      v-if="active && variant !== 'primary'" 
      class="absolute left-0 top-1/2 -translate-y-1/2 w-1 h-6 bg-primary rounded-r-full"
    ></div>

    <!-- Icon -->
    <div 
      class="w-8 h-8 rounded-md flex items-center justify-center transition-colors"
      :class="[
        active 
          ? (variant === 'primary' ? 'bg-white/20' : 'bg-white/5') 
          : 'bg-white/5 group-hover:bg-white/10'
      ]"
    >
      <span class="material-symbols-rounded text-lg" :class="{ 'animate-pulse': variant === 'primary' && active }">{{ icon }}</span>
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
