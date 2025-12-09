<script setup lang="ts">
import { useAIStore } from '../../../stores/ai';
import AIChatInterface from './AIChatInterface.vue';

const aiStore = useAIStore();

const toggleChat = () => {
  aiStore.isVisible = !aiStore.isVisible;
};
</script>

<template>
  <div class="fixed bottom-8 right-8 z-50 flex flex-col items-end gap-4 pointer-events-none">
    <!-- Chat Window -->
    <Transition name="pop-out">
      <div v-if="aiStore.isVisible" class="w-[400px] origin-bottom-right pointer-events-auto">
        <div class="relative shadow-2xl rounded-2xl overflow-hidden ring-1 ring-white/10">
           <AIChatInterface heightClass="h-[600px]" />
        </div>
      </div>
    </Transition>

    <!-- Floating Bubble Button -->
    <button 
      @click="toggleChat"
      class="group relative w-16 h-16 rounded-full bg-surface/30 backdrop-blur-xl border border-white/10 shadow-2xl shadow-black/50 flex items-center justify-center transition-all duration-300 hover:scale-110 hover:bg-surface/50 active:scale-95 pointer-events-auto"
      :class="{ 'bg-primary/20 border-primary/30': aiStore.isVisible }"
    >
      <!-- Icon Switch Transition -->
      <div class="relative w-8 h-8 flex items-center justify-center">
        <span 
          class="material-symbols-rounded text-3xl text-white absolute transition-all duration-500 ease-out"
          :class="aiStore.isVisible ? 'rotate-90 opacity-0 scale-50' : 'rotate-0 opacity-100 scale-100'"
        >
          smart_toy
        </span>
        <span 
          class="material-symbols-rounded text-3xl text-white absolute transition-all duration-500 ease-out"
          :class="aiStore.isVisible ? 'rotate-0 opacity-100 scale-100' : '-rotate-90 opacity-0 scale-50'"
        >
          close
        </span>
      </div>
      
      <!-- Glow Effect -->
      <div class="absolute inset-0 rounded-full bg-primary/20 blur-xl opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
      
      <!-- Pulse Ring (only when closed and idle) -->
      <div v-if="!aiStore.isVisible" class="absolute inset-0 rounded-full border border-white/20 animate-ping opacity-20"></div>
    </button>
  </div>
</template>

<style scoped>
.pop-out-enter-active,
.pop-out-leave-active {
  transition: all 0.5s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.pop-out-enter-from,
.pop-out-leave-to {
  opacity: 0;
  transform: scale(0.5) translate(50px, 50px);
  filter: blur(20px);
}
</style>
