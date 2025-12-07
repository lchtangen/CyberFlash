<script setup lang="ts">
interface LogEntry {
  id: number;
  timestamp: string;
  level: 'info' | 'warning' | 'error' | 'success';
  message: string;
}

defineProps<{
  logs: LogEntry[];
}>();

const getColor = (level: string) => {
  switch(level) {
    case 'error': return 'text-red-500 drop-shadow-[0_0_5px_rgba(239,68,68,0.5)]';
    case 'warning': return 'text-yellow-500';
    case 'success': return 'text-green-500 drop-shadow-[0_0_5px_rgba(34,197,94,0.5)]';
    default: return 'text-gray-300';
  }
};
</script>
<template>
  <div class="h-64 font-mono text-sm flex flex-col bg-white/5 border border-white/10 rounded-xl backdrop-blur-md">
    <div class="flex-1 overflow-y-auto custom-scrollbar p-4 space-y-1">
      <div v-for="log in logs" :key="log.id" class="flex gap-3 hover:bg-white/5 p-0.5 rounded transition-colors">
        <span class="text-gray-600 select-none shrink-0">{{ log.timestamp }}</span>
        <span :class="getColor(log.level)" class="break-all">{{ log.message }}</span>
      </div>
      <div v-if="logs.length === 0" class="text-gray-600 italic text-center mt-10">
        // No logs available...
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
