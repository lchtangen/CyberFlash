<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';

interface DeviceResult {
  serial: string;
  success: boolean;
  message: string;
}

interface BatchJob {
  id: string;
  device_serials: string[];
  action: string;
  status: string;
  results: DeviceResult[];
}

// Mock devices for demo
const devices = ref([
  { serial: 'DEVICE_001', model: 'OnePlus 7 Pro', status: 'online', selected: false },
  { serial: 'DEVICE_002', model: 'Pixel 6', status: 'online', selected: false },
  { serial: 'DEVICE_003', model: 'Xiaomi Mi 11', status: 'recovery', selected: false },
  { serial: 'DEVICE_004', model: 'Nothing Phone 1', status: 'bootloader', selected: false },
]);

const isProcessing = ref(false);
const lastJob = ref<BatchJob | null>(null);

const toggleSelectAll = () => {
  const allSelected = devices.value.every(d => d.selected);
  devices.value.forEach(d => d.selected = !allSelected);
};

const executeAction = async (action: string) => {
  const selectedSerials = devices.value.filter(d => d.selected).map(d => d.serial);
  if (selectedSerials.length === 0) return;

  isProcessing.value = true;
  lastJob.value = null;

  try {
    lastJob.value = await invoke<BatchJob>('execute_batch_action', { 
      serials: selectedSerials, 
      action 
    });
  } catch (e) {
    console.error(e);
  } finally {
    isProcessing.value = false;
  }
};
</script>

<template>
  <GlassCard>
    <div class="flex items-center gap-3 mb-6">
      <div class="p-3 rounded-xl bg-primary/20 text-primary">
        <span class="material-symbols-rounded text-2xl">grid_view</span>
      </div>
      <div>
        <h2 class="text-xl font-bold text-white">Multi-Device Grid</h2>
        <p class="text-sm text-white/60">Batch operations for device farms.</p>
      </div>
    </div>

    <div class="space-y-4">
      <!-- Toolbar -->
      <div class="flex gap-2 mb-4">
        <VisionButton size="sm" variant="secondary" @click="toggleSelectAll">Select All</VisionButton>
        <div class="w-px bg-white/10 mx-2"></div>
        <VisionButton size="sm" icon="restart_alt" @click="executeAction('reboot')" :disabled="isProcessing">Reboot</VisionButton>
        <VisionButton size="sm" icon="flash_on" @click="executeAction('flash')" :disabled="isProcessing">Flash</VisionButton>
        <VisionButton size="sm" icon="delete" variant="danger" @click="executeAction('wipe')" :disabled="isProcessing">Wipe</VisionButton>
      </div>

      <!-- Grid -->
      <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
        <div 
          v-for="device in devices" 
          :key="device.serial"
          class="p-4 rounded-xl border transition-all cursor-pointer relative overflow-hidden group"
          :class="device.selected ? 'bg-primary/20 border-primary' : 'bg-white/5 border-white/10 hover:bg-white/10'"
          @click="device.selected = !device.selected"
        >
          <div class="absolute top-2 right-2">
            <div class="w-4 h-4 rounded-full border border-white/20 flex items-center justify-center" :class="device.selected ? 'bg-primary border-primary' : 'bg-black/40'">
              <span v-if="device.selected" class="material-symbols-rounded text-[10px] text-white">check</span>
            </div>
          </div>

          <div class="flex flex-col items-center text-center py-2">
            <span class="material-symbols-rounded text-3xl mb-2" :class="{
              'text-success': device.status === 'online',
              'text-warning': device.status === 'recovery',
              'text-error': device.status === 'bootloader'
            }">smartphone</span>
            <div class="font-bold text-white text-sm truncate w-full">{{ device.model }}</div>
            <div class="text-xs text-white/40 font-mono truncate w-full">{{ device.serial }}</div>
            <div class="mt-2 text-[10px] px-2 py-0.5 rounded-full bg-white/10 text-white/60 uppercase tracking-wider">
              {{ device.status }}
            </div>
          </div>
        </div>
      </div>

      <!-- Results -->
      <div v-if="lastJob" class="mt-6 p-4 rounded-xl bg-black/20 border border-white/5">
        <h3 class="text-sm font-bold text-white mb-3">Batch Results</h3>
        <div class="space-y-2">
          <div v-for="res in lastJob.results" :key="res.serial" class="flex items-center justify-between text-sm">
            <span class="text-white/60">{{ res.serial }}</span>
            <span :class="res.success ? 'text-success' : 'text-error'">{{ res.message }}</span>
          </div>
        </div>
      </div>
    </div>
  </GlassCard>
</template>
