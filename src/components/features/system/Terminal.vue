<script setup lang="ts">
import { useFlashStore } from '../../../stores/flash';
import { computed, ref, watch, nextTick } from 'vue';

const flashStore = useFlashStore();
const terminalRef = ref<HTMLElement | null>(null);

const logs = computed(() => {
  return flashStore.logs.map((msg, index) => ({
    id: index,
    timestamp: new Date().toLocaleTimeString(),
    level: msg.toLowerCase().includes('error') ? 'error' : (msg.toLowerCase().includes('success') ? 'success' : 'info'),
    message: msg
  }));
});

const getColor = (level: string) => {
  switch(level) {
    case 'error': return 'text-error';
    case 'warning': return 'text-warning';
    case 'success': return 'text-success';
    default: return 'text-text-secondary';
  }
};

watch(() => flashStore.logs.length, async () => {
  await nextTick();
  if (terminalRef.value) {
    terminalRef.value.scrollTop = terminalRef.value.scrollHeight;
  }
});
</script>

<template>
  <div class="h-64 font-mono text-sm flex flex-col bg-black/40 border border-white/10 rounded-xl backdrop-blur-md shadow-inner overflow-hidden">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-2 border-b border-white/5 bg-white/5">
      <div class="flex items-center gap-2">
        <span class="material-symbols-rounded text-xs text-text-secondary">terminal</span>
        <span class="text-[10px] text-text-secondary uppercase tracking-wider font-bold">Terminal Output</span>
      </div>
    </div>

    <!-- Logs Area -->
    <div ref="terminalRef" class="flex-1 overflow-y-auto custom-scrollbar p-4 space-y-1">
      <div v-for="log in logs" :key="log.id" class="flex gap-3 hover:bg-white/5 p-0.5 rounded transition-colors group">
        <span class="text-text-muted select-none shrink-0 text-[10px] pt-1 opacity-50 group-hover:opacity-100 transition-opacity">{{ log.timestamp }}</span>
        <span :class="getColor(log.level)" class="break-all leading-relaxed">{{ log.message }}</span>
      </div>
      <div v-if="logs.length === 0" class="h-full flex flex-col items-center justify-center text-text-muted italic opacity-30">
        <span class="material-symbols-rounded text-3xl mb-2">code_blocks</span>
        <span>// System Ready. Waiting for commands...</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 6px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: rgba(0, 0, 0, 0.2);
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 3px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.2);
}
</style>
