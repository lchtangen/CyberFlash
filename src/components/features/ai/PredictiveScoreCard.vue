<template>
  <div class="relative overflow-hidden rounded-2xl border border-white/10 bg-surface/30 backdrop-blur-xl transition-all duration-300 hover:bg-surface/40 p-6">
    <div class="flex items-start justify-between mb-4">
      <div>
        <h3 class="text-lg font-semibold text-white flex items-center gap-2">
          <span class="i-lucide-brain-circuit text-primary"></span>
          AI Success Prediction
        </h3>
        <p class="text-sm text-white/60">Pre-flight analysis based on device state</p>
      </div>
      <div class="flex flex-col items-end">
        <div class="text-3xl font-bold" :class="scoreColorClass">{{ score }}%</div>
        <div class="text-xs font-medium px-2 py-0.5 rounded-full bg-white/5 border border-white/10" :class="scoreColorClass">
          {{ riskLevel }} Risk
        </div>
      </div>
    </div>

    <!-- Progress Bar -->
    <div class="h-2 w-full bg-white/10 rounded-full overflow-hidden mb-6">
      <div 
        class="h-full transition-all duration-1000 ease-out rounded-full relative overflow-hidden"
        :class="progressBarClass"
        :style="{ width: `${score}%` }"
      >
        <div class="absolute inset-0 bg-white/20 animate-pulse-slow"></div>
      </div>
    </div>

    <!-- Analysis Factors -->
    <div class="space-y-3">
      <div v-for="(factor, index) in factors" :key="index" class="flex items-start gap-3 p-3 rounded-xl bg-white/5 border border-white/5">
        <div class="mt-1">
          <span v-if="factor.impact > 0" class="i-lucide-check-circle text-success"></span>
          <span v-else-if="factor.impact > -30" class="i-lucide-alert-triangle text-warning"></span>
          <span v-else class="i-lucide-x-circle text-error"></span>
        </div>
        <div>
          <div class="flex items-center gap-2">
            <span class="text-sm font-medium text-white">{{ factor.name }}</span>
            <span class="text-xs font-mono opacity-70" :class="factor.impact > 0 ? 'text-success' : 'text-error'">
              {{ factor.impact > 0 ? '+' : '' }}{{ factor.impact }}%
            </span>
          </div>
          <p class="text-xs text-white/60 mt-0.5">{{ factor.description }}</p>
        </div>
      </div>
    </div>

    <!-- Recommendation -->
    <div class="mt-6 pt-4 border-t border-white/10">
      <div class="flex items-start gap-3">
        <span class="i-lucide-info text-primary mt-0.5"></span>
        <p class="text-sm text-white/80 italic">"{{ recommendation }}"</p>
      </div>
    </div>

    <div v-if="loading" class="absolute inset-0 bg-surface/80 backdrop-blur-md flex items-center justify-center z-20">
      <div class="flex flex-col items-center gap-3">
        <div class="w-8 h-8 border-2 border-primary border-t-transparent rounded-full animate-spin"></div>
        <span class="text-sm text-white/60 animate-pulse">Analyzing telemetry...</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useDeviceStore } from '../../../stores/device';

const deviceStore = useDeviceStore();

interface RiskFactor {
  name: string;
  impact: number;
  description: string;
}

interface PredictionResult {
  score: number;
  risk_level: string;
  factors: RiskFactor[];
  recommendation: string;
}

const loading = ref(true);
const score = ref(0);
const riskLevel = ref('Unknown');
const factors = ref<RiskFactor[]>([]);
const recommendation = ref('');

const scoreColorClass = computed(() => {
  if (score.value >= 90) return 'text-success';
  if (score.value >= 70) return 'text-primary';
  if (score.value >= 40) return 'text-warning';
  return 'text-error';
});

const progressBarClass = computed(() => {
  if (score.value >= 90) return 'bg-success shadow-[0_0_15px_rgba(48,209,88,0.5)]';
  if (score.value >= 70) return 'bg-primary shadow-[0_0_15px_rgba(10,132,255,0.5)]';
  if (score.value >= 40) return 'bg-warning shadow-[0_0_15px_rgba(255,214,10,0.5)]';
  return 'bg-error shadow-[0_0_15px_rgba(255,69,58,0.5)]';
});

async function analyze() {
  loading.value = true;
  try {
    // In a real scenario, we'd get these from the device store or active state
    // For now, we simulate based on what we have
    const result = await invoke<PredictionResult>('calculate_success_score', {
      batteryLevel: deviceStore.batteryLevel || 0,
      isBootloaderUnlocked: true, // TODO: Get actual status
      isDebugEnabled: deviceStore.isConnected,
      firmwareMatched: true // TODO: Implement firmware check
    });

    score.value = result.score;
    riskLevel.value = result.risk_level;
    factors.value = result.factors;
    recommendation.value = result.recommendation;
  } catch (e) {
    console.error('Failed to calculate prediction:', e);
    score.value = 0;
    riskLevel.value = 'Error';
    recommendation.value = 'Could not analyze device state.';
  } finally {
    loading.value = false;
  }
}

onMounted(() => {
  analyze();
});

watch(() => deviceStore.batteryLevel, () => {
  analyze();
});
</script>

<style scoped>
.animate-pulse-slow {
  animation: pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 0.2; }
  50% { opacity: 0.5; }
}
</style>
