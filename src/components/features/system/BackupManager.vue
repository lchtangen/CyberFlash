<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';
import { useNotificationStore } from '../../../stores/notifications';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';
import GlassInput from '../../ui/GlassInput.vue';
import ToggleSwitch from '../../ui/ToggleSwitch.vue';

const backupPath = ref('');
const isBackingUp = ref(false);
const message = ref<string | null>(null);
const notificationStore = useNotificationStore();

const options = ref({
  apk: false,
  shared: false,
  system: false,
  all: true
});

const selectBackupLocation = async () => {
  try {
    const path = await save({
      filters: [{
        name: 'Android Backup',
        extensions: ['ab']
      }]
    });
    if (path) {
      backupPath.value = path;
    }
  } catch (e) {
    console.error(e);
    notificationStore.addNotification({
      title: 'Selection Error',
      message: 'Failed to select backup location',
      type: 'error'
    });
  }
};

const startBackup = async () => {
  if (!backupPath.value) return;
  
  isBackingUp.value = true;
  message.value = null;

  try {
    const res = await invoke<string>('adb_backup', {
      backupPath: backupPath.value,
      packages: [], // Empty means -all if options.all is true
      apk: options.value.apk,
      shared: options.value.shared,
      system: options.value.system
    });
    message.value = res;
    notificationStore.addNotification({
      title: 'Backup Started',
      message: 'Check device to confirm backup operation.',
      type: 'success',
      duration: 5000
    });
  } catch (e) {
    message.value = `Error: ${e}`;
    notificationStore.addNotification({
      title: 'Backup Failed',
      message: String(e),
      type: 'error'
    });
  } finally {
    isBackingUp.value = false;
  }
};
</script>

<template>
  <GlassCard>
    <div class="flex items-center gap-3 mb-6">
      <div class="p-3 rounded-xl bg-primary/20 text-primary">
        <span class="material-symbols-rounded text-2xl">backup</span>
      </div>
      <div>
        <h3 class="text-xl font-bold text-white">Backup Manager</h3>
        <p class="text-sm text-white/60">Save data before flashing</p>
      </div>
    </div>

    <div class="space-y-4 mb-6">
      <div class="flex items-center justify-between p-3 rounded-xl bg-white/5 border border-white/5">
        <div>
          <div class="text-sm text-white font-medium">Include APKs</div>
          <div class="text-xs text-white/50">Backup app installers (-apk)</div>
        </div>
        <ToggleSwitch v-model="options.apk" />
      </div>

      <div class="flex items-center justify-between p-3 rounded-xl bg-white/5 border border-white/5">
        <div>
          <div class="text-sm text-white font-medium">Shared Storage</div>
          <div class="text-xs text-white/50">SD Card contents (-shared)</div>
        </div>
        <ToggleSwitch v-model="options.shared" />
      </div>

      <div class="flex items-center justify-between p-3 rounded-xl bg-white/5 border border-white/5">
        <div>
          <div class="text-sm text-white font-medium">System Apps</div>
          <div class="text-xs text-white/50">Include system packages (-system)</div>
        </div>
        <ToggleSwitch v-model="options.system" />
      </div>
    </div>

    <div class="space-y-4">
      <div class="flex gap-2 items-end">
        <GlassInput 
          v-model="backupPath"
          label="Backup Location"
          placeholder="/path/to/backup.ab"
          icon="save"
          class="flex-1"
        />
        <VisionButton 
          @click="selectBackupLocation" 
          variant="secondary"
          icon="folder_open"
          class="mb-[2px]"
        >
          Browse
        </VisionButton>
      </div>

      <VisionButton 
        @click="startBackup" 
        :loading="isBackingUp"
        :disabled="!backupPath"
        icon="cloud_upload"
        class="w-full"
      >
        Start Backup
      </VisionButton>

      <div v-if="message" class="p-3 rounded-lg text-center text-xs font-bold" :class="message.includes('Error') ? 'bg-error/20 text-error' : 'bg-success/20 text-success'">
        {{ message }}
      </div>
    </div>
  </GlassCard>
</template>
