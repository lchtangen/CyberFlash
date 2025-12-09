<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useNotificationStore } from '../../../stores/notifications';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';

interface DownloadItem {
  id: string;
  name: string;
  filename: string;
  url: string;
  size_mb: number;
  required: boolean;
  manual: boolean;
  manual_instructions?: string;
  status?: 'pending' | 'downloading' | 'completed' | 'error';
  progress?: number;
}

const downloads = ref<DownloadItem[]>([]);
const isLoading = ref(true);
const notificationStore = useNotificationStore();
const unlisten = ref<() => void>();

const fetchDownloads = async () => {
  try {
    const config: any = await invoke('get_required_downloads');
    downloads.value = config.files.map((f: any) => ({
      ...f,
      // If backend says downloaded, trust it. Otherwise pending.
      status: f.status === 'downloaded' ? 'completed' : 'pending',
      progress: f.status === 'downloaded' ? 100 : 0
    }));
  } catch (e) {
    notificationStore.addNotification({
      title: 'Config Error',
      message: 'Failed to load download list.',
      type: 'error'
    });
  } finally {
    isLoading.value = false;
  }
};

const startDownload = async (item: DownloadItem) => {
  if (item.manual) {
    // Open URL in browser
    await invoke('open_url', { url: item.url }).catch(() => {
        window.open(item.url, '_blank');
    });
    notificationStore.addNotification({
      title: 'Manual Download',
      message: item.manual_instructions || 'Please download the file manually.',
      type: 'warning'
    });
    return;
  }

  try {
    item.status = 'downloading';
    item.progress = 0;

    // Listen for progress
    if (unlisten.value) unlisten.value();
    unlisten.value = await listen<number>('download-progress', (event) => {
      item.progress = event.payload;
    });

    // Use Auto Download (to default folder)
    await invoke('download_file_auto', { url: item.url, filename: item.filename });
    
    item.status = 'completed';
    item.progress = 100;
    notificationStore.addNotification({
      title: 'Download Complete',
      message: `${item.name} saved to downloads folder.`,
      type: 'success'
    });
    
    // Refresh list to update status
    await fetchDownloads();

  } catch (e) {
    item.status = 'error';
    notificationStore.addNotification({
      title: 'Download Failed',
      message: String(e),
      type: 'error'
    });
  }
};

onMounted(fetchDownloads);

onUnmounted(() => {
  if (unlisten.value) unlisten.value();
});
</script>

<template>
  <GlassCard class="h-full flex flex-col">
    <div class="flex items-center gap-3 mb-6">
      <div class="w-10 h-10 rounded-full bg-primary/20 flex items-center justify-center text-primary shadow-lg shadow-primary/10">
        <span class="material-symbols-rounded">cloud_download</span>
      </div>
      <div>
        <h3 class="text-lg font-bold text-white tracking-tight">Supply Chain</h3>
        <p class="text-xs text-text-secondary">Required Assets for Flashing</p>
      </div>
    </div>

    <div v-if="isLoading" class="flex-1 flex items-center justify-center">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
    </div>

    <div v-else class="space-y-3 overflow-y-auto custom-scrollbar pr-2">
      <div 
        v-for="item in downloads" 
        :key="item.id"
        class="p-4 rounded-xl border border-white/5 bg-white/5 hover:bg-white/10 transition-all duration-200 group"
      >
        <div class="flex justify-between items-start mb-2">
          <div>
            <div class="flex items-center gap-2">
              <h4 class="font-medium text-white text-sm">{{ item.name }}</h4>
              <span v-if="item.required" class="px-1.5 py-0.5 rounded text-[10px] font-bold bg-error/20 text-error border border-error/20">REQ</span>
              <span v-if="item.manual" class="px-1.5 py-0.5 rounded text-[10px] font-bold bg-warning/20 text-warning border border-warning/20">MANUAL</span>
            </div>
            <p class="text-xs text-text-secondary mt-0.5">{{ item.filename }} • {{ item.size_mb }} MB</p>
          </div>
          
          <VisionButton 
            size="sm" 
            :variant="item.status === 'completed' ? 'secondary' : 'primary'"
            :disabled="item.status === 'downloading' || item.status === 'completed'"
            @click="startDownload(item)"
            :icon="item.status === 'completed' ? 'check' : 'download'"
          >
            {{ item.status === 'completed' ? 'Ready' : (item.manual ? 'Open' : 'Get') }}
          </VisionButton>
        </div>

        <!-- Progress Bar -->
        <div v-if="item.status === 'downloading'" class="w-full bg-white/10 rounded-full h-1.5 mt-3 overflow-hidden">
          <div 
            class="bg-primary h-full rounded-full transition-all duration-300"
            :style="{ width: `${item.progress}%` }"
          ></div>
        </div>
        
        <div v-if="item.status === 'error'" class="mt-2 text-xs text-error flex items-center gap-1">
          <span class="material-symbols-rounded text-sm">error</span>
          Download failed. Please try again.
        </div>
      </div>
    </div>
  </GlassCard>
</template>
