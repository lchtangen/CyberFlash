import { defineStore } from 'pinia';
import { ref, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';

export const useDeviceStore = defineStore('device', () => {
  const isConnected = ref(false);
  const deviceModel = ref('');
  const serial = ref('');
  const batteryLevel = ref(0);
  const connectionType = ref<'adb' | 'fastboot' | null>(null);
  const isBootloaderUnlocked = ref(false);

  // Session Resume: Load from localStorage
  const savedSerial = localStorage.getItem('last_device_serial');
  if (savedSerial) {
    serial.value = savedSerial;
  }

  // Session Resume: Save to localStorage
  watch(serial, (newSerial) => {
    if (newSerial) {
      localStorage.setItem('last_device_serial', newSerial);
    }
  });

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
        
        // Fetch details
        try {
          const model = await invoke<string>('get_prop_value', { prop: 'ro.product.model' });
          deviceModel.value = model.trim();
        } catch {
          deviceModel.value = 'Unknown Device';
        }

        try {
          const battery = await invoke<number>('check_battery_level');
          batteryLevel.value = battery;
        } catch {
          batteryLevel.value = 0;
        }

        try {
          const unlocked = await invoke<boolean>('check_bootloader_unlocked');
          isBootloaderUnlocked.value = unlocked;
        } catch {
          isBootloaderUnlocked.value = false;
        }

      } else {
        // Check Fastboot
        const fastbootDevices = await invoke<string[]>('get_fastboot_devices');
        if (fastbootDevices && fastbootDevices.length > 0) {
            isConnected.value = true;
            serial.value = fastbootDevices[0];
            connectionType.value = 'fastboot';
            deviceModel.value = 'Fastboot Device';
            batteryLevel.value = 0; // Cannot read battery in fastboot easily without getvar
            
            try {
                const vars = await invoke<any>('get_var_all', { serial: fastbootDevices[0] });
                if (vars['product']) deviceModel.value = vars['product'];
                if (vars['unlocked'] === 'yes') isBootloaderUnlocked.value = true;
            } catch {}
        } else {
            isConnected.value = false;
            serial.value = '';
            connectionType.value = null;
        }
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
