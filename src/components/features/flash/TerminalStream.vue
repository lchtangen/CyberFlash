<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick, watch } from 'vue';
import { listen } from '@tauri-apps/api/event';
import GlassCard from '../../ui/GlassCard.vue';

const props = defineProps<{
  active: boolean;
}>();

const logs = ref<string[]>([]);
const terminalRef = ref<HTMLElement | null>(null);
let unlisten: (() => void) | null = null;

const addLog = (message: string) => {
  logs.value.push(message);
  if (logs.value.length > 1000) {
    logs.value.shift();
  }
  scrollToBottom();
};

const scrollToBottom = async () => {
  await nextTick();
  if (terminalRef.value) {
    terminalRef.value.scrollTop = terminalRef.value.scrollHeight;
  }
};

onMounted(async () => {
  unlisten = await listen<string>('fastboot-output', (event) => {
    addLog(event.payload);
  });
});

onUnmounted(() => {
  if (unlisten) {
    unlisten();
  }
});

watch(() => props.active, (newVal) => {
    if (newVal) {
        scrollToBottom();
    }
});
</script>

<template>
  <GlassCard class="flex flex-col h-full overflow-hidden bg-black/80 font-mono text-xs">
    <div class="flex items-center justify-between px-4 py-2 border-b border-white/10 bg-white/5">
      <div class="flex items-center gap-2">
        <span class="material-symbols-rounded text-primary text-sm">terminal</span>
        <span class="font-bold text-white/80">Fastboot Stream</span>
      </div>
      <button @click="logs = []" class="text-white/40 hover:text-white transition-colors">
        <span class="material-symbols-rounded text-sm">delete</span>
      </button>
    </div>
    
    <div 
      ref="terminalRef"
      class="flex-1 overflow-y-auto p-4 space-y-1 custom-scrollbar"
    >
      <div v-if="logs.length === 0" class="text-white/20 italic text-center mt-10">
        Waiting for command output...
      </div>
      <div v-for="(log, index) in logs" :key="index" class="break-all whitespace-pre-wrap text-white/80">
        <span class="text-primary/50 mr-2">$</span>{{ log }}
      </div>
    </div>
  </GlassCard>
</template>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 6px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: rgba(255, 255, 255, 0.05);
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.2);
  border-radius: 3px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.3);
}
</style>
