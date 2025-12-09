<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { useFlashStore } from '../../../stores/flash';
import GlassCard from '../../ui/GlassCard.vue';

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
  <GlassCard class="relative overflow-hidden group">
    <div class="flex items-center justify-between mb-6">
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-xl bg-primary/10 flex items-center justify-center border border-primary/20 shadow-[0_0_15px_rgba(10,132,255,0.1)]">
          <span class="material-symbols-rounded text-primary text-xl">folder_open</span>
        </div>
        <div>
          <h3 class="text-lg font-bold text-white tracking-wide">ROM Selection</h3>
          <p class="text-xs text-text-secondary font-medium">Select firmware image to flash</p>
        </div>
      </div>
      <div v-if="metadata" class="px-2 py-1 rounded-lg bg-success/10 border border-success/20 text-success text-xs font-bold flex items-center gap-1.5">
        <span class="w-1.5 h-1.5 rounded-full bg-success animate-pulse"></span>
        READY
      </div>
    </div>

    <!-- File Input Area -->
    <div 
      class="relative border-2 border-dashed border-white/10 rounded-2xl p-8 text-center transition-all duration-300 group/drop"
      :class="[
        loading ? 'bg-white/5 border-white/20 cursor-wait' : 'hover:border-primary/50 hover:bg-primary/5 cursor-pointer',
        metadata ? 'border-success/30 bg-success/5' : ''
      ]"
      @click="!loading && handleFileSelect()"
    >
      <!-- Loading State -->
      <div v-if="loading" class="flex flex-col items-center justify-center py-4">
        <div class="relative w-12 h-12 mb-4">
          <div class="absolute inset-0 rounded-full border-2 border-primary/20"></div>
          <div class="absolute inset-0 rounded-full border-2 border-primary border-t-transparent animate-spin"></div>
          <span class="material-symbols-rounded absolute inset-0 flex items-center justify-center text-primary text-lg animate-pulse">memory</span>
        </div>
        <p class="text-white font-medium">Analyzing Firmware...</p>
        <p class="text-xs text-text-secondary mt-1">Verifying integrity & compatibility</p>
      </div>

      <!-- Empty State -->
      <div v-else-if="!selectedFile" class="py-2 group-hover/drop:scale-105 transition-transform duration-300">
        <div class="w-16 h-16 mx-auto mb-4 rounded-full bg-white/5 flex items-center justify-center border border-white/10 shadow-inner group-hover/drop:bg-primary/10 group-hover/drop:border-primary/20 transition-colors">
          <span class="material-symbols-rounded text-3xl text-white/40 group-hover/drop:text-primary transition-colors">cloud_upload</span>
        </div>
        <h4 class="text-white font-medium mb-1">Click to browse ROM file</h4>
        <p class="text-xs text-text-muted">Supports .zip, .img, .bin, .payload</p>
      </div>

      <!-- Selected State -->
      <div v-else class="text-left relative z-10">
        <div class="flex items-start gap-4">
          <div class="w-12 h-12 rounded-xl bg-gradient-to-br from-primary/20 to-primary/5 flex items-center justify-center border border-primary/20 shadow-lg shrink-0">
            <span class="material-symbols-rounded text-primary text-2xl">description</span>
          </div>
          <div class="flex-1 min-w-0">
            <h4 class="text-white font-bold truncate text-lg">{{ metadata?.name || 'Unknown File' }}</h4>
            <p class="text-xs text-text-secondary font-mono truncate opacity-70 mt-0.5">{{ selectedFile }}</p>
            
            <div class="flex items-center gap-2 mt-3">
              <button class="text-xs text-primary hover:text-primary-hover font-medium hover:underline" @click.stop="handleFileSelect">
                Change File
              </button>
            </div>
          </div>
        </div>
      </div>
      
      <!-- Background Glow for Selected -->
      <div v-if="metadata" class="absolute inset-0 bg-gradient-to-br from-success/5 to-transparent pointer-events-none rounded-2xl"></div>
    </div>

    <!-- Metadata Grid -->
    <div v-if="metadata" class="mt-6 grid grid-cols-2 gap-3 animate-slide-up">
      <div class="bg-white/5 p-3 rounded-xl border border-white/5 backdrop-blur-sm hover:bg-white/10 transition-colors">
        <div class="flex items-center gap-2 mb-1">
          <span class="material-symbols-rounded text-xs text-text-muted">category</span>
          <span class="text-xs text-text-muted uppercase tracking-wider font-bold">Type</span>
        </div>
        <div class="text-white font-mono text-sm pl-5">{{ metadata.rom_type }}</div>
      </div>
      
      <div class="bg-white/5 p-3 rounded-xl border border-white/5 backdrop-blur-sm hover:bg-white/10 transition-colors">
        <div class="flex items-center gap-2 mb-1">
          <span class="material-symbols-rounded text-xs text-text-muted">hard_drive</span>
          <span class="text-xs text-text-muted uppercase tracking-wider font-bold">Size</span>
        </div>
        <div class="text-white font-mono text-sm pl-5">{{ formatBytes(metadata.size) }}</div>
      </div>
      
      <div class="col-span-2 bg-white/5 p-3 rounded-xl border border-white/5 backdrop-blur-sm hover:bg-white/10 transition-colors group/hash relative overflow-hidden">
        <div class="flex items-center gap-2 mb-1">
          <span class="material-symbols-rounded text-xs text-text-muted">fingerprint</span>
          <span class="text-xs text-text-muted uppercase tracking-wider font-bold">SHA256 Hash</span>
        </div>
        <div class="text-white font-mono text-[10px] pl-5 break-all opacity-70 group-hover/hash:opacity-100 transition-opacity">{{ metadata.hash }}</div>
        
        <!-- Verified Badge -->
        <div class="absolute top-2 right-2">
           <span class="material-symbols-rounded text-success/50 text-lg">verified_user</span>
        </div>
      </div>
    </div>

    <!-- Error Message -->
    <div v-if="error" class="mt-4 p-4 bg-error/10 border border-error/20 rounded-xl flex items-start gap-3 animate-shake">
      <span class="material-symbols-rounded text-error mt-0.5">error</span>
      <div>
        <h4 class="text-error font-bold text-sm">Selection Failed</h4>
        <p class="text-error/80 text-xs mt-1">{{ error }}</p>
      </div>
    </div>
  </GlassCard>
</template>

<style scoped>
@keyframes slide-up {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}

.animate-slide-up {
  animation: slide-up 0.4s ease-out forwards;
}

@keyframes shake {
  0%, 100% { transform: translateX(0); }
  25% { transform: translateX(-5px); }
  75% { transform: translateX(5px); }
}

.animate-shake {
  animation: shake 0.4s ease-in-out;
}
</style>
