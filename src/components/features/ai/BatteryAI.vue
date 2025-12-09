<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';

interface BatteryReport {
  cycle_count: number;
  health_percentage: number;
  predicted_lifespan_months: number;
  degradation_rate: number;
  recommendation: string;
}

const report = ref<BatteryReport | null>(null);
const isLoading = ref(false);

async function analyze() {
  isLoading.value = true;
  try {
    report.value = await invoke<BatteryReport>('analyze_battery');
  } catch (e) {
    console.error(e);
  } finally {
    isLoading.value = false;
  }
}

onMounted(analyze);
</script>

<template>
  <GlassCard>
    <div class="flex items-center gap-3 mb-6">
      <div class="p-3 rounded-xl bg-primary/20 text-primary">
        <span class="material-symbols-rounded text-2xl">battery_charging_full</span>
      </div>
      <div>
        <h2 class="text-xl font-bold text-white">Battery AI Predictor</h2>
        <p class="text-sm text-white/60">Health analysis & lifespan forecasting.</p>
      </div>
    </div>

    <div v-if="isLoading" class="flex justify-center py-8">
      <div class="w-8 h-8 border-2 border-primary border-t-transparent rounded-full animate-spin"></div>
    </div>

    <div v-else-if="report" class="space-y-6">
      <div class="grid grid-cols-2 gap-4">
        <div class="p-4 rounded-xl bg-black/20 border border-white/5 text-center">
          <div class="text-sm text-white/60 mb-1">Health</div>
          <div class="text-2xl font-bold" :class="{
            'text-success': report.health_percentage >= 85,
            'text-warning': report.health_percentage >= 70 && report.health_percentage < 85,
            'text-error': report.health_percentage < 70
          }">{{ report.health_percentage }}%</div>
        </div>
        <div class="p-4 rounded-xl bg-black/20 border border-white/5 text-center">
          <div class="text-sm text-white/60 mb-1">Cycles</div>
          <div class="text-2xl font-bold text-white">{{ report.cycle_count }}</div>
        </div>
      </div>

      <div class="p-4 rounded-xl bg-primary/10 border border-primary/20">
        <div class="flex items-center gap-2 mb-2">
          <span class="material-symbols-rounded text-primary">timelapse</span>
          <span class="font-medium text-white">Prediction</span>
        </div>
        <p class="text-sm text-white/80">
          Based on your usage, this battery will need replacement in approximately 
          <span class="font-bold text-white">{{ report.predicted_lifespan_months }} months</span>.
        </p>
      </div>

      <div class="text-sm text-white/60 italic">
        "{{ report.recommendation }}"
      </div>
    </div>
  </GlassCard>
</template>
