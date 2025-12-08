<script setup lang="ts">
import { ref, onMounted, watch, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useSystemStore } from '../../stores/system';
import { useNotificationStore } from '../../stores/notifications';

const systemStore = useSystemStore();
const notificationStore = useNotificationStore();
const hasPermission = ref(true);

// Only show if this is the active modal
const showModal = computed(() => systemStore.activeModal === 'permissions');

const check = async () => {
  // Wait for driver check to complete
  if (!systemStore.isDriverCheckComplete) return;
  // Don't interrupt if another modal is active
  if (systemStore.activeModal && systemStore.activeModal !== 'permissions') return;

  try {
    hasPermission.value = await invoke('check_permissions');
    if (!hasPermission.value) {
      systemStore.setActiveModal('permissions');
    } else {
       if (systemStore.activeModal === 'permissions') {
        systemStore.setActiveModal(null);
      }
      systemStore.setPermissionCheckComplete(true);
    }
  } catch (e) {
    console.error(e);
  }
};

const request = async () => {
  try {
    await invoke('request_permissions');
    hasPermission.value = true;
    systemStore.setActiveModal(null);
    systemStore.setPermissionCheckComplete(true);
    notificationStore.addNotification({
      title: 'Permissions Granted',
      message: 'System permissions acquired successfully.',
      type: 'success'
    });
  } catch (e) {
    console.error(e);
    notificationStore.addNotification({
      title: 'Permission Error',
      message: 'Failed to acquire permissions.',
      type: 'error'
    });
  }
};

// Watch for driver check completion
watch(() => systemStore.isDriverCheckComplete, (newValue) => {
  if (newValue) {
    check();
  }
});

// Also check on mount if already ready
onMounted(() => {
  if (systemStore.isDriverCheckComplete) {
    check();
  }
});
</script>

<template>
  <div v-if="showModal" class="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm">
    <div class="bg-surface/90 border border-white/10 rounded-2xl p-6 max-w-md w-full shadow-2xl backdrop-blur-xl">
      <h2 class="text-2xl font-bold text-white mb-4">Permissions Required</h2>
      <p class="text-gray-300 mb-6">
        CyberFlash needs additional permissions to communicate with your device.
      </p>
      <button @click="request" class="w-full py-3 bg-primary text-white font-bold rounded-xl hover:bg-primary-hover transition-colors">
        Grant Permissions
      </button>
    </div>
  </div>
</template>
