<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useSystemStore } from '../../stores/system';
import { useNotificationStore } from '../../stores/notifications';

interface DriverStatus {
  platform: string;
  adb_installed: boolean;
  fastboot_installed: boolean;
  udev_rules_ok: boolean;
  drivers_ok: boolean;
  message: string;
}

const systemStore = useSystemStore();
const notificationStore = useNotificationStore();
const status = ref<DriverStatus | null>(null);
const loading = ref(true);

// Only show if this is the active modal
const showModal = computed(() => systemStore.activeModal === 'drivers');

const checkDrivers = async () => {
  loading.value = true;
  try {
    status.value = await invoke<DriverStatus>('check_drivers');
    if (!status.value.adb_installed || !status.value.fastboot_installed || !status.value.udev_rules_ok || !status.value.drivers_ok) {
      systemStore.setActiveModal('drivers');
      notificationStore.addNotification({
        title: 'Driver Issues Found',
        message: 'Some components are missing or misconfigured.',
        type: 'warning'
      });
    } else {
      // If all good, ensure we don't block
      if (systemStore.activeModal === 'drivers') {
        systemStore.setActiveModal(null);
      }
    }
  } catch (e) {
    console.error(e);
    notificationStore.addNotification({
      title: 'Driver Check Failed',
      message: String(e),
      type: 'error'
    });
  } finally {
    loading.value = false;
    systemStore.setDriverCheckComplete(true);
  }
};

onMounted(() => {
  checkDrivers();
});

const close = () => {
  systemStore.setActiveModal(null);
};
</script>

<template>
  <div v-if="showModal" class="fixed inset-0 z-[60] flex items-center justify-center bg-black/80 backdrop-blur-sm">
    <div class="bg-surface border border-white/10 rounded-2xl p-6 max-w-md w-full shadow-2xl">
      <h2 class="text-2xl font-bold text-white mb-4 flex items-center gap-2">
        <span class="material-symbols-rounded text-warning">warning</span>
        System Check
      </h2>
      
      <div v-if="loading" class="text-center py-4">
        <span class="loading loading-spinner text-primary"></span>
      </div>
      
      <div v-else-if="status" class="space-y-4">
        <p class="text-gray-300">{{ status.message }}</p>
        
        <div class="bg-black/30 rounded-lg p-4 space-y-2">
          <div class="flex justify-between items-center">
            <span class="text-sm text-gray-400">Platform</span>
            <span class="text-sm font-mono text-white">{{ status.platform }}</span>
          </div>
          <div class="flex justify-between items-center">
            <span class="text-sm text-gray-400">ADB Binary</span>
            <span :class="status.adb_installed ? 'text-success' : 'text-error'" class="material-symbols-rounded text-sm">
              {{ status.adb_installed ? 'check_circle' : 'cancel' }}
            </span>
          </div>
          <div class="flex justify-between items-center">
            <span class="text-sm text-gray-400">Fastboot Binary</span>
            <span :class="status.fastboot_installed ? 'text-success' : 'text-error'" class="material-symbols-rounded text-sm">
              {{ status.fastboot_installed ? 'check_circle' : 'cancel' }}
            </span>
          </div>
          <div v-if="status.platform === 'linux'" class="flex justify-between items-center">
            <span class="text-sm text-gray-400">Udev Rules</span>
            <span :class="status.udev_rules_ok ? 'text-success' : 'text-error'" class="material-symbols-rounded text-sm">
              {{ status.udev_rules_ok ? 'check_circle' : 'cancel' }}
            </span>
          </div>
        </div>

        <div class="flex justify-end gap-2 mt-6">
          <button @click="checkDrivers" class="px-4 py-2 bg-white/10 hover:bg-white/20 rounded-lg text-white text-sm transition-colors">
            Re-check
          </button>
          <button @click="close" class="px-4 py-2 bg-primary hover:bg-primary-hover rounded-lg text-white text-sm font-bold transition-colors">
            Dismiss
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
