<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import GlassCard from '../../ui/GlassCard.vue';

const logs = ref<{ id: number; text: string; type: 'info' | 'success' | 'warning' | 'error' | 'ai' }[]>([]);
const isThinking = ref(false);

const addLog = (text: string, type: 'info' | 'success' | 'warning' | 'error' | 'ai' = 'info') => {
  logs.value.push({ id: Date.now(), text, type });
  if (logs.value.length > 20) logs.value.shift();
};

let interval: any;

onMounted(() => {
  addLog('Neural Core Initialized...', 'info');
  addLog('Connecting to local LLM (Gemini 3 Pro)...', 'info');
  
  setTimeout(() => {
    addLog('Connection Established.', 'success');
    isThinking.value = true;
  }, 1000);

  interval = setInterval(() => {
    if (Math.random() > 0.7) {
      const messages = [
        'Analyzing partition table...',
        'Optimizing I/O buffers...',
        'Detecting thermal throttling patterns...',
        'Verifying cryptographic signatures...',
        'Predicting flash duration...'
      ];
      addLog(messages[Math.floor(Math.random() * messages.length)], 'ai');
    }
  }, 3000);
});

onUnmounted(() => clearInterval(interval));
</script>

<template>
  <GlassCard class="h-full flex flex-col overflow-hidden font-mono text-xs relative group">
    <!-- Header -->
    <div class="flex items-center justify-between mb-4 border-b border-white/10 pb-2">
      <div class="flex items-center gap-2">
        <div class="w-2 h-2 rounded-full bg-primary animate-pulse"></div>
        <span class="text-primary font-bold tracking-wider">NEURAL_TERMINAL_V2</span>
      </div>
      <div class="flex gap-1">
        <div class="w-2 h-2 rounded-full bg-white/20"></div>
        <div class="w-2 h-2 rounded-full bg-white/20"></div>
      </div>
    </div>

    <!-- Content -->
    <div class="flex-1 overflow-y-auto space-y-1 custom-scrollbar relative z-10">
      <div v-for="log in logs" :key="log.id" class="flex gap-2 animate-fade-in">
        <span class="text-white/30">[{{ new Date(log.id).toLocaleTimeString().split(' ')[0] }}]</span>
        <span :class="{
          'text-white/80': log.type === 'info',
          'text-success': log.type === 'success',
          'text-warning': log.type === 'warning',
          'text-error': log.type === 'error',
          'text-purple-400 font-bold': log.type === 'ai'
        }">
          <span v-if="log.type === 'ai'" class="mr-1">✦</span>
          {{ log.text }}
        </span>
      </div>
      
      <!-- Typing Cursor -->
      <div class="flex items-center gap-2 mt-2 opacity-50">
        <span class="text-primary">></span>
        <span class="w-2 h-4 bg-primary animate-blink"></span>
      </div>
    </div>

    <!-- Background Effects -->
    <div class="absolute inset-0 bg-gradient-to-b from-transparent to-primary/5 pointer-events-none"></div>
    <div class="absolute top-0 left-0 w-full h-px bg-gradient-to-r from-transparent via-primary/50 to-transparent animate-scan"></div>

  </GlassCard>
</template>

<style scoped>
.animate-blink {
  animation: blink 1s step-end infinite;
}
@keyframes blink {
  0%, 100% { opacity: 1; }
  50% { opacity: 0; }
}
.animate-scan {
  animation: scan 4s linear infinite;
}
@keyframes scan {
  0% { transform: translateY(-100%); opacity: 0; }
  10% { opacity: 1; }
  90% { opacity: 1; }
  100% { transform: translateY(400px); opacity: 0; }
}
</style>
