<script setup lang="ts">
import { useAIStore } from '../../stores/ai';
import AIChatInterface from './AIChatInterface.vue';

const aiStore = useAIStore();
</script>

<template>
  <div v-if="aiStore.isVisible" class="fixed bottom-6 right-6 z-50 w-[400px] shadow-2xl">
    <AIChatInterface />
    <button 
      @click="aiStore.isVisible = false"
      class="absolute -top-3 -right-3 w-8 h-8 bg-surface/90 border border-white/10 rounded-full text-white flex items-center justify-center shadow-lg hover:bg-white/10 transition-colors backdrop-blur-md"
    >
      <span class="material-symbols-rounded text-sm">close</span>
    </button>
  </div>
</template>
