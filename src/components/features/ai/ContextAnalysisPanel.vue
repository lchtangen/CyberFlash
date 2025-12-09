<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useNotificationStore } from '../../../stores/notifications';
import { useAIStore } from '../../../stores/ai';
import { useSettingsStore } from '../../../stores/settings';
import GlassCard from '../../ui/GlassCard.vue';

interface ContextHelp {
  title: string;
  explanation: string;
  suggestion: string;
  confidence: number;
}

const emit = defineEmits<{
  (e: 'close'): void;
  (e: 'switch-to-chat'): void;
}>();

const notificationStore = useNotificationStore();
const aiStore = useAIStore();
const settingsStore = useSettingsStore();

const isAnalyzing = ref(false);
const analysisResult = ref<ContextHelp | null>(null);
const lastErrorId = ref<string | null>(null);

// Watch for new errors
watch(() => notificationStore.history, (newHistory) => {
  if (newHistory.length > 0) {
    const latest = newHistory[0];
    if (latest.type === 'error' && latest.id !== lastErrorId.value) {
      lastErrorId.value = latest.id;
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
  aiStore.addMessage('user', `I encountered this error: "${latestError.value.message}". Can you help me fix it?`);
  emit('switch-to-chat');
};
</script>

<template>
  <GlassCard noPadding class="w-full h-full flex flex-col overflow-hidden border-white/10 shadow-2xl shadow-black/50">
    <!-- Header -->
    <div class="px-5 py-4 border-b border-white/5 flex items-center justify-between bg-white/5 backdrop-blur-md z-10">
      <div class="flex items-center gap-4">
        <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-red-500/20 to-red-500/5 flex items-center justify-center border border-white/10 shadow-inner">
          <span class="material-symbols-rounded text-red-400 text-xl">bug_report</span>
        </div>
        <div>
          <h3 class="font-bold text-white text-base tracking-wide">Error Doctor</h3>
          <p class="text-xs text-text-secondary font-medium">Contextual Analysis</p>
        </div>
      </div>
      
      <button 
        @click="$emit('close')"
        class="p-2 rounded-lg hover:bg-white/10 text-text-secondary hover:text-white transition-all duration-200 group"
      >
        <span class="material-symbols-rounded text-lg group-hover:scale-110 transition-transform">close</span>
      </button>
    </div>

    <!-- Content -->
    <div class="flex-1 overflow-y-auto p-5 custom-scrollbar">
      <div v-if="!latestError" class="h-full flex flex-col items-center justify-center text-center opacity-50">
        <span class="material-symbols-rounded text-5xl mb-4 text-success">check_circle</span>
        <h4 class="text-lg font-bold text-white mb-1">All Systems Normal</h4>
        <p class="text-sm text-text-secondary">No recent errors detected in the session logs.</p>
      </div>

      <div v-else class="space-y-6">
        <!-- Error Preview -->
        <div class="bg-red-500/10 border border-red-500/20 rounded-xl p-4">
          <div class="flex items-center gap-2 mb-2">
            <span class="material-symbols-rounded text-red-400 text-sm">error</span>
            <span class="text-xs font-bold text-red-400 uppercase tracking-wider">Latest Error</span>
          </div>
          <p class="text-sm text-white/90 font-mono bg-black/20 p-3 rounded-lg border border-white/5">{{ latestError.message }}</p>
        </div>

        <!-- Analysis Result -->
        <div v-if="analysisResult" class="space-y-4 animate-fade-in">
          <div class="bg-surface/50 rounded-xl p-4 border border-white/10">
            <h4 class="font-bold text-white text-lg mb-2">{{ analysisResult.title }}</h4>
            <p class="text-sm text-text-secondary leading-relaxed mb-4">{{ analysisResult.explanation }}</p>
            
            <div class="bg-primary/10 rounded-lg p-4 border border-primary/20">
              <div class="text-xs font-bold text-primary uppercase mb-2 flex items-center gap-2">
                <span class="material-symbols-rounded text-sm">lightbulb</span>
                Suggestion
              </div>
              <p class="text-sm text-white/90 whitespace-pre-wrap">{{ analysisResult.suggestion }}</p>
            </div>
          </div>
          
          <button 
            @click="askGemini" 
            class="w-full py-3 bg-white/5 hover:bg-white/10 rounded-xl text-sm font-medium text-white transition-colors border border-white/5 flex items-center justify-center gap-2 group"
          >
            <span class="material-symbols-rounded group-hover:text-primary transition-colors">chat</span>
            Discuss with AI Assistant
          </button>
        </div>

        <!-- Action Button -->
        <button 
          v-else 
          @click="analyzeError" 
          :disabled="isAnalyzing"
          class="w-full py-4 bg-primary hover:bg-primary-hover rounded-xl text-sm font-bold text-white shadow-lg shadow-primary/20 transition-all flex items-center justify-center gap-2 active:scale-95"
        >
          <span v-if="isAnalyzing" class="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></span>
          <span v-else class="material-symbols-rounded">auto_fix_high</span>
          {{ isAnalyzing ? 'Analyzing Error...' : 'Analyze This Error' }}
        </button>
      </div>
    </div>
  </GlassCard>
</template>
