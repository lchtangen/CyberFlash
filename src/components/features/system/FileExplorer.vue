<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '@/components/ui/GlassCard.vue';
import VisionButton from '@/components/ui/VisionButton.vue';
import { useDeviceStore } from '@/stores/device';

interface FileEntry {
  name: string;
  path: string;
  is_dir: boolean;
  size: string;
  permissions: string;
  date: string;
}

const deviceStore = useDeviceStore();
const currentPath = ref('/sdcard');
const files = ref<FileEntry[]>([]);
const isLoading = ref(false);
const errorMsg = ref('');
const selectedFile = ref<FileEntry | null>(null);

const loadFiles = async (path: string) => {
  if (!deviceStore.serial) return;
  
  isLoading.value = true;
  errorMsg.value = '';
  files.value = [];
  
  try {
    files.value = await invoke('list_directory', { path });
    currentPath.value = path;
  } catch (e) {
    errorMsg.value = String(e);
  } finally {
    isLoading.value = false;
  }
};

const navigateUp = () => {
  if (currentPath.value === '/') return;
  const parent = currentPath.value.split('/').slice(0, -1).join('/') || '/';
  loadFiles(parent);
};

const handleItemClick = (file: FileEntry) => {
  selectedFile.value = file;
};

const handleItemDblClick = (file: FileEntry) => {
  if (file.is_dir) {
    loadFiles(file.path);
  }
};

const downloadFile = async () => {
  if (!selectedFile.value || selectedFile.value.is_dir) return;
  // TODO: Implement download dialog
  // For now, just pull to Downloads
  try {
    await invoke('pull_file', { 
      remotePath: selectedFile.value.path,
      localPath: '' // Backend handles default download dir if empty? No, need to check adb.rs
    });
  } catch (e) {
    // errorMsg.value = String(e);
  }
};


onMounted(() => {
  if (deviceStore.serial) {
    loadFiles(currentPath.value);
  }
});

watch(() => deviceStore.serial, (newSerial) => {
  if (newSerial) {
    loadFiles(currentPath.value);
  }
});
</script>

<template>
  <GlassCard class="h-full flex flex-col overflow-hidden">
    <!-- Header / Breadcrumbs -->
    <div class="p-4 border-b border-white/10 flex items-center gap-2">
      <button 
        @click="navigateUp"
        class="p-2 rounded-lg hover:bg-white/10 transition-colors text-white/70 hover:text-white"
        :disabled="currentPath === '/'"
      >
        <span class="i-lucide-arrow-up w-4 h-4"></span>
      </button>
      
      <div class="flex-1 bg-black/30 rounded-lg px-3 py-2 text-sm font-mono text-white/80 truncate">
        {{ currentPath }}
      </div>
      
      <VisionButton 
        variant="secondary" 
        size="sm"
        @click="loadFiles(currentPath)"
        :loading="isLoading"
      >
        <span class="i-lucide-refresh-cw w-4 h-4"></span>
      </VisionButton>
    </div>

    <!-- File List -->
    <div class="flex-1 overflow-y-auto p-2">
      <div v-if="errorMsg" class="p-4 text-error text-sm bg-error/10 rounded-lg m-2">
        {{ errorMsg }}
      </div>
      
      <div v-else class="grid grid-cols-1 gap-1">
        <div 
          v-for="file in files" 
          :key="file.path"
          @click="handleItemClick(file)"
          @dblclick="handleItemDblClick(file)"
          class="flex items-center gap-3 p-2 rounded-lg cursor-pointer transition-colors group"
          :class="selectedFile?.path === file.path ? 'bg-primary/20 border border-primary/30' : 'hover:bg-white/5 border border-transparent'"
        >
          <div class="w-8 h-8 rounded-lg flex items-center justify-center bg-white/5 text-white/50 group-hover:text-white transition-colors">
            <span v-if="file.is_dir" class="i-lucide-folder w-5 h-5 text-yellow-400/80"></span>
            <span v-else class="i-lucide-file w-5 h-5"></span>
          </div>
          
          <div class="flex-1 min-w-0">
            <div class="text-sm font-medium text-white truncate">{{ file.name }}</div>
            <div class="text-xs text-white/40 flex gap-2">
              <span>{{ file.permissions }}</span>
              <span>•</span>
              <span>{{ file.size }}</span>
              <span>•</span>
              <span>{{ file.date }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Footer Actions -->
    <div class="p-4 border-t border-white/10 flex justify-end gap-2 bg-black/20">
      <VisionButton 
        variant="danger" 
        size="sm"
        :disabled="!selectedFile"
      >
        <span class="i-lucide-trash-2 w-4 h-4 mr-2"></span>
        Delete
      </VisionButton>
      
      <VisionButton 
        variant="primary" 
        size="sm"
        :disabled="!selectedFile || selectedFile.is_dir"
        @click="downloadFile"
      >
        <span class="i-lucide-download w-4 h-4 mr-2"></span>
        Download
      </VisionButton>
    </div>
  </GlassCard>
</template>
