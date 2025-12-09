<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { useFlashStore } from '../../../stores/flash';

interface RomMetadata {
  name: string;
  size: number;
  hash: string;
  rom_type: string;
}

const flashStore = useFlashStore();
const selectedFile = ref<string | null>(null);
const metadata = ref<RomMetadata | null>(null);
const loading = ref(false);
const error = ref<string | null>(null);

const emit = defineEmits<{
  (e: 'romSelected', metadata: RomMetadata): void
}>();

const formatBytes = (bytes: number) => {
  if (bytes === 0) return '0 Bytes';
  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};

const handleFileSelect = async () => {
  try {
    const file = await open({
      multiple: false,
      filters: [{
        name: 'ROM Images',
        extensions: ['zip', 'img', 'bin', 'payload.bin']
      }]
    });

    if (file) {
      const path = Array.isArray(file) ? file[0] : file;
      if (path) {
        await processFile(path);
      }
    }
  } catch (e) {
    error.value = `Selection failed: ${e}`;
  }
};

const processFile = async (path: string) => {
  selectedFile.value = path;
  loading.value = true;
  error.value = null;
  metadata.value = null;

  try {
    const res = await invoke<RomMetadata>('parse_rom_file', { path });
    metadata.value = res;
    
    // Update global store for AI prediction
    flashStore.selectedRom = {
      name: res.name,
      path: path,
      size: res.size,
      hash: res.hash
    };

    emit('romSelected', res);
  } catch (e) {
    error.value = `Parsing failed: ${e}`;
  } finally {
    loading.value = false;
  }
};
</script>

<template>
  <div class="bg-surface/30 border border-white/10 rounded-xl p-6 backdrop-blur-md">
    <h3 class="text-lg font-semibold text-white mb-4">ROM Selection</h3>

    <!-- File Input Area -->
    <div 
      class="border-2 border-dashed border-white/10 rounded-xl p-8 text-center transition-all hover:border-primary/50 hover:bg-white/5 cursor-pointer"
      @click="handleFileSelect"
    >
      <div v-if="!selectedFile && !loading">
        <span class="material-symbols-rounded text-4xl text-text-muted mb-2">cloud_upload</span>
        <p class="text-text-secondary">Click to browse ROM file</p>
        <p class="text-xs text-text-muted mt-1">Supports .zip, .img, .bin</p>
      </div>

      <div v-else-if="loading">
        <div class="w-8 h-8 border-2 border-primary border-t-transparent rounded-full animate-spin mx-auto mb-2"></div>
        <p class="text-text-secondary">Analyzing file...</p>
      </div>

      <div v-else class="text-left">
        <div class="flex items-center gap-3 mb-2">
          <div class="w-10 h-10 rounded bg-primary/20 flex items-center justify-center text-primary">
            <span class="material-symbols-rounded">description</span>
          </div>
          <div class="overflow-hidden">
            <p class="text-white font-medium truncate">{{ metadata?.name || 'Unknown File' }}</p>
            <p class="text-xs text-text-muted">{{ selectedFile }}</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Metadata Display -->
    <div v-if="metadata" class="mt-4 grid grid-cols-2 gap-3">
      <div class="bg-black/20 p-3 rounded-lg border border-white/5">
        <div class="text-xs text-text-muted uppercase tracking-wider">Type</div>
        <div class="text-white font-mono">{{ metadata.rom_type }}</div>
      </div>
      <div class="bg-black/20 p-3 rounded-lg border border-white/5">
        <div class="text-xs text-text-muted uppercase tracking-wider">Size</div>
        <div class="text-white font-mono">{{ formatBytes(metadata.size) }}</div>
      </div>
      <div class="col-span-2 bg-black/20 p-3 rounded-lg border border-white/5">
        <div class="text-xs text-text-muted uppercase tracking-wider">SHA256 Hash</div>
        <div class="text-white font-mono text-xs break-all">{{ metadata.hash }}</div>
      </div>
    </div>

    <div v-if="error" class="mt-4 p-3 bg-error/10 border border-error/20 rounded-lg text-error text-sm">
      {{ error }}
    </div>
  </div>
</template>
