<script setup lang="ts">
import { computed } from 'vue';

const props = withDefaults(defineProps<{
  modelValue: boolean;
  label?: string;
  description?: string;
  color?: 'primary' | 'success' | 'warning' | 'error';
}>(), {
  color: 'primary'
});

defineEmits<{
  (e: 'update:modelValue', value: boolean): void;
}>();

const colorClasses = computed(() => {
  switch (props.color) {
    case 'success':
      return {
        bg: 'bg-success',
        bgOp: 'bg-success/20',
        border: 'border-success/50',
        text: 'text-success',
        shadow: 'shadow-[0_0_8px_rgba(48,209,88,0.8)]',
        shadowInset: 'shadow-[inset_0_0_10px_rgba(48,209,88,0.3)]',
        knobBg: 'bg-success'
      };
    case 'error':
      return {
        bg: 'bg-error',
        bgOp: 'bg-error/20',
        border: 'border-error/50',
        text: 'text-error',
        shadow: 'shadow-[0_0_8px_rgba(255,69,58,0.8)]',
        shadowInset: 'shadow-[inset_0_0_10px_rgba(255,69,58,0.3)]',
        knobBg: 'bg-error'
      };
    case 'warning':
      return {
        bg: 'bg-warning',
        bgOp: 'bg-warning/20',
        border: 'border-warning/50',
        text: 'text-warning',
        shadow: 'shadow-[0_0_8px_rgba(255,214,10,0.8)]',
        shadowInset: 'shadow-[inset_0_0_10px_rgba(255,214,10,0.3)]',
        knobBg: 'bg-warning'
      };
    default: // primary
      return {
        bg: 'bg-primary',
        bgOp: 'bg-primary/20',
        border: 'border-primary/50',
        text: 'text-primary',
        shadow: 'shadow-[0_0_8px_rgba(10,132,255,0.8)]',
        shadowInset: 'shadow-[inset_0_0_10px_rgba(10,132,255,0.3)]',
        knobBg: 'bg-primary'
      };
  }
});
</script>

<template>
  <div class="flex items-center justify-between py-3 group cursor-pointer" @click="$emit('update:modelValue', !modelValue)">
    <div class="flex-1 pr-4">
      <div v-if="label" class="text-sm font-medium text-white group-hover:text-white transition-colors duration-300 flex items-center gap-2">
        {{ label }}
        <span v-if="modelValue" class="w-1.5 h-1.5 rounded-full animate-pulse" :class="[colorClasses.bg, colorClasses.shadow]"></span>
      </div>
      <div v-if="description" class="text-xs text-text-secondary mt-0.5 group-hover:text-white/70 transition-colors">
        {{ description }}
      </div>
    </div>
    
    <button 
      class="relative w-12 h-7 rounded-full transition-all duration-300 focus:outline-none shadow-inner border border-white/5"
      :class="modelValue ? [colorClasses.bgOp, colorClasses.border] : 'bg-black/40 border-white/10 group-hover:border-white/30'"
    >
      <!-- Track Glow -->
      <div class="absolute inset-0 rounded-full transition-opacity duration-300" :class="[modelValue ? 'opacity-100' : 'opacity-0', colorClasses.shadowInset]"></div>

      <!-- Knob -->
      <div 
        class="absolute top-0.5 left-0.5 w-6 h-6 rounded-full shadow-lg transition-all duration-500 cubic-bezier flex items-center justify-center border border-white/10"
        :class="[
          modelValue ? ['translate-x-5 text-white', colorClasses.knobBg] : 'translate-x-0 bg-surface/80 text-text-muted',
          'backdrop-blur-sm'
        ]"
      >
        <!-- Icon inside the knob -->
        <span class="material-symbols-rounded text-[14px] font-bold transition-transform duration-500" :class="modelValue ? 'rotate-0' : '-rotate-180 opacity-50'">
          {{ modelValue ? 'check' : 'close' }}
        </span>
      </div>
      
      <!-- Knob Glow -->
      <div 
        class="absolute top-1 left-1 w-5 h-5 rounded-full blur-md transition-all duration-500 opacity-0 pointer-events-none"
        :class="[modelValue ? 'translate-x-5 opacity-60' : 'translate-x-0', colorClasses.bg]"
      ></div>
    </button>
  </div>
</template>

<style scoped>
.cubic-bezier {
  transition-timing-function: cubic-bezier(0.34, 1.56, 0.64, 1);
}
</style>
