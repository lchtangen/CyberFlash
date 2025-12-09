<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useNotificationStore } from '../../../stores/notifications';
import { useAIStore } from '../../../stores/ai';
import { useSettingsStore } from '../../../stores/settings';

interface ContextHelp {
  title: string;
  explanation: string;
  suggestion: string;
  confidence: number;
}

const notificationStore = useNotificationStore();
const aiStore = useAIStore();
const settingsStore = useSettingsStore();

const isOpen = ref(false);
const isAnalyzing = ref(false);
const analysisResult = ref<ContextHelp | null>(null);
const lastErrorId = ref<string | null>(null);

// Watch for new errors
watch(() => notificationStore.history, (newHistory) => {
  if (newHistory.length > 0) {
    const latest = newHistory[0];
    if (latest.type === 'error' && latest.id !== lastErrorId.value) {
      lastErrorId.value = latest.id;
      // Auto-open or pulse? Let's pulse for now, user clicks to open.
      // But if it's a critical error, maybe we auto-analyze?
      // Let's just reset the analysis so the user sees the new error ready to be analyzed.
      analysisResult.value = null;
    }
  }
}, { deep: true });

const latestError = computed(() => {
  return notificationStore.history.find(n => n.type === 'error');
});

const analyzeError = async () => {
  if (!latestError.value) return;
  
  isAnalyzing.value = true;
  try {
    analysisResult.value = await invoke<ContextHelp>('analyze_error_context', {
      errorMessage: latestError.value.message,
      currentView: 'FlashView',
      apiKey: settingsStore.effectiveGeminiApiKey
    });
  } catch (e) {
    console.error(e);
  } finally {
    isAnalyzing.value = false;
  }
};

const askGemini = () => {
  if (!latestError.value) return;
  // Pre-fill the AI chat
  aiStore.addMessage('user', `I encountered this error: "${latestError.value.message}". Can you help me fix it?`);
  // Open the main AI overlay (assuming there's a global bus or store action for this)
  // For now, we'll just show a toast that we sent it.
  notificationStore.addNotification({
    title: 'Sent to AI',
    message: 'Check the AI Assistant for a detailed response.',
    type: 'info'
  });
};
</script>

<template>
  <div class="fixed bottom-6 right-6 z-50 flex flex-col items-end gap-4">
    
    <!-- Expanded Card -->
    <div v-if="isOpen" class="bg-surface/90 backdrop-blur-xl border border-white/10 rounded-2xl p-5 w-80 shadow-2xl animate-in slide-in-from-bottom-4 fade-in duration-300">
      <div class="flex justify-between items-start mb-4">
        <div class="flex items-center gap-2">
          <div class="p-1.5 bg-primary/20 rounded-lg">
            <span class="i-lucide-bot text-primary"></span>
          </div>
          <h3 class="font-bold text-white text-sm">Context Bot</h3>
        </div>
        <button @click="isOpen = false" class="text-white/40 hover:text-white transition-colors">
          <span class="i-lucide-x text-sm"></span>
        </button>
      </div>

      <div v-if="!latestError" class="text-center py-8 text-white/40 text-sm">
        <span class="i-lucide-check-circle text-2xl mb-2 block mx-auto text-success/50"></span>
        No recent errors detected.
      </div>

      <div v-else>
        <!-- Error Preview -->
        <div class="bg-red-500/10 border border-red-500/20 rounded-lg p-3 mb-4">
          <div class="text-[10px] font-bold text-red-400 uppercase mb-1">Last Error</div>
          <p class="text-xs text-white/90 line-clamp-3 font-mono">{{ latestError.message }}</p>
        </div>

        <!-- Analysis Result -->
        <div v-if="analysisResult" class="space-y-3">
          <div class="bg-white/5 rounded-lg p-3 border border-white/5">
            <h4 class="font-bold text-white text-sm mb-1">{{ analysisResult.title }}</h4>
            <p class="text-xs text-white/70 mb-2">{{ analysisResult.explanation }}</p>
            
            <div class="bg-black/20 rounded p-2">
              <div class="text-[10px] font-bold text-primary uppercase mb-1">Suggestion</div>
              <p class="text-xs text-white/90 whitespace-pre-wrap">{{ analysisResult.suggestion }}</p>
            </div>
          </div>
          
          <div class="flex gap-2">
             <button @click="askGemini" class="flex-1 py-2 bg-white/5 hover:bg-white/10 rounded-lg text-xs font-medium text-white transition-colors border border-white/5">
              Ask Gemini
            </button>
          </div>
        </div>

        <!-- Action Button -->
        <button 
          v-else 
          @click="analyzeError" 
          :disabled="isAnalyzing"
          class="w-full py-2 bg-primary hover:bg-primary-hover rounded-lg text-xs font-bold text-white shadow-lg shadow-primary/20 transition-all flex items-center justify-center gap-2"
        >
          <span v-if="isAnalyzing" class="i-lucide-loader-2 animate-spin"></span>
          <span v-else class="i-lucide-sparkles"></span>
          Analyze Error
        </button>
      </div>
    </div>

    <!-- FAB -->
    <button 
      @click="isOpen = !isOpen"
      class="w-12 h-12 rounded-full flex items-center justify-center shadow-lg transition-all duration-300 hover:scale-110 active:scale-95 relative group"
      :class="latestError && !isOpen ? 'bg-red-500 hover:bg-red-400 animate-pulse' : 'bg-surface border border-white/10 hover:bg-white/10'"
    >
      <span v-if="latestError && !isOpen" class="absolute -top-1 -right-1 w-3 h-3 bg-red-500 border-2 border-black rounded-full z-10"></span>
      <span class="i-lucide-life-buoy text-xl" :class="latestError && !isOpen ? 'text-white' : 'text-white/80'"></span>
      
      <!-- Tooltip -->
      <div class="absolute right-full mr-3 top-1/2 -translate-y-1/2 px-2 py-1 bg-black/80 backdrop-blur rounded text-xs text-white opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap pointer-events-none">
        {{ latestError ? 'Error Detected' : 'Help' }}
      </div>
    </button>
  </div>
</template>
