<script setup lang="ts">
defineProps<{
  modelValue: string | number;
  label?: string;
  placeholder?: string;
  type?: string;
  icon?: string;
  error?: string;
  disabled?: boolean;
}>();

defineEmits<{
  (e: 'update:modelValue', value: string | number): void;
}>();
</script>

<template>
  <div class="flex flex-col gap-1.5">
    <label v-if="label" class="text-xs font-medium text-text-secondary ml-1">
      {{ label }}
    </label>
    
    <div class="relative group">
      <!-- Icon -->
      <div v-if="icon" class="absolute left-3 top-1/2 -translate-y-1/2 text-text-muted transition-colors group-focus-within:text-primary">
        <span class="material-symbols-rounded text-lg">{{ icon }}</span>
      </div>

      <input
        :value="modelValue"
        @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
        :type="type || 'text'"
        :placeholder="placeholder"
        :disabled="disabled"
        class="w-full bg-surface/20 border border-white/10 rounded-xl text-sm text-white placeholder-text-muted transition-all duration-200
               focus:outline-none focus:border-primary/50 focus:bg-surface/30 focus:ring-1 focus:ring-primary/20
               disabled:opacity-50 disabled:cursor-not-allowed"
        :class="[
          icon ? 'pl-10 pr-4 py-2.5' : 'px-4 py-2.5',
          error ? 'border-error/50 focus:border-error focus:ring-error/20' : ''
        ]"
      />
    </div>
    
    <span v-if="error" class="text-xs text-error ml-1">{{ error }}</span>
  </div>
</template>
