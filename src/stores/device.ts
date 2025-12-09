import { defineStore } from 'pinia';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export const useDeviceStore = defineStore('device', () => {
  const isConnected = ref(false);
  const deviceModel = ref('');
  const serial = ref('');
  const batteryLevel = ref(0);
  const connectionType = ref<'adb' | 'fastboot' | null>(null);
  const isBootloaderUnlocked = ref(false);

  function setConnected(status: boolean) {
    isConnected.value = status;
  }

  function setDeviceDetails(model: string, battery: number, serialId: string = '', unlocked: boolean = false) {
    deviceModel.value = model;
    batteryLevel.value = battery;
    if (serialId) serial.value = serialId;
    isBootloaderUnlocked.value = unlocked;
  }

  async function scanDevices() {
    try {
      const devices = await invoke<string[]>('get_connected_devices');
      if (devices && devices.length > 0) {
        isConnected.value = true;
        serial.value = devices[0];
        connectionType.value = 'adb';
        // Ideally fetch more info here
      } else {
        isConnected.value = false;
        serial.value = '';
        connectionType.value = null;
      }
    } catch (e) {
      console.error('Failed to scan devices:', e);
      isConnected.value = false;
    }
  }

  return {
    isConnected,
    deviceModel,
    serial,
    batteryLevel,
    connectionType,
    isBootloaderUnlocked,
    setConnected,
    setDeviceDetails,
    scanDevices
  };
});
