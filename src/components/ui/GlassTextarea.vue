<script setup lang="ts">
defineProps<{
  modelValue: string;
  label?: string;
  placeholder?: string;
  rows?: number;
  disabled?: boolean;
}>();

defineEmits<{
  (e: 'update:modelValue', value: string): void;
}>();
</script>

<template>
  <div class="flex flex-col gap-1.5">
    <label v-if="label" class="text-xs font-medium text-text-secondary ml-1">
      {{ label }}
    </label>
    
    <textarea
      :value="modelValue"
      @input="$emit('update:modelValue', ($event.target as HTMLTextAreaElement).value)"
      :placeholder="placeholder"
      :rows="rows || 4"
      :disabled="disabled"
      class="w-full bg-surface/20 border border-white/10 rounded-xl text-sm text-white placeholder-text-muted transition-all duration-200
             focus:outline-none focus:border-primary/50 focus:bg-surface/30 focus:ring-1 focus:ring-primary/20
             disabled:opacity-50 disabled:cursor-not-allowed resize-none custom-scrollbar p-3 font-mono"
    ></textarea>
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
