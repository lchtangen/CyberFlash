<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { useNotificationStore } from '../../stores/notifications';

const bootImgPath = ref('');
const isPatching = ref(false);
const message = ref<string | null>(null);
const notificationStore = useNotificationStore();

const selectBootImg = async () => {
  try {
    const file = await open({
      filters: [{ name: 'Boot Image', extensions: ['img'] }]
    });
    if (file) {
      bootImgPath.value = Array.isArray(file) ? file[0] : file;
    }
  } catch (e) {
    console.error(e);
    notificationStore.addNotification({
      title: 'Selection Error',
      message: 'Failed to select file',
      type: 'error'
    });
  }
};

const patchImage = async () => {
  if (!bootImgPath.value) return;
  isPatching.value = true;
  message.value = null;

  try {
    // In a real scenario, we might ask for Magisk APK location too, or bundle it.
    // For now, we pass a dummy path.
    const res = await invoke<string>('patch_boot_image', { 
      bootImgPath: bootImgPath.value,
      magiskApkPath: "" 
    });
    message.value = `Success! Patched image saved to: ${res}`;
    notificationStore.addNotification({
      title: 'Patch Successful',
      message: `Saved to: ${res}`,
      type: 'success'
    });
  } catch (e) {
    message.value = `Error: ${e}`;
    notificationStore.addNotification({
      title: 'Patch Failed',
      message: String(e),
      type: 'error'
    });
  } finally {
    isPatching.value = false;
  }
};
</script>

<template>
  <div class="bg-surface/30 border border-white/10 rounded-xl p-6 backdrop-blur-md">
    <div class="flex items-center gap-3 mb-4">
      <div class="w-10 h-10 rounded-full bg-success/20 flex items-center justify-center text-success">
        <span class="material-symbols-rounded">android</span>
      </div>
      <div>
        <h3 class="text-lg font-bold text-white">Magisk Injector</h3>
        <p class="text-xs text-text-secondary">Root your device</p>
      </div>
    </div>

    <div class="space-y-4">
      <div 
        @click="selectBootImg"
        class="border border-dashed border-white/20 rounded-lg p-4 text-center cursor-pointer hover:bg-white/5 transition-colors"
      >
        <div v-if="!bootImgPath" class="text-text-muted text-sm">
          Select boot.img
        </div>
        <div v-else class="text-white text-xs font-mono break-all">
          {{ bootImgPath }}
        </div>
      </div>

      <button 
        @click="patchImage"
        :disabled="!bootImgPath || isPatching"
        class="w-full py-3 rounded-xl font-bold bg-success text-white hover:bg-success-hover shadow-lg shadow-success/20 transition-all duration-200"
      >
        <span v-if="isPatching" class="material-symbols-rounded animate-spin text-sm mr-2">refresh</span>
        {{ isPatching ? 'Patching...' : 'Patch with Magisk' }}
      </button>
    </div>

    <div v-if="message" class="mt-3 text-xs text-center font-mono p-2 rounded bg-black/20" :class="message.includes('Error') ? 'text-error' : 'text-success'">
      {{ message }}
    </div>
  </div>
</template>
