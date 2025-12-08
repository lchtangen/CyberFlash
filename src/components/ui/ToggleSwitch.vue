<script setup lang="ts">
defineProps<{
  modelValue: boolean;
  label?: string;
  description?: string;
}>();

defineEmits<{
  (e: 'update:modelValue', value: boolean): void;
}>();
</script>

<template>
  <div class="flex items-center justify-between py-3 group">
    <div class="flex-1 pr-4">
      <div v-if="label" class="text-sm font-medium text-white group-hover:text-primary transition-colors duration-300">
        {{ label }}
      </div>
      <div v-if="description" class="text-xs text-text-secondary mt-0.5">
        {{ description }}
      </div>
    </div>
    
    <button 
      class="relative w-12 h-7 rounded-full transition-colors duration-300 focus:outline-none focus:ring-2 focus:ring-primary/50 focus:ring-offset-2 focus:ring-offset-black"
      :class="modelValue ? 'bg-primary' : 'bg-white/10 hover:bg-white/20'"
      @click="$emit('update:modelValue', !modelValue)"
    >
      <div 
        class="absolute top-1 left-1 w-5 h-5 bg-white rounded-full shadow-md transition-all duration-300 cubic-bezier(0.4, 0.0, 0.2, 1)"
        :class="modelValue ? 'translate-x-5' : 'translate-x-0'"
      >
        <!-- Optional Icon inside the knob -->
        <div class="flex items-center justify-center w-full h-full opacity-0 transition-opacity duration-200" :class="{ 'opacity-100': modelValue }">
          <span class="material-symbols-rounded text-[10px] text-primary font-bold">check</span>
        </div>
      </div>
      
      <!-- Glow effect behind the knob when active -->
      <div 
        class="absolute top-1 left-1 w-5 h-5 rounded-full bg-white blur-md transition-all duration-300 opacity-0"
        :class="modelValue ? 'translate-x-5 opacity-50' : 'translate-x-0'"
      ></div>
    </button>
  </div>
</template>

<style scoped>
.cubic-bezier {
  transition-timing-function: cubic-bezier(0.34, 1.56, 0.64, 1);
}
</style>
