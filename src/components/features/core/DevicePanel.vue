<script setup lang="ts">
import { computed, watch } from 'vue';
import { useDeviceStore } from '../../../stores/device';
import { useNotificationStore } from '../../../stores/notifications';
import GlassCard from '../../ui/GlassCard.vue';

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
</script>

<template>
  <GlassCard class="relative overflow-hidden min-h-[300px] flex flex-col">
    <div class="flex flex-col h-full justify-between">
      <!-- Header -->
      <div class="flex items-center justify-between mb-6">
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-xl bg-white/5 border border-white/10">
            <span class="material-symbols-rounded text-white">smartphone</span>
          </div>
          <div>
            <h3 class="text-lg font-bold text-white tracking-tight leading-none">Device Status</h3>
            <p class="text-[10px] text-text-secondary font-mono uppercase tracking-widest mt-1">Real-time Telemetry</p>
          </div>
        </div>
        
        <div 
          class="flex items-center gap-2 px-3 py-1.5 rounded-full border backdrop-blur-md transition-all duration-300"
          :class="isConnected ? 'bg-success/10 border-success/20' : 'bg-error/10 border-error/20'"
        >
          <div 
            class="w-2 h-2 rounded-full"
            :class="isConnected ? 'bg-success animate-pulse' : 'bg-error'"
          ></div>
          <span 
            class="text-xs font-bold uppercase tracking-wide"
            :class="isConnected ? 'text-success' : 'text-error'"
          >
            {{ statusText }}
          </span>
        </div>
      </div>

      <!-- Stats Grid -->
      <div class="grid grid-cols-2 gap-4">
        <!-- Model Info -->
        <div class="col-span-2 p-4 rounded-xl bg-white/5 border border-white/5">
          <p class="text-[10px] text-text-secondary uppercase tracking-wider font-bold mb-1">Connected Model</p>
          <p class="text-2xl font-bold text-white tracking-tight truncate">{{ deviceStore.deviceModel || 'No Device' }}</p>
          <div class="flex items-center gap-2 mt-2">
            <span class="px-2 py-0.5 rounded text-[10px] font-mono bg-white/10 text-white/70 border border-white/5">{{ deviceStore.connectionType || 'N/A' }}</span>
            <span class="px-2 py-0.5 rounded text-[10px] font-mono bg-primary/10 text-primary border border-primary/20" v-if="isConnected">ADB Authorized</span>
          </div>
        </div>

        <!-- Battery -->
        <div class="p-4 rounded-xl bg-white/5 border border-white/5">
          <div class="flex justify-between items-start mb-2">
            <p class="text-[10px] text-text-secondary uppercase tracking-wider font-bold">Battery</p>
            <span class="material-symbols-rounded text-lg text-text-secondary">battery_std</span>
          </div>
          <div class="flex items-end gap-1">
            <span class="text-2xl font-bold text-white font-mono">{{ deviceStore.batteryLevel }}</span>
            <span class="text-sm text-text-secondary mb-1">%</span>
          </div>
          <div class="w-full h-1 bg-white/10 rounded-full mt-3 overflow-hidden">
            <div class="h-full bg-success transition-all duration-1000" :style="{ width: deviceStore.batteryLevel + '%' }"></div>
          </div>
        </div>

        <!-- Serial -->
        <div class="p-4 rounded-xl bg-white/5 border border-white/5">
          <div class="flex justify-between items-start mb-2">
            <p class="text-[10px] text-text-secondary uppercase tracking-wider font-bold">Serial ID</p>
            <span class="material-symbols-rounded text-lg text-text-secondary">fingerprint</span>
          </div>
          <p class="text-sm font-mono text-white/90 break-all leading-tight">{{ deviceStore.serial || 'Unknown' }}</p>
          <p class="text-[10px] text-text-muted mt-2">Hardware ID</p>
        </div>
      </div>
    </div>
  </GlassCard>
</template>
