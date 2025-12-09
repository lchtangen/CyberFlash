import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useDeviceStore = defineStore('device', () => {
  const isConnected = ref(false);
  const deviceModel = ref('');
  const serial = ref('');
  const batteryLevel = ref(0);
  const connectionType = ref<'adb' | 'fastboot' | null>(null);

  function setConnected(status: boolean) {
    isConnected.value = status;
  }

  function setDeviceDetails(model: string, battery: number, serialId: string = '') {
    deviceModel.value = model;
    batteryLevel.value = battery;
    if (serialId) serial.value = serialId;
  }

  return {
    isConnected,
    deviceModel,
    serial,
    batteryLevel,
    connectionType,
    setConnected,
    setDeviceDetails
  };
});
