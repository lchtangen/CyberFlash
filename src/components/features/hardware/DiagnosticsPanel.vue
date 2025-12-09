<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '@/components/ui/GlassCard.vue';
import VisionButton from '@/components/ui/VisionButton.vue';

const healthScore = ref<any>(null);
const touchEnabled = ref(false);

const loadHealth = async () => {
  try {
    healthScore.value = await invoke('calculate_device_health_score');
  } catch (e) {
    console.error(e);
  }
};

const runScreenTest = async () => {
  await invoke('run_screen_test');
};

const toggleTouch = async () => {
  touchEnabled.value = !touchEnabled.value;
  await invoke('toggle_touch_test', { enable: touchEnabled.value });
};

const exportReport = async () => {
  try {
    const html = await invoke('generate_health_report');
    // Save to file
    // For now, just log or show success
    console.log("Report generated");
    // In a real app, we'd use the save dialog
    const blob = new Blob([html as string], { type: 'text/html' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'device_health_report.html';
    a.click();
  } catch (e) {
    console.error(e);
  }
};

onMounted(() => {
  loadHealth();
});
</script>

<template>
  <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
    <!-- Health Score Card -->
    <GlassCard class="relative overflow-hidden">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-lg font-bold text-white">Device Health</h3>
        <span class="material-symbols-rounded text-success">health_and_safety</span>
      </div>
      
      <div v-if="healthScore" class="flex flex-col items-center justify-center py-4">
        <div class="relative w-32 h-32 flex items-center justify-center mb-4">
          <svg class="w-full h-full -rotate-90" viewBox="0 0 36 36">
            <path class="text-white/10" d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831" fill="none" stroke="currentColor" stroke-width="3" />
            <path class="text-success drop-shadow-[0_0_10px_rgba(48,209,88,0.5)]" :stroke-dasharray="`${healthScore.score}, 100`" d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831" fill="none" stroke="currentColor" stroke-width="3" />
          </svg>
          <div class="absolute text-3xl font-bold text-white">{{ healthScore.score }}</div>
        </div>
        
        <div class="w-full space-y-2">
          <div class="flex justify-between text-sm">
            <span class="text-white/60">Battery</span>
            <span :class="healthScore.battery_health === 'Good' ? 'text-success' : 'text-warning'">{{ healthScore.battery_health }}</span>
          </div>
          <div class="flex justify-between text-sm">
            <span class="text-white/60">Storage</span>
            <span :class="healthScore.storage_health === 'Good' ? 'text-success' : 'text-warning'">{{ healthScore.storage_health }}</span>
          </div>
        </div>
        
        <p class="mt-4 text-xs text-center text-white/50">{{ healthScore.recommendation }}</p>
      </div>
      
      <div v-else class="flex items-center justify-center h-48">
        <span class="animate-spin material-symbols-rounded text-primary">refresh</span>
      </div>
    </GlassCard>

    <!-- Tools Card -->
    <GlassCard>
      <div class="flex items-center justify-between mb-6">
        <h3 class="text-lg font-bold text-white">Diagnostics Tools</h3>
        <span class="material-symbols-rounded text-primary">build</span>
      </div>
      
      <div class="space-y-4">
        <VisionButton @click="runScreenTest" variant="secondary" class="w-full justify-start">
          <span class="material-symbols-rounded mr-2">display_settings</span>
          Run Screen Test (Dead Pixels)
        </VisionButton>
        
        <VisionButton @click="toggleTouch" :variant="touchEnabled ? 'primary' : 'secondary'" class="w-full justify-start">
          <span class="material-symbols-rounded mr-2">touch_app</span>
          {{ touchEnabled ? 'Disable' : 'Enable' }} Touch Visualization
        </VisionButton>
        
        <VisionButton @click="exportReport" variant="secondary" class="w-full justify-start">
          <span class="material-symbols-rounded mr-2">assignment</span>
          Export Health Report
        </VisionButton>
      </div>
    </GlassCard>
  </div>
</template>
