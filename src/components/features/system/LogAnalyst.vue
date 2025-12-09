<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useNotificationStore } from '../../../stores/notifications';

interface LogEntry {
  level: string;
  tag: string;
  message: string;
}

const logs = ref<LogEntry[]>([]);
const loading = ref(false);
const filterLevel = ref('All');
const searchQuery = ref('');
const notificationStore = useNotificationStore();

const fetchLogs = async () => {
  loading.value = true;
  try {
    logs.value = await invoke('get_logcat_dump');
    notificationStore.addNotification({
      title: 'Logs Refreshed',
      message: `Fetched ${logs.value.length} log entries.`,
      type: 'success',
      duration: 2000
    });
  } catch (e) {
    console.error(e);
    notificationStore.addNotification({
      title: 'Log Fetch Failed',
      message: String(e),
      type: 'error'
    });
  } finally {
    loading.value = false;
  }
};

const filteredLogs = computed(() => {
  return logs.value.filter(log => {
    const matchesLevel = filterLevel.value === 'All' || log.level === filterLevel.value;
    const matchesSearch = log.tag.toLowerCase().includes(searchQuery.value.toLowerCase()) || 
                          log.message.toLowerCase().includes(searchQuery.value.toLowerCase());
    return matchesLevel && matchesSearch;
  });
});

const getLevelColor = (level: string) => {
  switch (level) {
    case 'Error': return 'text-error';
    case 'Warning': return 'text-warning';
    case 'Info': return 'text-success';
    case 'Debug': return 'text-primary';
    default: return 'text-text-secondary';
  }
};

onMounted(fetchLogs);
</script>

<template>
  <div class="bg-surface/30 border border-white/10 rounded-xl p-4 backdrop-blur-md flex flex-col h-[400px]">
    <div class="flex justify-between items-center mb-4">
      <h3 class="text-lg font-semibold text-white flex items-center gap-2">
        <span class="material-symbols-rounded text-warning">bug_report</span>
        Log Analyst
      </h3>
      <div class="flex gap-2">
        <button @click="fetchLogs" class="p-2 hover:bg-white/10 rounded-lg text-white transition-colors flex items-center justify-center">
          <span class="material-symbols-rounded text-sm" :class="{ 'animate-spin': loading }">refresh</span>
        </button>
      </div>
    </div>

    <!-- Filters -->
    <div class="flex gap-2 mb-4">
      <select v-model="filterLevel" class="bg-black/20 border border-white/10 rounded-lg px-3 py-2 text-xs text-white focus:outline-none focus:border-primary/50">
        <option value="All">All Levels</option>
        <option value="Error">Errors</option>
        <option value="Warning">Warnings</option>
        <option value="Info">Info</option>
      </select>
      <input 
        v-model="searchQuery" 
        type="text" 
        placeholder="Search logs..." 
        class="flex-1 bg-black/20 border border-white/10 rounded-lg px-3 py-2 text-xs text-white focus:outline-none focus:border-primary/50"
      >
    </div>

    <!-- Log Window -->
    <div class="flex-1 overflow-y-auto custom-scrollbar bg-black/40 rounded-lg p-2 font-mono text-[10px] space-y-1">
      <div v-if="loading" class="text-center py-8 text-text-muted">Loading logs...</div>
      <div v-else-if="filteredLogs.length === 0" class="text-center py-8 text-text-muted">No logs found.</div>
      
      <div v-for="(log, i) in filteredLogs" :key="i" class="flex gap-2 hover:bg-white/5 p-0.5 rounded">
        <span class="w-8 font-bold shrink-0" :class="getLevelColor(log.level)">{{ log.level[0] }}</span>
        <span class="w-24 text-text-secondary truncate shrink-0" :title="log.tag">{{ log.tag }}</span>
        <span class="text-gray-300 break-all">{{ log.message }}</span>
      </div>
    </div>
  </div>
</template>
