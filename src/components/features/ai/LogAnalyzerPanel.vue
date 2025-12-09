<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue';
import GlassCard from '../../ui/GlassCard.vue';
import GlassInput from '../../ui/GlassInput.vue';

const logs = ref<string[]>([]);
const searchQuery = ref('');
const isListening = ref(false);
const unlisten = ref<() => void>();

const filteredLogs = computed(() => {
  if (!searchQuery.value) return logs.value;
  return logs.value.filter(log => log.toLowerCase().includes(searchQuery.value.toLowerCase()));
});

const startLogcat = async () => {
  isListening.value = true;
  logs.value = [];
  // Mocking log stream for now as backend might not be fully ready for streaming logcat to frontend event
  // In a real scenario, we would invoke a command that starts emitting events
  try {
    // await invoke('start_logcat_stream'); 
    // unlisten.value = await listen<string>('logcat-data', (event) => {
    //   logs.value.push(event.payload);
    //   if (logs.value.length > 1000) logs.value.shift();
    // });
    
    // Mock data
    logs.value.push("[INFO] ADB Server started successfully");
    logs.value.push("[DEBUG] Device connected: Pixel 7 Pro");
    logs.value.push("[INFO] Battery level: 85%");
    
    setInterval(() => {
      if(isListening.value) {
        const types = ['[INFO]', '[DEBUG]', '[WARN]', '[ERROR]'];
        const type = types[Math.floor(Math.random() * types.length)];
        logs.value.push(`${type} System event timestamp: ${Date.now()}`);
        if (logs.value.length > 100) logs.value.shift();
      }
    }, 2000);

  } catch (e) {
    console.error(e);
  }
};

const stopLogcat = async () => {
  isListening.value = false;
  if (unlisten.value) {
    unlisten.value();
    unlisten.value = undefined;
  }
  // await invoke('stop_logcat_stream');
};

const clearLogs = () => {
  logs.value = [];
};

onMounted(() => {
  startLogcat();
});

onUnmounted(() => {
  stopLogcat();
});
</script>

<template>
  <GlassCard noPadding class="w-full h-full flex flex-col overflow-hidden border-white/10 shadow-2xl shadow-black/50">
    <!-- Header -->
    <div class="px-5 py-4 border-b border-white/5 flex items-center justify-between bg-white/5 backdrop-blur-md z-10">
      <div class="flex items-center gap-4">
        <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-blue-500/20 to-blue-500/5 flex items-center justify-center border border-white/10 shadow-inner">
          <span class="material-symbols-rounded text-blue-400 text-xl">analytics</span>
        </div>
        <div>
          <h3 class="font-bold text-white text-base tracking-wide">Log Analyzer</h3>
          <p class="text-xs text-text-secondary font-medium">Real-time ADB Logcat</p>
        </div>
      </div>
      
      <button 
        @click="$emit('close')"
        class="p-2 rounded-lg hover:bg-white/10 text-text-secondary hover:text-white transition-all duration-200 group"
      >
        <span class="material-symbols-rounded text-lg group-hover:scale-110 transition-transform">close</span>
      </button>
    </div>

    <!-- Toolbar -->
    <div class="p-3 border-b border-white/5 bg-white/5 flex gap-2">
      <GlassInput 
        v-model="searchQuery" 
        placeholder="Filter logs..." 
        icon="filter_list"
        class="flex-1 text-xs"
      />
      <button 
        @click="isListening ? stopLogcat() : startLogcat()"
        class="px-3 py-1.5 rounded-lg border transition-colors flex items-center gap-2 text-xs font-medium"
        :class="isListening ? 'bg-red-500/10 border-red-500/20 text-red-400 hover:bg-red-500/20' : 'bg-green-500/10 border-green-500/20 text-green-400 hover:bg-green-500/20'"
      >
        <span class="material-symbols-rounded text-sm">{{ isListening ? 'stop' : 'play_arrow' }}</span>
        {{ isListening ? 'Stop' : 'Start' }}
      </button>
      <button 
        @click="clearLogs"
        class="px-3 py-1.5 rounded-lg bg-white/5 border border-white/10 hover:bg-white/10 text-text-secondary hover:text-white transition-colors text-xs font-medium flex items-center gap-2"
      >
        <span class="material-symbols-rounded text-sm">delete_sweep</span>
        Clear
      </button>
    </div>

    <!-- Log View -->
    <div class="flex-1 overflow-y-auto p-4 custom-scrollbar bg-black/40 font-mono text-xs">
      <div v-if="filteredLogs.length === 0" class="h-full flex flex-col items-center justify-center text-text-secondary opacity-50">
        <span class="material-symbols-rounded text-4xl mb-2">text_snippet</span>
        <p>No logs to display</p>
      </div>
      <div v-else class="space-y-1">
        <div 
          v-for="(log, index) in filteredLogs" 
          :key="index" 
          class="break-all hover:bg-white/5 px-2 py-0.5 rounded transition-colors"
          :class="{
            'text-blue-400': log.includes('[INFO]'),
            'text-yellow-400': log.includes('[WARN]'),
            'text-red-400': log.includes('[ERROR]'),
            'text-gray-400': log.includes('[DEBUG]')
          }"
        >
          {{ log }}
        </div>
      </div>
    </div>
  </GlassCard>
</template>
