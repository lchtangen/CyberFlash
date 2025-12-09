<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { listen } from '@tauri-apps/api/event';
import HolographicTerminal from '../../ui/HolographicTerminal.vue';

const lines = ref<string[]>([]);

const addLine = (text: string) => {
  const timestamp = new Date().toLocaleTimeString();
  lines.value.push(`[${timestamp}] ${text}`);
  if (lines.value.length > 1000) lines.value.shift();
};

let unlistenStdout: () => void;
let unlistenStderr: () => void;

onMounted(async () => {
  unlistenStdout = await listen<string>('shell-stdout', (event) => addLine(event.payload));
  unlistenStderr = await listen<string>('shell-stderr', (event) => addLine(event.payload));
  
  addLine('CyberFlash V2 Terminal Ready...');
});

onUnmounted(() => {
  if (unlistenStdout) unlistenStdout();
  if (unlistenStderr) unlistenStderr();
});
</script>

<template>
  <HolographicTerminal 
    :logs="lines" 
    title="LIVE_TERMINAL_V2" 
    height="16rem" 
  />
</template>
