<script setup lang="ts">
import { computed, watch } from 'vue';
import { useDeviceStore } from '../../../stores/device';
import { useNotificationStore } from '../../../stores/notifications';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';

const deviceStore = useDeviceStore();
const notificationStore = useNotificationStore();

const isConnected = computed(() => deviceStore.isConnected);
const statusText = computed(() => isConnected.value ? 'Online' : 'Offline');

watch(isConnected, (newVal) => {
  if (newVal) {
    notificationStore.addNotification({
      title: 'Device Connected',
      message: `${deviceStore.deviceModel || 'Device'} is now online.`,
      type: 'success'
    });
  } else {
    notificationStore.addNotification({
      title: 'Device Disconnected',
      message: 'Connection lost.',
      type: 'error'
    });
  }
});

const reboot = async (mode: string) => {
  if (!isConnected.value) return;
  try {
    await invoke('reboot_device', { mode });
    notificationStore.addNotification({
      title: 'Reboot Initiated',
      message: `Rebooting to ${mode}...`,
      type: 'success'
    });
  } catch (e) {
    notificationStore.addNotification({
      title: 'Reboot Failed',
      message: String(e),
      type: 'error'
    });
  }
};
</script>

<template>
  <GlassCard class="relative overflow-hidden min-h-[320px] flex flex-col group">
    <!-- Background Effects -->
    <div v-if="isConnected" class="absolute inset-0 bg-gradient-to-br from-primary/10 via-transparent to-transparent opacity-50 transition-opacity duration-500"></div>
    <div v-else class="absolute inset-0 bg-[radial-gradient(circle_at_center,_var(--tw-gradient-stops))] from-white/5 via-transparent to-transparent opacity-20 animate-pulse"></div>

    <div class="relative z-10 flex flex-col h-full justify-between">
      <div>
        <!-- Header -->
        <div class="flex items-center justify-between mb-6">
          <div class="flex items-center gap-4">
            <div class="relative">
              <div class="p-3 rounded-2xl bg-white/5 border border-white/10 backdrop-blur-md shadow-inner">
                <span class="material-symbols-rounded text-2xl text-white" :class="{'text-primary drop-shadow-[0_0_8px_rgba(10,132,255,0.5)]': isConnected}">smartphone</span>
              </div>
              <!-- Status Dot Badge -->
              <div class="absolute -bottom-1 -right-1 w-4 h-4 rounded-full border-2 border-[#1C1C1E] flex items-center justify-center" :class="isConnected ? 'bg-success' : 'bg-surface'">
                 <div v-if="!isConnected" class="w-1.5 h-1.5 rounded-full bg-white/30"></div>
                 <span v-else class="material-symbols-rounded text-[10px] text-black font-bold">check</span>
              </div>
            </div>
            <div>
              <h3 class="text-xl font-bold text-white tracking-tight leading-none">Device Status</h3>
              <div class="flex items-center gap-2 mt-1.5">
                <span class="w-1.5 h-1.5 rounded-full" :class="isConnected ? 'bg-success animate-pulse' : 'bg-error'"></span>
                <p class="text-xs font-medium tracking-wide" :class="isConnected ? 'text-success' : 'text-text-secondary'">
                  {{ isConnected ? 'System Online' : 'Searching for Device...' }}
                </p>
              </div>
            </div>
          </div>
          
          <div v-if="isConnected" class="px-3 py-1 rounded-lg bg-white/5 border border-white/10 backdrop-blur-md">
             <span class="text-[10px] font-mono text-white/50">ADB: AUTH</span>
          </div>
        </div>

        <!-- Stats Grid -->
        <div v-if="isConnected" class="grid grid-cols-2 gap-3 animate-in fade-in slide-in-from-bottom-4 duration-500">
          <!-- Model Info -->
          <div class="col-span-2 p-4 rounded-2xl bg-gradient-to-br from-white/10 to-white/5 border border-white/10 shadow-lg backdrop-blur-sm">
            <div class="flex justify-between items-start">
              <div>
                <p class="text-[10px] text-primary font-bold uppercase tracking-widest mb-1">Connected Model</p>
                <p class="text-2xl font-bold text-white tracking-tight truncate drop-shadow-md">{{ deviceStore.deviceModel || 'Unknown Model' }}</p>
              </div>
              <span class="material-symbols-rounded text-white/20 text-3xl">devices</span>
            </div>
            <div class="mt-3 flex items-center gap-2">
               <div class="h-1.5 w-1.5 rounded-full bg-primary"></div>
               <span class="text-xs text-white/70 font-mono">{{ deviceStore.serial || 'UNKNOWN_SERIAL' }}</span>
            </div>
          </div>

          <!-- Battery -->
          <div class="p-4 rounded-2xl bg-white/5 border border-white/5 hover:bg-white/10 transition-colors group/battery">
            <div class="flex justify-between items-start mb-3">
              <p class="text-[10px] text-text-secondary uppercase tracking-wider font-bold">Power</p>
              <span class="material-symbols-rounded text-lg text-text-secondary group-hover/battery:text-success transition-colors">battery_std</span>
            </div>
            <div class="relative pt-2">
               <div class="flex items-baseline gap-1">
                  <span class="text-3xl font-bold text-white font-mono tracking-tighter">{{ deviceStore.batteryLevel }}</span>
                  <span class="text-sm text-text-secondary">%</span>
               </div>
               <!-- Progress Bar -->
               <div class="w-full h-1.5 bg-white/10 rounded-full mt-3 overflow-hidden">
                  <div 
                    class="h-full rounded-full transition-all duration-1000 relative overflow-hidden" 
                    :class="deviceStore.batteryLevel > 20 ? 'bg-success' : 'bg-error'"
                    :style="{ width: deviceStore.batteryLevel + '%' }"
                  >
                    <div class="absolute inset-0 bg-white/30 w-full h-full animate-[shimmer_2s_infinite]"></div>
                  </div>
               </div>
            </div>
          </div>

          <!-- Connection Type -->
          <div class="p-4 rounded-2xl bg-white/5 border border-white/5 hover:bg-white/10 transition-colors">
            <div class="flex justify-between items-start mb-3">
              <p class="text-[10px] text-text-secondary uppercase tracking-wider font-bold">Link</p>
              <span class="material-symbols-rounded text-lg text-text-secondary">usb</span>
            </div>
            <div class="pt-2">
               <p class="text-lg font-bold text-white capitalize">{{ deviceStore.connectionType || 'USB' }}</p>
               <p class="text-[10px] text-text-muted mt-1">High Speed</p>
            </div>
          </div>
        </div>

        <!-- Disconnected State -->
        <div v-else class="flex flex-col items-center justify-center py-8 text-center animate-in fade-in zoom-in-95 duration-300">
           <div class="relative w-24 h-24 mb-4">
              <div class="absolute inset-0 rounded-full border-2 border-primary/20 animate-[ping_3s_linear_infinite]"></div>
              <div class="absolute inset-4 rounded-full border border-primary/40 animate-[ping_3s_linear_infinite_1s]"></div>
              <div class="absolute inset-0 flex items-center justify-center">
                 <span class="material-symbols-rounded text-4xl text-white/20">radar</span>
              </div>
           </div>
           <p class="text-white font-medium">Waiting for connection...</p>
           <p class="text-xs text-text-secondary mt-1 max-w-[200px]">Connect your device via USB and ensure USB Debugging is enabled.</p>
        </div>
      </div>

      <!-- Actions -->
      <div v-if="isConnected" class="grid grid-cols-3 gap-3 mt-6 pt-6 border-t border-white/5">
         <VisionButton size="sm" variant="secondary" @click="reboot('system')" icon="restart_alt" class="w-full justify-center">System</VisionButton>
         <VisionButton size="sm" variant="secondary" @click="reboot('recovery')" icon="build" class="w-full justify-center">Recovery</VisionButton>
         <VisionButton size="sm" variant="secondary" @click="reboot('bootloader')" icon="developer_board" class="w-full justify-center">Bootloader</VisionButton>
      </div>
      <div v-else class="mt-6 pt-6 border-t border-white/5">
          <VisionButton size="sm" variant="primary" class="w-full justify-center" icon="refresh" @click="deviceStore.scanDevices()">Scan for Devices</VisionButton>
      </div>
    </div>
  </GlassCard>
</template>
