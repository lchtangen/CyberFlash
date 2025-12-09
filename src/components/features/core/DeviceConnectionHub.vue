<script setup lang="ts">
import { onMounted, onUnmounted, ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useDeviceStore } from '../../../stores/device';

const deviceStore = useDeviceStore();
const unlisten = ref<() => void>();

interface DeviceStatus {
  serial: string;
  state: string;
  connection_type: 'adb' | 'fastboot';
}

const statusColor = computed(() => {
  if (!deviceStore.isConnected) return 'bg-error';
  if (deviceStore.connectionType === 'fastboot') return 'bg-warning';
  if (deviceStore.connectionType === 'adb') return 'bg-success';
  return 'bg-text-secondary';
});

const statusText = computed(() => {
  if (!deviceStore.isConnected) return 'Disconnected';
  if (deviceStore.connectionType === 'fastboot') return 'Fastboot Mode';
  if (deviceStore.connectionType === 'adb') return 'ADB Connected';
  return 'Unknown';
});

const statusGlow = computed(() => {
  if (!deviceStore.isConnected) return 'shadow-error/20';
  if (deviceStore.connectionType === 'fastboot') return 'shadow-warning/20';
  if (deviceStore.connectionType === 'adb') return 'shadow-success/20';
  return '';
});

onMounted(async () => {
  // Start the backend monitor
  await invoke('start_usb_monitor');

  // Listen for updates
  unlisten.value = await listen<DeviceStatus[]>('device-status-update', (event) => {
    const devices = event.payload;
    
    if (devices.length > 0) {
      const activeDevice = devices[0]; // Focus on the first device for now
      deviceStore.setConnected(true);
      deviceStore.connectionType = activeDevice.connection_type;
      deviceStore.deviceModel = activeDevice.serial; // Temporary until we parse model
    } else {
      deviceStore.setConnected(false);
      deviceStore.connectionType = null;
    }
  });
});

onUnmounted(() => {
  if (unlisten.value) {
    unlisten.value();
  }
});
</script>

<template>
  <div 
    class="flex items-center gap-3 px-4 py-2 rounded-xl bg-surface/30 border border-white/10 backdrop-blur-md transition-all duration-300 hover:bg-surface/50 shadow-lg"
    :class="statusGlow"
  >
    <!-- Status Indicator -->
    <div class="relative flex h-3 w-3">
      <span 
        v-if="deviceStore.isConnected"
        class="animate-ping absolute inline-flex h-full w-full rounded-full opacity-75"
        :class="statusColor"
      ></span>
      <span 
        class="relative inline-flex rounded-full h-3 w-3 transition-colors duration-300"
        :class="statusColor"
      ></span>
    </div>

    <!-- Text Info -->
    <div class="flex flex-col">
      <span class="text-xs font-bold tracking-wide text-white uppercase transition-all duration-300">
        {{ statusText }}
      </span>
      <span v-if="deviceStore.isConnected" class="text-[10px] text-text-secondary font-mono truncate max-w-[100px]">
        {{ deviceStore.deviceModel }}
      </span>
      <span v-else class="text-[10px] text-text-secondary font-mono">
        Waiting for device...
      </span>
    </div>
    
    <!-- Icon -->
    <div class="ml-2 text-text-secondary">
      <span v-if="deviceStore.connectionType === 'adb'" class="material-symbols-rounded text-lg">adb</span>
      <span v-else-if="deviceStore.connectionType === 'fastboot'" class="material-symbols-rounded text-lg">electric_bolt</span>
      <span v-else class="material-symbols-rounded text-lg">usb_off</span>
    </div>
  </div>
</template>
