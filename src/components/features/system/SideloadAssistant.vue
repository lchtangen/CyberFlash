<script setup lang="ts">
import { ref, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/plugin-dialog';
import { useNotificationStore } from '../../../stores/notifications';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';
import GlassInput from '../../ui/GlassInput.vue';

const filePath = ref('');
const isFlashing = ref(false);
const progress = ref('');
const logs = ref<string[]>([]);
const notificationStore = useNotificationStore();

const unlisten = ref<() => void>();

const selectFile = async () => {
  try {
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'OTA Packages',
        extensions: ['zip']
      }]
    });
    if (selected && typeof selected === 'string') {
      filePath.value = selected;
    }
  } catch (e) {
    console.error('File selection failed', e);
  }
};

const startSideload = async () => {
  if (!filePath.value) return;
  
  isFlashing.value = true;
  logs.value = [];
  
  // Listen for progress
  unlisten.value = await listen<string>('sideload-progress', (event) => {
    progress.value = event.payload;
    logs.value.push(event.payload);
  });

  try {
    await invoke('adb_sideload', { filePath: filePath.value });
    logs.value.push("Sideload Complete!");
    notificationStore.addNotification({
      title: 'Sideload Complete',
      message: 'OTA package installed successfully.',
      type: 'success'
    });
  } catch (e) {
    logs.value.push(`Error: ${e}`);
    notificationStore.addNotification({
      title: 'Sideload Failed',
      message: String(e),
      type: 'error'
    });
  } finally {
    isFlashing.value = false;
    if (unlisten.value) unlisten.value();
  }
};

onUnmounted(() => {
  if (unlisten.value) unlisten.value();
});
</script>

<template>
  <GlassCard>
    <div class="flex items-center gap-3 mb-6">
      <div class="p-3 rounded-xl bg-primary/20 text-primary">
        <span class="material-symbols-rounded text-2xl">system_update</span>
      </div>
      <div>
        <h3 class="text-xl font-bold text-white">ADB Sideload</h3>
        <p class="text-sm text-white/60">Flash OTA zips directly via ADB</p>
      </div>
    </div>

    <div class="space-y-6">
      <div class="flex gap-2 items-end">
        <GlassInput 
          v-model="filePath"
          label="OTA Package"
          placeholder="/path/to/update.zip"
          icon="folder_zip"
          class="flex-1"
        />
        <VisionButton 
          @click="selectFile" 
          variant="secondary"
          icon="folder_open"
          class="mb-[2px]"
        >
          Browse
        </VisionButton>
      </div>

      <VisionButton 
        @click="startSideload"
        :loading="isFlashing"
        :disabled="!filePath"
        icon="send_to_mobile"
        class="w-full"
      >
        Start Sideload
      </VisionButton>

      <div v-if="logs.length > 0" class="bg-black/40 rounded-xl p-4 h-48 overflow-y-auto font-mono text-xs text-gray-300 custom-scrollbar border border-white/5">
        <div v-for="(log, i) in logs" :key="i" class="mb-1">
          <span class="text-primary mr-2">➜</span>{{ log }}
        </div>
      </div>
    </div>
  </GlassCard>
</template>
