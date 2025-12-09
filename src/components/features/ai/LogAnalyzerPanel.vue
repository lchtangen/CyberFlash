<script setup lang="ts">
import { ref, onUnmounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import GlassCard from '../../ui/GlassCard.vue';
import GlassInput from '../../ui/GlassInput.vue';
import VisionButton from '../../ui/VisionButton.vue';
const logs = ref<any[]>([]);
const searchQuery = ref('');
const isListening = ref(false);
const unlisten = ref<() => void>();
const selectedLog = ref<any>(null);
const aiAnalysis = ref('');
const analyzing = ref(false);

const filteredLogs = computed(() => {
  if (!searchQuery.value) return logs.value;
  return logs.value.filter(log => 
    log.message.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
    log.tag.toLowerCase().includes(searchQuery.value.toLowerCase())
  );
});

const startLogcat = async () => {
  if (isListening.value) return;
  isListening.value = true;
  logs.value = [];
  
  try {
    await invoke('start_log_sentinel');
    unlisten.value = await listen('log-sentinel-update', (event: any) => {
      const newEntries = event.payload;
      // Append new entries and keep last 500
      logs.value = [...logs.value, ...newEntries].slice(-500);
    });
  } catch (e) {
    console.error("Failed to start sentinel:", e);
    isListening.value = false;
  }
};

const stopLogcat = async () => {
  isListening.value = false;
  try {
    await invoke('stop_log_sentinel');
    if (unlisten.value) {
      unlisten.value();
      unlisten.value = undefined;
    }
  } catch (e) {
    console.error(e);
  }
};

const analyzeWithAI = async (log: any) => {
  selectedLog.value = log;
  analyzing.value = true;
  aiAnalysis.value = '';
  
  try {
    const result = await invoke<string>('analyze_log_with_ai', { 
      logEntry: `${log.timestamp} ${log.level} ${log.tag}: ${log.message}`
    });
    aiAnalysis.value = result;
  } catch (e) {
    aiAnalysis.value = "Analysis failed: " + e;
  } finally {
    analyzing.value = false;
  }
};

onUnmounted(() => {
  stopLogcat();
});
</script>

<template>
  <div class="h-full flex flex-col space-y-4">
    <!-- Controls -->
    <div class="flex justify-between items-center">
      <div class="flex space-x-2">
        <VisionButton 
          @click="isListening ? stopLogcat() : startLogcat()" 
          :variant="isListening ? 'danger' : 'primary'"
        >
          {{ isListening ? 'Stop Sentinel' : 'Start Sentinel' }}
        </VisionButton>
        <VisionButton @click="logs = []" variant="secondary">Clear</VisionButton>
      </div>
      <div class="w-64">
        <GlassInput v-model="searchQuery" placeholder="Filter logs..." icon="search" />
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex space-x-4 min-h-0">
      <!-- Log List -->
      <GlassCard class="flex-1 flex flex-col overflow-hidden">
        <div class="flex-1 overflow-y-auto custom-scrollbar p-2 space-y-1">
          <div 
            v-for="(log, index) in filteredLogs" 
            :key="index"
            class="text-xs font-mono p-2 rounded hover:bg-white/5 cursor-pointer transition-colors border-l-2"
            :class="{
              'border-error bg-error/5': log.level === 'Error' || log.level === 'Fatal',
              'border-warning bg-warning/5': log.level === 'Warning',
              'border-transparent': log.level !== 'Error' && log.level !== 'Warning'
            }"
            @click="selectedLog = log"
          >
            <span class="text-white/40 mr-2">{{ log.timestamp }}</span>
            <span :class="{
              'text-error': log.level === 'Error' || log.level === 'Fatal',
              'text-warning': log.level === 'Warning',
              'text-info': log.level === 'Info',
              'text-white/60': log.level === 'Debug'
            }" class="font-bold mr-2">{{ log.level }}</span>
            <span class="text-primary mr-2">{{ log.tag }}:</span>
            <span class="text-white/80">{{ log.message }}</span>
            
            <div v-if="log.analysis" class="mt-1 ml-4 text-warning italic">
              🤖 {{ log.analysis }}
            </div>
          </div>
          
          <div v-if="logs.length === 0" class="text-center text-white/40 mt-10">
            No logs captured yet. Start the Sentinel to monitor device.
          </div>
        </div>
      </GlassCard>

      <!-- Analysis Panel -->
      <GlassCard v-if="selectedLog" class="w-1/3 flex flex-col overflow-hidden">
        <div class="p-4 border-b border-white/10">
          <h3 class="font-bold text-white">Log Details</h3>
        </div>
        <div class="p-4 flex-1 overflow-y-auto">
          <div class="mb-4">
            <span class="text-xs text-white/40 block">Tag</span>
            <span class="text-sm text-white font-mono">{{ selectedLog.tag }}</span>
          </div>
          <div class="mb-4">
            <span class="text-xs text-white/40 block">Message</span>
            <p class="text-sm text-white font-mono break-words">{{ selectedLog.message }}</p>
          </div>
          
          <div class="mt-6">
            <VisionButton 
              @click="analyzeWithAI(selectedLog)" 
              variant="primary" 
              class="w-full"
              :disabled="analyzing"
            >
              <span v-if="analyzing" class="animate-spin mr-2">🤖</span>
              <span v-else class="mr-2">✨</span>
              Ask Gemini AI
            </VisionButton>
          </div>
          
          <div v-if="aiAnalysis" class="mt-4 p-3 rounded-xl bg-primary/10 border border-primary/20">
            <h4 class="text-sm font-bold text-primary mb-2">AI Analysis</h4>
            <p class="text-sm text-white/80 whitespace-pre-wrap">{{ aiAnalysis }}</p>
          </div>
        </div>
      </GlassCard>
    </div>
  </div>
</template>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: rgba(255, 255, 255, 0.05);
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.2);
  border-radius: 2px;
}
</style>
