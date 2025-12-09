<template>
  <GlassCard class="relative overflow-hidden">
    <div class="flex justify-between items-center mb-6 relative z-10">
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-xl bg-primary/10 flex items-center justify-center border border-primary/20 shadow-[0_0_15px_rgba(10,132,255,0.1)]">
          <span class="material-symbols-rounded text-primary text-xl">usb</span>
        </div>
        <div>
          <h3 class="text-lg font-bold text-white tracking-wide">Cable Tester</h3>
          <p class="text-xs text-text-secondary font-medium">Data transfer integrity check</p>
        </div>
      </div>
      <VisionButton size="sm" @click="runTest" :loading="testing" :disabled="testing">
        Start Test
      </VisionButton>
    </div>

    <div v-if="!result && !testing" class="text-center py-12 text-white/40 relative z-10">
      <div class="w-20 h-20 mx-auto mb-4 rounded-full bg-white/5 flex items-center justify-center border border-white/5">
        <span class="material-symbols-rounded text-4xl opacity-50">cable</span>
      </div>
      <p class="text-sm font-medium">Connect device and click Start to test USB cable quality.</p>
    </div>

    <div v-else-if="testing" class="py-12 space-y-6 relative z-10">
      <div class="flex justify-center relative">
        <div class="absolute inset-0 bg-primary/20 blur-xl rounded-full animate-pulse"></div>
        <div class="relative w-24 h-24">
          <svg class="animate-spin w-full h-full text-primary" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
        </div>
      </div>
      <div class="text-center">
        <p class="text-white font-bold tracking-wide animate-pulse">Analyzing Data Packets...</p>
        <p class="text-xs text-text-secondary mt-1">Do not disconnect device</p>
      </div>
    </div>

    <div v-else-if="result" class="space-y-6 animate-fade-in relative z-10">
      <div class="text-center p-6 rounded-2xl bg-white/5 border border-white/10 backdrop-blur-md">
        <div class="text-xs text-text-secondary uppercase tracking-wider font-bold mb-2">Signal Quality</div>
        <div class="text-4xl font-bold text-white drop-shadow-glow" :class="{
          'text-success': result.quality_rating === 'Excellent',
          'text-warning': result.quality_rating === 'Good',
          'text-error': result.quality_rating === 'Poor'
        }">{{ result.quality_rating }}</div>
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div class="space-y-2 p-4 rounded-xl bg-white/5 border border-white/5 hover:bg-white/10 transition-colors">
          <div class="flex items-center gap-2 text-xs text-text-secondary uppercase tracking-wider font-bold">
            <span class="material-symbols-rounded text-sm">download</span>
            Read Speed
          </div>
          <div class="text-2xl font-mono text-white">{{ result.read_speed_mbps.toFixed(1) }} <span class="text-xs text-white/50">MB/s</span></div>
          <div class="h-1.5 bg-white/10 rounded-full overflow-hidden">
            <div class="h-full bg-primary transition-all duration-1000" :style="{ width: Math.min(result.read_speed_mbps / 10, 100) + '%' }"></div>
          </div>
        </div>
        
        <div class="space-y-2 p-4 rounded-xl bg-white/5 border border-white/5 hover:bg-white/10 transition-colors">
          <div class="flex items-center gap-2 text-xs text-text-secondary uppercase tracking-wider font-bold">
            <span class="material-symbols-rounded text-sm">upload</span>
            Write Speed
          </div>
          <div class="text-2xl font-mono text-white">{{ result.write_speed_mbps.toFixed(1) }} <span class="text-xs text-white/50">MB/s</span></div>
          <div class="h-1.5 bg-white/10 rounded-full overflow-hidden">
            <div class="h-full bg-secondary transition-all duration-1000" :style="{ width: Math.min(result.write_speed_mbps / 10, 100) + '%' }"></div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Background Mesh -->
    <div class="absolute inset-0 mesh-gradient-bg opacity-10 pointer-events-none"></div>
  </GlassCard>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';

interface CableTestResult {
  write_speed_mbps: number;
  read_speed_mbps: number;
  quality_rating: string;
}

const result = ref<CableTestResult | null>(null);
const testing = ref(false);

const runTest = async () => {
  testing.value = true;
  result.value = null;
  try {
    result.value = await invoke('test_cable_speed');
  } catch (e) {
    console.error(e);
  } finally {
    testing.value = false;
  }
};
</script>
