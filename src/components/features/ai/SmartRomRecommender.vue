<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useNotificationStore } from '../../../stores/notifications';

interface RomRecommendation {
  name: string;
  description: string;
  tags: string[];
  official_url: string;
  match_score: number;
}

const notificationStore = useNotificationStore();
const selectedPersona = ref<string | null>(null);
const recommendations = ref<RomRecommendation[]>([]);
const loading = ref(false);

const personas = [
  { id: 'gaming', name: 'Gamer', icon: 'i-lucide-gamepad-2', desc: 'Max performance & customization' },
  { id: 'battery', name: 'Minimalist', icon: 'i-lucide-battery-charging', desc: 'Stability & battery life' },
  { id: 'privacy', name: 'Privacy Advocate', icon: 'i-lucide-shield-check', desc: 'De-googled & secure' },
  { id: 'balanced', name: 'Daily Driver', icon: 'i-lucide-smartphone', desc: 'Best of both worlds' },
];

const getRecommendations = async (personaId: string) => {
  selectedPersona.value = personaId;
  loading.value = true;
  recommendations.value = [];

  try {
    recommendations.value = await invoke<RomRecommendation[]>('recommend_roms', { usagePattern: personaId });
  } catch (e) {
    notificationStore.addNotification({
      title: 'Recommendation Failed',
      message: String(e),
      type: 'error'
    });
  } finally {
    loading.value = false;
  }
};

const openUrl = (url: string) => {
  window.open(url, '_blank');
};
</script>

<template>
  <div class="bg-surface/30 border border-white/10 rounded-2xl p-6 backdrop-blur-xl">
    <div class="flex items-center gap-3 mb-6">
      <div class="p-2 bg-primary/20 rounded-lg">
        <span class="i-lucide-lightbulb text-primary text-xl"></span>
      </div>
      <div>
        <h3 class="text-lg font-bold text-white">Smart ROM Recommender</h3>
        <p class="text-xs text-white/60">Select your usage style to find the perfect OS</p>
      </div>
    </div>

    <!-- Persona Selection -->
    <div class="grid grid-cols-2 md:grid-cols-4 gap-3 mb-6">
      <button 
        v-for="persona in personas" 
        :key="persona.id"
        @click="getRecommendations(persona.id)"
        class="relative p-4 rounded-xl border transition-all duration-300 text-left group overflow-hidden"
        :class="selectedPersona === persona.id ? 'bg-primary/20 border-primary/50' : 'bg-white/5 border-white/5 hover:bg-white/10'"
      >
        <div class="absolute inset-0 bg-gradient-to-br from-white/5 to-transparent opacity-0 group-hover:opacity-100 transition-opacity"></div>
        <span :class="[persona.icon, selectedPersona === persona.id ? 'text-primary' : 'text-white/60']" class="text-2xl mb-2 block transition-colors"></span>
        <div class="font-bold text-sm text-white mb-1">{{ persona.name }}</div>
        <div class="text-[10px] text-white/50 leading-tight">{{ persona.desc }}</div>
      </button>
    </div>

    <!-- Results -->
    <div v-if="loading" class="py-8 text-center">
      <div class="w-8 h-8 border-2 border-primary border-t-transparent rounded-full animate-spin mx-auto mb-2"></div>
      <span class="text-xs text-white/40">Analyzing compatibility...</span>
    </div>

    <div v-else-if="recommendations.length > 0" class="space-y-3 animate-in fade-in slide-in-from-bottom-4 duration-500">
      <div v-for="(rom, index) in recommendations" :key="index" class="bg-black/20 border border-white/10 rounded-xl p-4 flex items-center justify-between group hover:border-primary/30 transition-colors">
        <div class="flex-1">
          <div class="flex items-center gap-2 mb-1">
            <h4 class="font-bold text-white">{{ rom.name }}</h4>
            <span class="text-[10px] font-mono px-1.5 py-0.5 rounded bg-success/20 text-success border border-success/20">
              {{ rom.match_score }}% Match
            </span>
          </div>
          <p class="text-xs text-white/60 mb-2">{{ rom.description }}</p>
          <div class="flex gap-2">
            <span v-for="tag in rom.tags" :key="tag" class="text-[10px] px-2 py-0.5 rounded-full bg-white/5 text-white/40 border border-white/5">
              {{ tag }}
            </span>
          </div>
        </div>
        
        <button 
          @click="openUrl(rom.official_url)"
          class="p-2 rounded-lg bg-white/5 hover:bg-primary hover:text-white text-white/40 transition-all ml-4"
          title="Visit Website"
        >
          <span class="i-lucide-external-link"></span>
        </button>
      </div>
    </div>
  </div>
</template>
