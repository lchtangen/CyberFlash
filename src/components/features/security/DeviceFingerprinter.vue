<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useNotificationStore } from '../../../stores/notifications';
import VisionButton from '../../ui/VisionButton.vue';

const props = ref<Record<string, string>>({});
const loading = ref(false);
const error = ref<string | null>(null);
const notificationStore = useNotificationStore();

const isUnauthorized = computed(() => error.value?.includes('unauthorized') || error.value?.includes('$ADB_VENDOR_KEYS'));

const fetchProps = async () => {
  loading.value = true;
  error.value = null;
  try {
    props.value = await invoke('get_device_props', { serial: null });
    notificationStore.addNotification({
      title: 'Device Scanned',
      message: 'Device fingerprint retrieved successfully.',
      type: 'success',
      duration: 3000
    });
  } catch (e) {
    const errMsg = String(e);
    error.value = errMsg;
    
    // Check for unauthorized error
    if (errMsg.includes('unauthorized') || errMsg.includes('$ADB_VENDOR_KEYS')) {
       notificationStore.addNotification({
        title: 'Authorization Required',
        message: 'Please check your phone screen and allow USB Debugging.',
        type: 'warning',
        duration: 8000
      });
    } else {
      notificationStore.addNotification({
        title: 'Scan Failed',
        message: errMsg,
        type: 'error',
        duration: 5000
      });
    }
  } finally {
    loading.value = false;
  }
};

const restartAdb = async () => {
  loading.value = true;
  try {
    await invoke('kill_server');
    notificationStore.addNotification({
      title: 'ADB Restarted',
      message: 'ADB Server killed. Please retry scanning.',
      type: 'info'
    });
    // Wait a bit for server to die
    setTimeout(fetchProps, 2000);
  } catch (e) {
    notificationStore.addNotification({
      title: 'Restart Failed',
      message: String(e),
      type: 'error'
    });
    loading.value = false;
  }
};

onMounted(fetchProps);

const importantKeys = [
  { key: 'ro.product.model', label: 'Model' },
  { key: 'ro.product.device', label: 'Device Code' },
  { key: 'ro.build.version.release', label: 'Android Version' },
  { key: 'ro.build.id', label: 'Build ID' },
  { key: 'ro.bootloader', label: 'Bootloader' },
];
</script>

<template>
  <div class="bg-surface/30 border border-white/10 rounded-xl p-4 backdrop-blur-md">
    <div class="flex justify-between items-center mb-4">
      <h3 class="text-lg font-semibold text-white flex items-center gap-2">
        <span class="material-symbols-rounded text-primary">fingerprint</span>
        Device Fingerprint
      </h3>
      <VisionButton 
        @click="fetchProps" 
        :loading="loading"
        icon="radar"
        variant="primary"
        size="sm"
        class="shadow-lg shadow-primary/20 hover:shadow-primary/40 transition-all duration-300"
      >
        Scan Device
      </VisionButton>
    </div>

    <div v-if="error" class="mb-4">
      <!-- Unauthorized State -->
      <div v-if="isUnauthorized" class="p-4 bg-warning/10 border border-warning/20 rounded-xl flex flex-col gap-3">
        <div class="flex items-center gap-2 text-warning">
          <span class="material-symbols-rounded">lock</span>
          <span class="font-bold">Device Unauthorized</span>
        </div>
        <p class="text-sm text-gray-300 leading-relaxed">
          Your device rejected the connection. Please check your phone screen and tap 
          <strong class="text-white">Allow</strong> on the "Allow USB Debugging?" prompt.
        </p>
        <button 
          @click="restartAdb"
          class="self-start px-4 py-2 bg-warning/20 hover:bg-warning/30 text-warning rounded-lg text-sm font-bold transition-colors flex items-center gap-2"
        >
          <span class="material-symbols-rounded text-lg">refresh</span>
          Restart ADB & Retry
        </button>
      </div>

      <!-- Generic Error -->
      <div v-else class="p-3 bg-error/10 border border-error/20 rounded-lg text-error text-sm">
        {{ error }}
      </div>
    </div>

    <div v-if="loading" class="flex justify-center py-4">
      <div class="w-6 h-6 border-2 border-primary border-t-transparent rounded-full animate-spin"></div>
    </div>

    <div v-else class="space-y-3">
      <div v-for="item in importantKeys" :key="item.key" class="flex justify-between items-center border-b border-white/5 pb-2 last:border-0">
        <span class="text-text-secondary text-sm">{{ item.label }}</span>
        <span class="text-white font-mono text-sm">{{ props[item.key] || 'N/A' }}</span>
      </div>
      
      <div class="mt-4 pt-2 border-t border-white/10">
        <div class="text-xs text-text-muted uppercase tracking-wider mb-1">Full Fingerprint</div>
        <div class="text-[10px] text-text-secondary font-mono break-all leading-tight">
          {{ props['ro.build.fingerprint'] || 'Unknown' }}
        </div>
      </div>
    </div>
  </div>
</template>
