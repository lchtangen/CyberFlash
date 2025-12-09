<template>
  <div class="relative overflow-hidden rounded-2xl border border-white/10 bg-surface/30 backdrop-blur-xl transition-all duration-300 hover:bg-surface/40 p-6">
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-semibold text-white flex items-center gap-2">
        <span class="i-lucide-shield-alert text-primary"></span>
        Log Sentinel
      </h3>
      <div class="flex items-center gap-2">
        <div v-if="isMonitoring" class="flex items-center gap-2 px-2 py-1 rounded-full bg-success/10 border border-success/20">
          <span class="w-2 h-2 rounded-full bg-success animate-pulse"></span>
          <span class="text-xs font-medium text-success">Active</span>
        </div>
        <button 
          @click="toggleMonitoring"
          class="p-2 rounded-lg hover:bg-white/10 transition-colors"
          :title="isMonitoring ? 'Stop Monitoring' : 'Start Monitoring'"
        >
          <span :class="isMonitoring ? 'i-lucide-square text-error' : 'i-lucide-play text-success'"></span>
        </button>
      </div>
    </div>

    <!-- Log Stream Window -->
    <div class="h-48 overflow-y-auto custom-scrollbar bg-black/40 rounded-xl border border-white/5 p-3 font-mono text-xs space-y-1 mb-4" ref="logContainer">
      <div v-if="logs.length === 0" class="h-full flex items-center justify-center text-white/30 italic">
        Waiting for device logs...
      </div>
      <div v-for="(log, index) in logs" :key="index" class="flex gap-2 animate-in fade-in slide-in-from-bottom-1 duration-200">
        <span class="text-white/40 shrink-0">{{ log.timestamp }}</span>
        <span 
          class="font-bold shrink-0 w-4 text-center"
          :class="{
            'text-error': log.level === 'Error' || log.level === 'Fatal',
            'text-warning': log.level === 'Warning',
            'text-primary': log.level === 'Info'
          }"
        >{{ log.level[0] }}</span>
        <span class="text-white/60 shrink-0 w-24 truncate" :title="log.tag">{{ log.tag }}</span>
        <span class="text-white/90 break-all">{{ log.message }}</span>
      </div>
    </div>

    <!-- AI Analysis Alert -->
    <div v-if="latestAnalysis" class="bg-error/10 border border-error/20 rounded-xl p-4 animate-in zoom-in-95 duration-300">
      <div class="flex items-start gap-3">
        <div class="p-2 bg-error/20 rounded-lg">
          <span class="i-lucide-alert-octagon text-error text-xl"></span>
        </div>
        <div>
          <h4 class="text-sm font-bold text-white flex items-center gap-2">
            {{ latestAnalysis.summary }}
            <span class="text-[10px] px-1.5 py-0.5 rounded bg-error text-white uppercase">{{ latestAnalysis.severity }}</span>
          </h4>
          <p class="text-xs text-white/70 mt-1">{{ latestAnalysis.possible_fix }}</p>
        </div>
        <button @click="latestAnalysis = null" class="ml-auto text-white/40 hover:text-white">
          <span class="i-lucide-x"></span>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

interface LogEntry {
  timestamp: string;
  level: string;
  tag: string;
  message: string;
  analysis?: string;
}

interface LogAnalysis {
  severity: string;
  summary: string;
  possible_fix: string;
}

const isMonitoring = ref(false);
const logs = ref<LogEntry[]>([]);
const latestAnalysis = ref<LogAnalysis | null>(null);
const logContainer = ref<HTMLElement | null>(null);
let unlisten: (() => void) | null = null;

async function toggleMonitoring() {
  if (isMonitoring.value) {
    await invoke('stop_log_sentinel');
    isMonitoring.value = false;
  } else {
    await invoke('start_log_sentinel');
    isMonitoring.value = true;
  }
}

function parseAnalysis(analysisStr: string): LogAnalysis {
  // Format: "Summary - Possible Fix"
  const [summary, fix] = analysisStr.split(' - ');
  return {
    severity: 'High', // Default, backend should send struct but sending string for simplicity in demo
    summary: summary || 'Unknown Error',
    possible_fix: fix || 'Check logs for details.'
  };
}

onMounted(async () => {
  unlisten = await listen<LogEntry[]>('log-sentinel-update', (event) => {
    const newLogs = event.payload;
    logs.value.push(...newLogs);
    
    // Keep only last 100 logs
    if (logs.value.length > 100) {
      logs.value = logs.value.slice(-100);
    }

    // Check for analysis
    const criticalLog = newLogs.find(l => l.analysis);
    if (criticalLog && criticalLog.analysis) {
      latestAnalysis.value = parseAnalysis(criticalLog.analysis);
    }

    // Auto-scroll
    nextTick(() => {
      if (logContainer.value) {
        logContainer.value.scrollTop = logContainer.value.scrollHeight;
      }
    });
  });
});

onUnmounted(async () => {
  if (unlisten) unlisten();
  if (isMonitoring.value) {
    await invoke('stop_log_sentinel');
  }
});
</script>

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
