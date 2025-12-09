<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';

const props = defineProps<{
  modelValue: string | number;
  options: Array<{ label: string; value: string | number }>;
  placeholder?: string;
  label?: string;
  icon?: string;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: string | number): void;
}>();

const isOpen = ref(false);
const containerRef = ref<HTMLElement | null>(null);

const selectedLabel = computed(() => {
  const option = props.options.find(o => o.value === props.modelValue);
  return option ? option.label : props.placeholder || 'Select...';
});

const toggle = () => isOpen.value = !isOpen.value;

const select = (value: string | number) => {
  emit('update:modelValue', value);
  isOpen.value = false;
};

const handleClickOutside = (event: MouseEvent) => {
  if (containerRef.value && !containerRef.value.contains(event.target as Node)) {
    isOpen.value = false;
  }
};

onMounted(() => document.addEventListener('click', handleClickOutside));
onUnmounted(() => document.removeEventListener('click', handleClickOutside));
</script>

<template>
  <div class="relative" ref="containerRef">
    <label v-if="label" class="block text-xs font-bold text-text-secondary uppercase tracking-wider mb-2 ml-1">
      {{ label }}
    </label>
    
    <button 
      @click="toggle"
      class="w-full flex items-center justify-between px-4 py-3 rounded-xl bg-black/20 border border-white/10 text-white text-sm hover:bg-white/5 hover:border-white/20 transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-primary/50"
      :class="{ 'ring-2 ring-primary/50 border-primary/50': isOpen }"
    >
      <div class="flex items-center gap-2">
        <span v-if="icon" class="material-symbols-rounded text-lg text-text-secondary">{{ icon }}</span>
        <span :class="{ 'text-text-secondary': !modelValue && placeholder }">{{ selectedLabel }}</span>
      </div>
      <span class="material-symbols-rounded text-text-secondary transition-transform duration-300" :class="{ 'rotate-180': isOpen }">expand_more</span>
    </button>

    <transition
      enter-active-class="transition duration-200 ease-out"
      enter-from-class="transform scale-95 opacity-0"
      enter-to-class="transform scale-100 opacity-100"
      leave-active-class="transition duration-150 ease-in"
      leave-from-class="transform scale-100 opacity-100"
      leave-to-class="transform scale-95 opacity-0"
    >
      <div 
        v-if="isOpen" 
        class="absolute z-50 w-full mt-2 overflow-hidden rounded-xl border border-white/10 bg-surface/90 backdrop-blur-xl shadow-2xl shadow-black/50 max-h-60 overflow-y-auto custom-scrollbar"
      >
        <div class="p-1">
          <button
            v-for="option in options"
            :key="option.value"
            @click="select(option.value)"
            class="w-full flex items-center justify-between px-3 py-2 rounded-lg text-sm text-left transition-colors"
            :class="[
              modelValue === option.value 
                ? 'bg-primary/20 text-primary font-medium' 
                : 'text-white hover:bg-white/10'
            ]"
          >
            <span>{{ option.label }}</span>
            <span v-if="modelValue === option.value" class="material-symbols-rounded text-lg">check</span>
          </button>
        </div>
      </div>
    </transition>
  </div>
</template>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 10px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.2);
}
</style>
