<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';
import GlassInput from '../../ui/GlassInput.vue';

interface SentimentReport {
  score: number;
  verdict: string;
  positive_count: number;
  negative_count: number;
  key_issues: string[];
}

const url = ref('');
const report = ref<SentimentReport | null>(null);
const isLoading = ref(false);

async function analyze() {
  if (!url.value) return;
  isLoading.value = true;
  report.value = null;
  
  try {
    report.value = await invoke<SentimentReport>('analyze_sentiment', { url: url.value });
  } catch (e) {
    console.error(e);
  } finally {
    isLoading.value = false;
  }
}

function useMock(type: 'stable' | 'buggy') {
  url.value = `https://forum.xda-developers.com/thread-${type}`;
}
</script>

<template>
  <GlassCard>
    <div class="flex items-center gap-3 mb-6">
      <div class="p-3 rounded-xl bg-primary/20 text-primary">
        <span class="material-symbols-rounded text-2xl">psychology</span>
      </div>
      <div>
        <h2 class="text-xl font-bold text-white">Community Sentiment AI</h2>
        <p class="text-sm text-white/60">Analyze XDA threads for stability reports.</p>
      </div>
    </div>

    <div class="space-y-4">
      <div class="flex gap-2">
        <GlassInput v-model="url" placeholder="Paste XDA Thread URL..." class="flex-1" />
        <VisionButton @click="analyze" :disabled="isLoading || !url">
          <span v-if="isLoading" class="animate-spin material-symbols-rounded">sync</span>
          <span v-else>Analyze</span>
        </VisionButton>
      </div>
      
      <div class="flex gap-4 text-xs text-white/40">
        <span class="cursor-pointer hover:text-primary" @click="useMock('stable')">Try Stable URL</span>
        <span class="cursor-pointer hover:text-primary" @click="useMock('buggy')">Try Buggy URL</span>
      </div>

      <div v-if="report" class="mt-6 p-4 rounded-xl bg-black/20 border border-white/5">
        <div class="flex items-center justify-between mb-4">
          <div>
            <div class="text-sm text-white/60">Stability Score</div>
            <div class="text-3xl font-bold" :class="{
              'text-success': report.score >= 80,
              'text-warning': report.score >= 50 && report.score < 80,
              'text-error': report.score < 50
            }">{{ report.score }}%</div>
          </div>
          <div class="text-right">
            <div class="text-sm text-white/60">Verdict</div>
            <div class="text-xl font-medium text-white">{{ report.verdict }}</div>
          </div>
        </div>

        <div class="space-y-2">
          <div class="flex justify-between text-sm">
            <span class="text-success">Positive Mentions</span>
            <span>{{ report.positive_count }}</span>
          </div>
          <div class="flex justify-between text-sm">
            <span class="text-error">Negative Mentions</span>
            <span>{{ report.negative_count }}</span>
          </div>
        </div>

        <div v-if="report.key_issues.length > 0" class="mt-4 pt-4 border-t border-white/10">
          <div class="text-sm text-white/60 mb-2">Reported Issues</div>
          <div class="flex flex-wrap gap-2">
            <span v-for="issue in report.key_issues" :key="issue" class="px-2 py-1 rounded-md bg-error/20 text-error text-xs capitalize">
              {{ issue }}
            </span>
          </div>
        </div>
      </div>
    </div>
  </GlassCard>
</template>
