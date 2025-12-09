<script setup lang="ts">
import { ref } from 'vue';
import GlassCard from '../../ui/GlassCard.vue';
import ToggleSwitch from '../../ui/ToggleSwitch.vue';

// Local state for UI demonstration (some might map to store later)
const autoAnalyze = ref(true);
const voiceFeedback = ref(false);
const aggressiveMode = ref(false);
const cloudSync = ref(true);
const privacyMode = ref(false);

</script>

<template>
  <GlassCard noPadding class="w-full h-full flex flex-col overflow-hidden border-white/10 shadow-2xl shadow-black/50">
    <!-- Header -->
    <div class="px-5 py-4 border-b border-white/5 flex items-center justify-between bg-white/5 backdrop-blur-md z-10">
      <div class="flex items-center gap-4">
        <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-purple-500/20 to-purple-500/5 flex items-center justify-center border border-white/10 shadow-inner">
          <span class="material-symbols-rounded text-purple-400 text-xl">settings_suggest</span>
        </div>
        <div>
          <h3 class="font-bold text-white text-base tracking-wide">AI Settings</h3>
          <p class="text-xs text-text-secondary font-medium">Configure Assistant</p>
        </div>
      </div>
      
      <button 
        @click="$emit('close')"
        class="p-2 rounded-lg hover:bg-white/10 text-text-secondary hover:text-white transition-all duration-200 group"
      >
        <span class="material-symbols-rounded text-lg group-hover:scale-110 transition-transform">close</span>
      </button>
    </div>

    <!-- Content -->
    <div class="flex-1 overflow-y-auto p-5 custom-scrollbar space-y-6">
      
      <!-- Section: Behavior -->
      <div class="space-y-3">
        <h4 class="text-xs font-bold text-text-secondary uppercase tracking-wider ml-1">Behavior</h4>
        
        <div class="bg-surface/30 border border-white/5 rounded-xl p-4 flex items-center justify-between">
          <div>
            <div class="text-sm font-medium text-white">Auto-Analyze Logs</div>
            <div class="text-xs text-text-secondary mt-0.5">Scan ADB logs for errors automatically</div>
          </div>
          <ToggleSwitch v-model="autoAnalyze" />
        </div>

        <div class="bg-surface/30 border border-white/5 rounded-xl p-4 flex items-center justify-between">
          <div>
            <div class="text-sm font-medium text-white">Voice Feedback</div>
            <div class="text-xs text-text-secondary mt-0.5">Speak critical alerts and confirmations</div>
          </div>
          <ToggleSwitch v-model="voiceFeedback" />
        </div>
      </div>

      <!-- Section: Performance -->
      <div class="space-y-3">
        <h4 class="text-xs font-bold text-text-secondary uppercase tracking-wider ml-1">Performance</h4>
        
        <div class="bg-surface/30 border border-white/5 rounded-xl p-4 flex items-center justify-between">
          <div>
            <div class="text-sm font-medium text-white">Aggressive Mode</div>
            <div class="text-xs text-text-secondary mt-0.5">Prioritize speed over safety checks (Risky)</div>
          </div>
          <ToggleSwitch v-model="aggressiveMode" color="error" />
        </div>
      </div>

      <!-- Section: Cloud & Privacy -->
      <div class="space-y-3">
        <h4 class="text-xs font-bold text-text-secondary uppercase tracking-wider ml-1">Cloud & Privacy</h4>
        
        <div class="bg-surface/30 border border-white/5 rounded-xl p-4 flex items-center justify-between">
          <div>
            <div class="text-sm font-medium text-white">Cloud Sync</div>
            <div class="text-xs text-text-secondary mt-0.5">Backup profiles to CyberFlash Cloud</div>
          </div>
          <ToggleSwitch v-model="cloudSync" />
        </div>

        <div class="bg-surface/30 border border-white/5 rounded-xl p-4 flex items-center justify-between">
          <div>
            <div class="text-sm font-medium text-white">Privacy Mode</div>
            <div class="text-xs text-text-secondary mt-0.5">Do not send telemetry or logs</div>
          </div>
          <ToggleSwitch v-model="privacyMode" />
        </div>
      </div>

      <!-- Model Info -->
      <div class="mt-6 p-4 rounded-xl bg-primary/10 border border-primary/20">
        <div class="flex items-center gap-2 mb-2">
          <span class="material-symbols-rounded text-primary text-sm">psychology</span>
          <span class="text-xs font-bold text-primary uppercase">Model Info</span>
        </div>
        <div class="text-xs text-white/80">
          Running <span class="font-mono text-primary">Gemini 3 Pro</span> (Preview)
        </div>
        <div class="text-[10px] text-white/50 mt-1">
          Latency: 45ms • Context: 128k
        </div>
      </div>

    </div>
  </GlassCard>
</template>
