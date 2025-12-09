<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useNotificationStore } from '../../../stores/notifications';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';
import GlassInput from '../../ui/GlassInput.vue';
import GlassSelect from '../../ui/GlassSelect.vue';

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

const levelOptions = [
  { value: 'All', label: 'All Levels' },
  { value: 'Error', label: 'Errors' },
  { value: 'Warning', label: 'Warnings' },
  { value: 'Info', label: 'Info' }
];

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
  <GlassCard class="flex flex-col h-[500px]">
    <div class="flex justify-between items-center mb-6">
      <div class="flex items-center gap-3">
        <div class="p-2 rounded-lg bg-warning/20 text-warning">
          <span class="material-symbols-rounded text-xl">bug_report</span>
        </div>
        <div>
          <h3 class="text-lg font-bold text-white">Log Analyst</h3>
          <p class="text-xs text-white/60">Analyze system logs (Logcat)</p>
        </div>
      </div>
      <VisionButton 
        @click="fetchLogs" 
        :loading="loading"
        variant="secondary"
        icon="refresh"
        class="!p-2"
      />
    </div>

    <!-- Filters -->
    <div class="grid grid-cols-3 gap-3 mb-4">
      <GlassSelect
        v-model="filterLevel"
        :options="levelOptions"
        class="col-span-1"
      />
      <GlassInput 
        v-model="searchQuery" 
        placeholder="Search logs..." 
        icon="search"
        class="col-span-2"
      />
    </div>

    <!-- Log Window -->
    <div class="flex-1 overflow-y-auto custom-scrollbar bg-black/40 rounded-xl p-3 font-mono text-[10px] space-y-1 border border-white/5">
      <div v-if="loading" class="text-center py-8 text-white/40">Loading logs...</div>
      <div v-else-if="filteredLogs.length === 0" class="text-center py-8 text-white/40">No logs found.</div>
      
      <div v-for="(log, i) in filteredLogs" :key="i" class="flex gap-2 hover:bg-white/5 p-1 rounded transition-colors">
        <span class="w-6 font-bold shrink-0 text-center" :class="getLevelColor(log.level)">{{ log.level[0] }}</span>
        <span class="w-24 text-white/60 truncate shrink-0" :title="log.tag">{{ log.tag }}</span>
        <span class="text-gray-300 break-all">{{ log.message }}</span>
      </div>
    </div>
  </GlassCard>
</template>
