<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useNotificationStore } from '../../../stores/notifications';

interface ParsedCommand {
  original: string;
  commands: string[];
  confidence: number;
  explanation: string;
}

const notificationStore = useNotificationStore();
const query = ref('');
const isProcessing = ref(false);
const result = ref<ParsedCommand | null>(null);
const executionOutput = ref<string | null>(null);
const isExecuting = ref(false);

const parseQuery = async () => {
  if (!query.value.trim()) return;
  
  isProcessing.value = true;
  result.value = null;
  executionOutput.value = null;

  try {
    result.value = await invoke<ParsedCommand>('parse_natural_language', { query: query.value });
  } catch (e) {
    notificationStore.addNotification({
      title: 'Parsing Failed',
      message: String(e),
      type: 'error'
    });
  } finally {
    isProcessing.value = false;
  }
};

const executeCommand = async (cmd: string) => {
  isExecuting.value = true;
  executionOutput.value = null;
  
  try {
    const output = await invoke<string>('execute_cli_command', { command: cmd });
    executionOutput.value = output || 'Command executed successfully (no output).';
    notificationStore.addNotification({
      title: 'Success',
      message: 'Command executed.',
      type: 'success'
    });
  } catch (e) {
    executionOutput.value = `Error: ${e}`;
    notificationStore.addNotification({
      title: 'Execution Failed',
      message: String(e),
      type: 'error'
    });
  } finally {
    isExecuting.value = false;
  }
};
</script>

<template>
  <div class="bg-surface/30 border border-white/10 rounded-2xl p-6 backdrop-blur-xl">
    <div class="flex items-center gap-3 mb-4">
      <div class="p-2 bg-primary/20 rounded-lg">
        <span class="i-lucide-sparkles text-primary text-xl"></span>
      </div>
      <div>
        <h3 class="text-lg font-bold text-white">Natural Language CLI</h3>
        <p class="text-xs text-white/60">Type what you want to do in plain English</p>
      </div>
    </div>

    <div class="relative">
      <input 
        v-model="query"
        @keydown.enter="parseQuery"
        type="text" 
        placeholder="e.g., 'Reboot to recovery' or 'List all packages'"
        class="w-full bg-black/40 border border-white/10 rounded-xl px-4 py-3 text-white placeholder-white/30 focus:outline-none focus:border-primary/50 focus:ring-1 focus:ring-primary/50 transition-all pr-12"
      />
      <button 
        @click="parseQuery"
        :disabled="isProcessing || !query"
        class="absolute right-2 top-1/2 -translate-y-1/2 p-2 rounded-lg hover:bg-white/10 text-white/60 hover:text-white transition-colors disabled:opacity-50"
      >
        <span v-if="isProcessing" class="i-lucide-loader-2 animate-spin"></span>
        <span v-else class="i-lucide-arrow-right"></span>
      </button>
    </div>

    <!-- Results Area -->
    <div v-if="result" class="mt-4 space-y-4 animate-in fade-in slide-in-from-top-2 duration-300">
      
      <!-- Explanation Card -->
      <div class="bg-white/5 rounded-xl p-4 border border-white/5">
        <div class="flex justify-between items-start mb-2">
          <span class="text-xs font-bold text-white/40 uppercase">Interpretation</span>
          <span class="text-xs font-mono px-2 py-0.5 rounded bg-white/10" :class="result.confidence > 0.8 ? 'text-success' : 'text-warning'">
            {{ Math.round(result.confidence * 100) }}% Confidence
          </span>
        </div>
        <p class="text-white/90 text-sm">{{ result.explanation }}</p>
      </div>

      <!-- Commands List -->
      <div v-if="result.commands.length > 0" class="space-y-2">
        <div v-for="(cmd, index) in result.commands" :key="index" class="flex items-center gap-2 group">
          <code class="flex-1 bg-black/50 border border-white/10 rounded-lg px-3 py-2 text-sm font-mono text-primary">
            $ {{ cmd }}
          </code>
          <button 
            @click="executeCommand(cmd)"
            :disabled="isExecuting"
            class="p-2 bg-white/5 hover:bg-primary hover:text-white rounded-lg text-white/60 transition-all"
            title="Run Command"
          >
            <span class="i-lucide-play text-sm"></span>
          </button>
        </div>
      </div>

      <!-- Execution Output -->
      <div v-if="executionOutput" class="bg-black/80 rounded-xl p-4 border border-white/10 font-mono text-xs text-white/80 whitespace-pre-wrap max-h-40 overflow-y-auto custom-scrollbar">
        {{ executionOutput }}
      </div>
    </div>
  </div>
</template>
