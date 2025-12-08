<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick } from 'vue';
import { listen } from '@tauri-apps/api/event';

const lines = ref<string[]>([]);
const terminalRef = ref<HTMLElement | null>(null);

const addLine = (text: string, type: 'stdout' | 'stderr' = 'stdout') => {
  const timestamp = new Date().toLocaleTimeString();
  lines.value.push(`[${timestamp}] ${text}`);
  if (lines.value.length > 1000) lines.value.shift();
  
  nextTick(() => {
    if (terminalRef.value) {
      terminalRef.value.scrollTop = terminalRef.value.scrollHeight;
    }
  });
};

let unlistenStdout: () => void;
let unlistenStderr: () => void;

onMounted(async () => {
  unlistenStdout = await listen<string>('shell-stdout', (event) => addLine(event.payload, 'stdout'));
  unlistenStderr = await listen<string>('shell-stderr', (event) => addLine(event.payload, 'stderr'));
  
  addLine('CyberFlash V2 Terminal Ready...', 'stdout');
});

onUnmounted(() => {
  if (unlistenStdout) unlistenStdout();
  if (unlistenStderr) unlistenStderr();
});
</script>

<template>
  <div class="bg-black/80 border border-white/10 rounded-xl overflow-hidden flex flex-col h-64 font-mono text-xs">
    <div class="bg-white/5 px-4 py-2 flex justify-between items-center border-b border-white/5">
      <span class="text-gray-400">Live Terminal</span>
      <div class="flex gap-2">
        <div class="w-3 h-3 rounded-full bg-red-500/20"></div>
        <div class="w-3 h-3 rounded-full bg-yellow-500/20"></div>
        <div class="w-3 h-3 rounded-full bg-green-500/20"></div>
      </div>
    </div>
    <div ref="terminalRef" class="flex-1 p-4 overflow-y-auto custom-scrollbar space-y-1">
      <div v-for="(line, i) in lines" :key="i" class="text-gray-300 break-all">
        <span class="text-primary mr-2">$</span>{{ line }}
      </div>
    </div>
  </div>
</template>
