<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';
import { useNotificationStore } from '../../../stores/notifications';

interface TuningRecommendation {
  category: string;
  setting: string;
  current_value: string;
  suggested_value: string;
  reason: string;
  impact: string;
}

const notificationStore = useNotificationStore();
const recommendations = ref<TuningRecommendation[]>([]);
const isLoading = ref(false);

async function analyze() {
  isLoading.value = true;
  try {
    recommendations.value = await invoke<TuningRecommendation[]>('analyze_performance');
  } catch (e) {
    console.error(e);
  } finally {
    isLoading.value = false;
  }
}

async function apply(rec: TuningRecommendation) {
  try {
    await invoke('apply_tuning', { setting: rec.setting, value: rec.suggested_value });
    notificationStore.addNotification({
      title: 'Tuning Applied',
      message: `Changed ${rec.setting} to ${rec.suggested_value}`,
      type: 'success'
    });
    // Remove from list or mark as done
    recommendations.value = recommendations.value.filter(r => r !== rec);
  } catch (e) {
    notificationStore.addNotification({
      title: 'Failed',
      message: String(e),
      type: 'error'
    });
  }
}

onMounted(analyze);
</script>

<template>
  <GlassCard>
    <div class="flex items-center gap-3 mb-6">
      <div class="p-3 rounded-xl bg-primary/20 text-primary">
        <span class="material-symbols-rounded text-2xl">speed</span>
      </div>
      <div>
        <h2 class="text-xl font-bold text-white">AI Performance Tuner</h2>
        <p class="text-sm text-white/60">Optimize kernel and system settings.</p>
      </div>
    </div>

    <div v-if="isLoading" class="flex justify-center py-8">
      <div class="w-8 h-8 border-2 border-primary border-t-transparent rounded-full animate-spin"></div>
    </div>

    <div v-else class="space-y-4">
      <div v-for="(rec, i) in recommendations" :key="i" class="p-4 rounded-xl bg-black/20 border border-white/5 hover:bg-white/5 transition-colors">
        <div class="flex justify-between items-start mb-2">
          <div>
            <div class="flex items-center gap-2">
              <span class="text-xs font-bold uppercase tracking-wider text-primary">{{ rec.category }}</span>
              <span class="text-xs px-1.5 py-0.5 rounded bg-white/10 text-white/60">{{ rec.impact }} Impact</span>
            </div>
            <h3 class="font-medium text-white mt-1">{{ rec.setting }}</h3>
          </div>
          <VisionButton size="sm" @click="apply(rec)">Apply</VisionButton>
        </div>
        
        <p class="text-sm text-white/60 mb-3">{{ rec.reason }}</p>
        
        <div class="flex items-center gap-4 text-xs font-mono">
          <div class="text-white/40">Current: <span class="text-white">{{ rec.current_value }}</span></div>
          <div class="text-white/40">→</div>
          <div class="text-success">New: {{ rec.suggested_value }}</div>
        </div>
      </div>

      <div v-if="recommendations.length === 0" class="text-center py-8 text-white/40">
        <span class="material-symbols-rounded text-4xl mb-2">check_circle</span>
        <p>System is fully optimized.</p>
      </div>
    </div>
  </GlassCard>
</template>
