<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';

interface BatchOpResult {
  item: string;
  success: boolean;
  message: string;
}

const files = ref<string[]>([]);
const isProcessing = ref(false);
const results = ref<BatchOpResult[]>([]);

const selectApks = async () => {
  try {
    const selected = await open({
      multiple: true,
      filters: [{ name: 'Android Package', extensions: ['apk'] }]
    });
    
    if (selected) {
      if (Array.isArray(selected)) {
        files.value = [...files.value, ...selected];
      } else {
        files.value.push(selected);
      }
    }
  } catch (e) {
    console.error(e);
  }
};

const removeFile = (index: number) => {
  files.value.splice(index, 1);
};

const installAll = async () => {
  if (files.value.length === 0) return;
  isProcessing.value = true;
  results.value = [];

  try {
    results.value = await invoke('batch_install_apks', { filePaths: files.value });
  } catch (e) {
    console.error(e);
  } finally {
    isProcessing.value = false;
  }
};

const clearAll = () => {
  files.value = [];
  results.value = [];
};
</script>

<template>
  <GlassCard>
    <div class="flex items-center gap-3 mb-6">
      <div class="p-3 rounded-xl bg-primary/20 text-primary">
        <span class="material-symbols-rounded text-2xl">apps</span>
      </div>
      <div>
        <h2 class="text-xl font-bold text-white">APK Batch Installer</h2>
        <p class="text-sm text-white/60">Install multiple apps at once.</p>
      </div>
    </div>

    <div class="space-y-4">
      <div class="flex gap-2">
        <VisionButton @click="selectApks" icon="add" size="sm">Add APKs</VisionButton>
        <VisionButton @click="clearAll" variant="secondary" size="sm" :disabled="files.length === 0">Clear</VisionButton>
        <div class="flex-1"></div>
        <VisionButton @click="installAll" :loading="isProcessing" :disabled="files.length === 0" icon="download">Install All</VisionButton>
      </div>

      <div class="bg-black/20 rounded-xl border border-white/5 min-h-[200px] max-h-[300px] overflow-y-auto custom-scrollbar p-2">
        <div v-if="files.length === 0" class="h-full flex flex-col items-center justify-center text-white/20">
          <span class="material-symbols-rounded text-4xl mb-2">drag_drop</span>
          <p class="text-sm">Select APKs to start</p>
        </div>
        
        <div v-else class="space-y-2">
          <div 
            v-for="(file, index) in files" 
            :key="index"
            class="flex items-center justify-between p-3 rounded-lg bg-white/5 border border-white/10 group"
          >
            <div class="flex items-center gap-3 overflow-hidden">
              <span class="material-symbols-rounded text-white/40">android</span>
              <span class="text-sm text-white/80 truncate font-mono">{{ file.split(/[/\\]/).pop() }}</span>
            </div>
            
            <div class="flex items-center gap-2">
              <!-- Result Status -->
              <template v-if="results.length > 0">
                <span v-if="results[index]?.success" class="text-success text-xs font-bold">INSTALLED</span>
                <span v-else-if="results[index]" class="text-error text-xs font-bold">FAILED</span>
                <span v-else class="text-white/20 text-xs">PENDING</span>
              </template>
              
              <button v-if="results.length === 0" @click="removeFile(index)" class="text-white/20 hover:text-error transition-colors">
                <span class="material-symbols-rounded text-sm">close</span>
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </GlassCard>
</template>
