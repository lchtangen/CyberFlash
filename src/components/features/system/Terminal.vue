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
  <div class="h-full font-mono text-sm flex flex-col bg-surface/80 border border-white/10 rounded-xl backdrop-blur-xl shadow-2xl shadow-black/50 overflow-hidden relative group ring-1 ring-white/5">
    <!-- Header -->
    <div class="flex items-center justify-between px-4 py-2.5 border-b border-white/10 bg-surface/50 select-none backdrop-blur-md">
      <div class="flex items-center gap-2">
        <div class="flex gap-1.5">
          <div class="w-3 h-3 rounded-full bg-[#FF5F56] border border-[#E0443E]"></div>
          <div class="w-3 h-3 rounded-full bg-[#FFBD2E] border border-[#DEA123]"></div>
          <div class="w-3 h-3 rounded-full bg-[#27C93F] border border-[#1AAB29]"></div>
        </div>
      </div>
      <div class="absolute left-1/2 -translate-x-1/2 flex items-center gap-2 opacity-50">
        <span class="material-symbols-rounded text-xs">terminal</span>
        <span class="text-[10px] uppercase tracking-wider font-bold">cyberflash-cli — bash</span>
      </div>
      <div class="text-[10px] text-text-muted opacity-50">80x24</div>
    </div>

    <!-- Logs Area -->
    <div ref="terminalRef" class="flex-1 overflow-y-auto custom-scrollbar p-4 space-y-1 relative">
      <!-- Welcome Message -->
      <div v-if="logs.length === 0" class="text-text-muted mb-4">
        <p>CyberFlash v2.0.0 (x86_64-unknown-linux-gnu)</p>
        <p>Type "help" for assistance.</p>
        <br>
      </div>

      <div v-for="log in logs" :key="log.id" class="flex gap-3 hover:bg-white/5 p-0.5 -mx-2 px-2 rounded transition-colors group/line">
        <span class="text-text-muted select-none shrink-0 text-[10px] pt-0.5 opacity-30 group-hover/line:opacity-100 transition-opacity font-mono">{{ log.timestamp }}</span>
        <div class="flex-1 break-all leading-relaxed">
           <span v-if="log.level === 'info'" class="text-primary mr-2">➜</span>
           <span v-else-if="log.level === 'success'" class="text-success mr-2">✔</span>
           <span v-else-if="log.level === 'error'" class="text-error mr-2">✖</span>
           <span :class="getColor(log.level)">{{ log.message }}</span>
        </div>
      </div>

      <!-- Active Prompt Line -->
      <div class="flex items-center gap-2 mt-2 text-white/90">
        <span class="text-success font-bold">user@cyberflash</span>
        <span class="text-text-muted">:</span>
        <span class="text-primary font-bold">~</span>
        <span class="text-text-muted">$</span>
        <span class="w-2 h-4 bg-white/50 animate-pulse"></span>
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
