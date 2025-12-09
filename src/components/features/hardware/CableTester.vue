<template>
  <GlassCard>
    <div class="flex justify-between items-center mb-6">
      <h3 class="text-lg font-bold text-white flex items-center gap-2">
        <i class="fas fa-usb text-primary"></i>
        Cable Tester
      </h3>
      <VisionButton size="sm" @click="runTest" :loading="testing" :disabled="testing">
        Start Test
      </VisionButton>
    </div>

    <div v-if="!result && !testing" class="text-center py-8 text-white/40">
      <i class="fas fa-plug text-4xl mb-3"></i>
      <p>Connect device and click Start to test USB cable quality.</p>
    </div>

    <div v-else-if="testing" class="py-8 space-y-4">
      <div class="flex justify-center">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary"></div>
      </div>
      <p class="text-center text-white/60 animate-pulse">Transferring data packets...</p>
    </div>

    <div v-else-if="result" class="space-y-6 animate-fade-in">
      <div class="text-center p-4 rounded-xl bg-white/5 border border-white/10">
        <div class="text-sm text-white/50 uppercase tracking-wider mb-1">Quality Rating</div>
        <div class="text-2xl font-bold text-success">{{ result.quality_rating }}</div>
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div class="space-y-1">
          <div class="text-xs text-white/50">Read Speed</div>
          <div class="h-2 bg-white/10 rounded-full overflow-hidden">
            <div class="h-full bg-primary transition-all duration-1000" :style="{ width: Math.min(result.read_speed_mbps / 10, 100) + '%' }"></div>
          </div>
          <div class="text-right font-mono text-sm text-white">{{ result.read_speed_mbps.toFixed(1) }} MB/s</div>
        </div>
        <div class="space-y-1">
          <div class="text-xs text-white/50">Write Speed</div>
          <div class="h-2 bg-white/10 rounded-full overflow-hidden">
            <div class="h-full bg-secondary transition-all duration-1000" :style="{ width: Math.min(result.write_speed_mbps / 10, 100) + '%' }"></div>
          </div>
          <div class="text-right font-mono text-sm text-white">{{ result.write_speed_mbps.toFixed(1) }} MB/s</div>
        </div>
      </div>
    </div>
  </GlassCard>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '@/components/ui/GlassCard.vue';
import VisionButton from '@/components/ui/VisionButton.vue';

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
