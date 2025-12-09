<script setup lang="ts">
import { onMounted, ref, computed } from 'vue';
import { useHistoryStore } from '../../../stores/history';
import GlassCard from '../../ui/GlassCard.vue';
import GlassInput from '../../ui/GlassInput.vue';

const historyStore = useHistoryStore();
const searchQuery = ref('');
const showDetailModal = ref(false);
const selectedItem = ref<any>(null);

onMounted(() => {
  historyStore.fetchHistory();
});

const filteredActivities = computed(() => {
  if (!searchQuery.value) return historyStore.activities;
  const query = searchQuery.value.toLowerCase();
  return historyStore.activities.filter(item => 
    item.action.toLowerCase().includes(query) || 
    (item.details && item.details.toLowerCase().includes(query)) ||
    (item.device_serial && item.device_serial.toLowerCase().includes(query))
  );
});

const openDetail = (item: any) => {
  selectedItem.value = item;
  showDetailModal.value = true;
};

const getIcon = (action: string) => {
  const lower = action.toLowerCase();
  if (lower.includes('flash')) return 'flash_on';
  if (lower.includes('backup')) return 'backup';
  if (lower.includes('unlock')) return 'lock_open';
  if (lower.includes('install')) return 'download';
  if (lower.includes('reboot')) return 'restart_alt';
  if (lower.includes('wipe')) return 'delete';
  if (lower.includes('connect')) return 'usb';
  return 'history';
};

const formatTime = (timestamp: string) => {
    const date = new Date(timestamp);
    const now = new Date();
    const diff = (now.getTime() - date.getTime()) / 1000; // seconds
    
    if (diff < 60) return 'Just now';
    if (diff < 3600) return `${Math.floor(diff / 60)}m ago`;
    if (diff < 86400) return `${Math.floor(diff / 3600)}h ago`;
    return date.toLocaleDateString();
};
</script>

<template>
  <GlassCard class="h-full flex flex-col relative">
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-sm font-bold text-white flex items-center gap-2">
        <span class="material-symbols-rounded text-primary">history</span>
        Recent Activity
      </h3>
      <div class="flex gap-2">
        <button @click="historyStore.fetchHistory()" class="text-[10px] text-white/50 hover:text-white transition-colors" title="Refresh">
            <span class="material-symbols-rounded text-sm">refresh</span>
        </button>
        <button @click="historyStore.clearHistory()" class="text-[10px] text-white/50 hover:text-error transition-colors" title="Clear History">
            <span class="material-symbols-rounded text-sm">delete_sweep</span>
        </button>
      </div>
    </div>

    <!-- Filter Input -->
    <div class="mb-3">
      <GlassInput 
        v-model="searchQuery" 
        placeholder="Filter logs..." 
        icon="filter_list"
        class="text-xs h-8"
      />
    </div>

    <div class="space-y-3 flex-1 overflow-y-auto custom-scrollbar pr-2">
      <div v-if="filteredActivities.length === 0" class="text-center py-8 text-white/30 text-xs">
        No matching activity
      </div>

      <div 
        v-for="item in filteredActivities" 
        :key="item.id"
        @click="openDetail(item)"
        class="flex items-center gap-3 p-3 rounded-xl bg-white/5 border border-white/5 hover:bg-white/10 transition-colors group cursor-pointer"
      >
        <div 
          class="w-8 h-8 rounded-full flex items-center justify-center shrink-0"
          :class="{
            'bg-success/20 text-success': item.status === 'success',
            'bg-warning/20 text-warning': item.status === 'warning',
            'bg-error/20 text-error': item.status === 'error',
            'bg-blue-500/20 text-blue-400': item.status === 'info',
          }"
        >
          <span class="material-symbols-rounded text-sm">{{ getIcon(item.action) }}</span>
        </div>
        
        <div class="flex-1 min-w-0">
          <div class="text-xs font-bold text-white group-hover:text-primary transition-colors truncate">{{ item.action }}</div>
          <div class="text-[10px] text-text-secondary flex justify-between">
             <span class="truncate max-w-[100px]">{{ item.details || item.device_serial || 'System' }}</span>
             <span>{{ formatTime(item.timestamp) }}</span>
          </div>
        </div>

        <div class="w-1.5 h-1.5 rounded-full shrink-0" :class="{
            'bg-success shadow-[0_0_5px_rgba(48,209,88,0.5)]': item.status === 'success',
            'bg-warning shadow-[0_0_5px_rgba(255,214,10,0.5)]': item.status === 'warning',
            'bg-error shadow-[0_0_5px_rgba(255,69,58,0.5)]': item.status === 'error',
            'bg-blue-400 shadow-[0_0_5px_rgba(96,165,250,0.5)]': item.status === 'info',
        }"></div>
      </div>
    </div>

    <!-- Detail Modal -->
    <div v-if="showDetailModal" class="absolute inset-0 z-50 bg-surface/95 backdrop-blur-xl flex flex-col p-4 animate-fade-in">
      <div class="flex justify-between items-center mb-4">
        <h3 class="text-sm font-bold text-white">Log Details</h3>
        <button @click="showDetailModal = false" class="text-white/50 hover:text-white">
          <span class="material-symbols-rounded">close</span>
        </button>
      </div>
      
      <div v-if="selectedItem" class="flex-1 overflow-y-auto space-y-4 text-xs">
        <div class="space-y-1">
          <div class="text-white/50 uppercase tracking-wider text-[10px]">Action</div>
          <div class="text-white font-mono">{{ selectedItem.action }}</div>
        </div>
        <div class="space-y-1">
          <div class="text-white/50 uppercase tracking-wider text-[10px]">Status</div>
          <div :class="{
            'text-success': selectedItem.status === 'success',
            'text-error': selectedItem.status === 'error',
            'text-warning': selectedItem.status === 'warning',
            'text-blue-400': selectedItem.status === 'info'
          }" class="font-bold uppercase">{{ selectedItem.status }}</div>
        </div>
        <div class="space-y-1">
          <div class="text-white/50 uppercase tracking-wider text-[10px]">Timestamp</div>
          <div class="text-white font-mono">{{ new Date(selectedItem.timestamp).toLocaleString() }}</div>
        </div>
        <div class="space-y-1">
          <div class="text-white/50 uppercase tracking-wider text-[10px]">Device</div>
          <div class="text-white font-mono">{{ selectedItem.device_serial || 'N/A' }}</div>
        </div>
        <div class="space-y-1">
          <div class="text-white/50 uppercase tracking-wider text-[10px]">Details</div>
          <div class="p-3 bg-black/30 rounded-lg font-mono text-white/80 whitespace-pre-wrap break-all border border-white/5">
            {{ selectedItem.details || 'No details provided.' }}
          </div>
        </div>
      </div>
    </div>
  </GlassCard>
</template>

<style scoped>
.animate-fade-in {
  animation: fadeIn 0.2s ease-out;
}
@keyframes fadeIn {
  from { opacity: 0; transform: scale(0.95); }
  to { opacity: 1; transform: scale(1); }
}
</style>
