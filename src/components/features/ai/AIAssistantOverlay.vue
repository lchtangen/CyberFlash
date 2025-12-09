<script setup lang="ts">
import { ref } from 'vue';
import { useAIStore } from '../../../stores/ai';
import AIChatInterface from './AIChatInterface.vue';
import ContextAnalysisPanel from './ContextAnalysisPanel.vue';
import LogAnalyzerPanel from './LogAnalyzerPanel.vue';
import FlashGuidePanel from './FlashGuidePanel.vue';
import AISettingsPanel from './AISettingsPanel.vue';

const aiStore = useAIStore();
const isMenuOpen = ref(false);
const activeFeature = ref<string | null>(null);

const toggleMenu = () => {
  isMenuOpen.value = !isMenuOpen.value;
  if (!isMenuOpen.value) {
    // Optional: Close window when menu closes? 
    // No, let's keep window open if user just wants to hide the FAB stack.
    // But for simplicity, let's say closing the menu doesn't close the window.
  }
};

const selectFeature = (feature: string) => {
  activeFeature.value = feature;
  aiStore.isVisible = true; // Ensure visibility is tracked if needed, though we use activeFeature now
  // isMenuOpen.value = false; // Auto-close menu? Maybe keep it open for quick switching.
};

const closeWindow = () => {
  activeFeature.value = null;
  aiStore.isVisible = false;
};

const menuItems = [
  { id: 'settings', icon: 'settings', label: 'AI Settings', color: 'text-gray-400', delay: '0ms' },
  { id: 'guide', icon: 'menu_book', label: 'Flash Guide', color: 'text-yellow-400', delay: '50ms' },
  { id: 'logs', icon: 'analytics', label: 'Log Analyzer', color: 'text-blue-400', delay: '100ms' },
  { id: 'context', icon: 'bug_report', label: 'Error Doctor', color: 'text-red-400', delay: '150ms' },
  { id: 'chat', icon: 'smart_toy', label: 'AI Assistant', color: 'text-primary', delay: '200ms' },
];
</script>

<template>
  <div class="fixed bottom-8 right-8 z-50 flex flex-col items-end gap-4 pointer-events-none">
    
    <!-- Content Window -->
    <Transition name="pop-out">
      <div v-if="activeFeature" class="absolute bottom-24 right-0 w-[400px] origin-bottom-right pointer-events-auto z-50">
        <div class="relative shadow-2xl rounded-2xl overflow-hidden ring-1 ring-white/10 h-[600px]">
           
           <!-- 1. Chat Interface -->
           <AIChatInterface 
             v-if="activeFeature === 'chat'" 
             heightClass="h-full" 
             @close="closeWindow" 
           />

           <!-- 2. Context Analysis -->
           <ContextAnalysisPanel 
             v-else-if="activeFeature === 'context'" 
             @close="closeWindow"
             @switch-to-chat="selectFeature('chat')"
           />

           <!-- 3. Log Analyzer -->
           <LogAnalyzerPanel 
             v-else-if="activeFeature === 'logs'" 
             @close="closeWindow"
           />

           <!-- 4. Flash Guide -->
           <FlashGuidePanel 
             v-else-if="activeFeature === 'guide'" 
             @close="closeWindow"
           />

           <!-- 5. Settings -->
           <AISettingsPanel 
             v-else-if="activeFeature === 'settings'" 
             @close="closeWindow"
           />

        </div>
      </div>
    </Transition>

    <!-- Speed Dial Menu -->
    <div class="flex flex-col gap-3 items-end mb-2 pointer-events-auto">
      <TransitionGroup name="slide-up">
        <div 
          v-if="isMenuOpen"
          v-for="item in menuItems" 
          :key="item.id"
          class="flex items-center gap-3 group"
          :style="{ transitionDelay: item.delay }"
        >
          <!-- Label -->
          <div class="bg-black/80 backdrop-blur text-white text-xs px-2 py-1 rounded opacity-0 group-hover:opacity-100 transition-opacity duration-200 shadow-lg">
            {{ item.label }}
          </div>
          
          <!-- Button -->
          <button 
            @click="selectFeature(item.id)"
            class="relative w-12 h-12 rounded-full bg-surface/30 backdrop-blur-xl border border-white/10 shadow-2xl shadow-black/50 flex items-center justify-center transition-all duration-200 hover:scale-110 hover:bg-surface/50 active:scale-95 group"
            :class="activeFeature === item.id ? 'bg-primary/20 border-primary/30' : ''"
          >
            <span class="material-symbols-rounded text-xl relative z-10" :class="item.color">{{ item.icon }}</span>
            
            <!-- Glow Effect -->
            <div class="absolute inset-0 rounded-full bg-primary/20 blur-md opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
          </button>
        </div>
      </TransitionGroup>
    </div>

    <!-- Main Floating Button -->
    <button 
      @click="toggleMenu"
      class="group relative w-16 h-16 rounded-full bg-surface/30 backdrop-blur-xl border border-white/10 shadow-2xl shadow-black/50 flex items-center justify-center transition-all duration-300 hover:scale-110 hover:bg-surface/50 active:scale-95 pointer-events-auto z-50"
      :class="{ 'bg-primary/20 border-primary/30': isMenuOpen || activeFeature }"
    >
      <!-- Icon Switch Transition -->
      <div class="relative w-8 h-8 flex items-center justify-center">
        <span 
          class="material-symbols-rounded text-3xl text-white absolute transition-all duration-500 ease-out"
          :class="isMenuOpen ? 'rotate-90 opacity-0 scale-50' : 'rotate-0 opacity-100 scale-100'"
        >
          smart_toy
        </span>
        <span 
          class="material-symbols-rounded text-3xl text-white absolute transition-all duration-500 ease-out"
          :class="isMenuOpen ? 'rotate-0 opacity-100 scale-100' : '-rotate-90 opacity-0 scale-50'"
        >
          close
        </span>
      </div>
      
      <!-- Glow Effect -->
      <div class="absolute inset-0 rounded-full bg-primary/20 blur-xl opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
      
      <!-- Pulse Ring (only when idle) -->
      <div v-if="!isMenuOpen && !activeFeature" class="absolute inset-0 rounded-full border border-white/20 animate-ping opacity-20"></div>
    </button>
  </div>
</template>

<style scoped>
.pop-out-enter-active,
.pop-out-leave-active {
  transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.pop-out-enter-from,
.pop-out-leave-to {
  opacity: 0;
  transform: scale(0.8) translate(20px, 20px);
  filter: blur(10px);
}

.slide-up-enter-active,
.slide-up-leave-active {
  transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.slide-up-enter-from,
.slide-up-leave-to {
  opacity: 0;
  transform: translateY(20px) scale(0.5);
}
</style>
