<script setup lang="ts">
import { useFlashStore } from '../../../stores/flash';
import { computed, watch, ref, nextTick } from 'vue';
import { useNotificationStore } from '../../../stores/notifications';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';

const flashStore = useFlashStore();
const notificationStore = useNotificationStore();
const logContainer = ref<HTMLElement | null>(null);

const phases = [
  { id: 1, title: 'Downloads', description: 'Download required files' },
  { id: 2, title: 'Device Prep', description: 'Enable Dev Options & Debugging' },
  { id: 3, title: 'Unlock Bootloader', description: 'Unlock device bootloader' },
  { id: 4, title: 'Install TWRP', description: 'Flash Recovery' },
  { id: 5, title: 'Flash Firmware', description: 'Update Firmware H.41' },
  { id: 6, title: 'Wipe Device', description: 'Factory Reset & Format Data' },
  { id: 7, title: 'Install ROM', description: 'Flash crDroid 12' },
  { id: 8, title: 'Post-Install', description: 'Final Setup & Root' },
];

const currentPhaseInfo = computed(() => phases[flashStore.currentPhase] || phases[0]);
const globalProgress = computed(() => ((flashStore.currentPhase) / phases.length) * 100);

const startPhase = () => {
  flashStore.startFlash();
  notificationStore.addNotification({
    title: `Phase ${flashStore.currentPhase + 1} Started`,
    message: currentPhaseInfo.value.title,
    type: 'info'
  });
};

const next = () => {
  if (flashStore.currentPhase < phases.length - 1) {
    flashStore.nextPhase();
  }
};

// Auto-scroll logs
watch(() => flashStore.logs.length, async () => {
  await nextTick();
  if (logContainer.value) {
    logContainer.value.scrollTop = logContainer.value.scrollHeight;
  }
});

// Watch for phase completion
watch(() => flashStore.progress, (newVal) => {
  if (newVal === 100) {
    notificationStore.addNotification({
      title: 'Phase Complete',
      message: `${currentPhaseInfo.value.title} finished successfully.`,
      type: 'success'
    });
  }
});
</script>

<template>
  <GlassCard class="flex flex-col h-full relative overflow-hidden">
    <!-- Background Ambient Glow -->
    <div class="absolute top-0 right-0 w-64 h-64 bg-primary/20 blur-[100px] rounded-full pointer-events-none"></div>

    <div class="relative z-10 flex flex-col h-full">
      <!-- Header -->
      <div class="flex items-center justify-between mb-8">
        <div class="flex items-center gap-4">
          <div class="w-12 h-12 rounded-2xl bg-gradient-to-br from-primary to-secondary flex items-center justify-center shadow-lg shadow-primary/20">
            <span class="material-symbols-rounded text-white text-2xl">auto_fix_high</span>
          </div>
          <div>
            <h2 class="text-2xl font-bold text-white tracking-tight">Flash Wizard</h2>
            <div class="flex items-center gap-2 mt-1">
              <span class="px-2 py-0.5 rounded text-[10px] font-bold uppercase tracking-wider bg-white/10 text-white/70 border border-white/5">
                Phase {{ flashStore.currentPhase + 1 }}/{{ phases.length }}
              </span>
              <span class="text-sm text-text-secondary">{{ currentPhaseInfo.title }}</span>
            </div>
          </div>
        </div>
        <div class="text-right">
          <div class="text-3xl font-mono font-bold text-white tracking-tighter">{{ Math.round(globalProgress) }}<span class="text-lg text-white/50">%</span></div>
          <p class="text-[10px] text-text-secondary uppercase tracking-widest">Total Progress</p>
        </div>
      </div>

      <!-- Global Progress Bar -->
      <div class="h-1.5 bg-surface/50 rounded-full overflow-hidden mb-8 border border-white/5">
        <div 
          class="h-full bg-gradient-to-r from-primary via-secondary to-primary bg-[length:200%_100%] animate-[shimmer_2s_linear_infinite]"
          :style="{ width: `${globalProgress}%` }"
        ></div>
      </div>

      <div class="flex gap-6 flex-1 min-h-0">
        <!-- Phase List (Timeline) -->
        <div class="w-64 flex-shrink-0 overflow-y-auto custom-scrollbar pr-2 space-y-2">
          <div 
            v-for="(phase, index) in phases" 
            :key="phase.id"
            class="p-3 rounded-xl border transition-all duration-300 relative overflow-hidden group"
            :class="[
              index === flashStore.currentPhase 
                ? 'bg-white/10 border-primary/50 shadow-lg shadow-primary/10' 
                : (index < flashStore.currentPhase ? 'bg-success/5 border-success/20 opacity-70' : 'bg-surface/30 border-white/5 opacity-50')
            ]"
          >
            <div class="flex items-center gap-3 relative z-10">
              <div 
                class="w-6 h-6 rounded-full flex items-center justify-center text-xs font-bold border transition-colors"
                :class="[
                  index === flashStore.currentPhase ? 'bg-primary text-white border-primary' :
                  (index < flashStore.currentPhase ? 'bg-success text-black border-success' : 'bg-white/5 text-white/30 border-white/10')
                ]"
              >
                <span v-if="index < flashStore.currentPhase" class="material-symbols-rounded text-sm">check</span>
                <span v-else>{{ index + 1 }}</span>
              </div>
              <div class="flex-1 min-w-0">
                <p class="text-sm font-medium truncate" :class="index === flashStore.currentPhase ? 'text-white' : 'text-white/70'">{{ phase.title }}</p>
              </div>
            </div>
            <!-- Active Indicator -->
            <div v-if="index === flashStore.currentPhase" class="absolute left-0 top-0 bottom-0 w-1 bg-primary"></div>
          </div>
        </div>

        <!-- Active Phase Content -->
        <div class="flex-1 flex flex-col min-w-0 bg-black/20 rounded-2xl border border-white/10 overflow-hidden backdrop-blur-sm">
          <!-- Phase Header -->
          <div class="p-6 border-b border-white/5 bg-white/5">
            <h3 class="text-xl font-bold text-white mb-2">{{ currentPhaseInfo.title }}</h3>
            <p class="text-text-secondary text-sm leading-relaxed">{{ currentPhaseInfo.description }}</p>
          </div>

          <!-- Action Area -->
          <div class="p-6 flex-1 flex flex-col justify-center items-center text-center relative">
             <!-- Background Animation for Active State -->
             <div v-if="flashStore.isFlashing" class="absolute inset-0 flex items-center justify-center opacity-10 pointer-events-none">
                <div class="w-64 h-64 rounded-full border-4 border-primary border-t-transparent animate-spin"></div>
             </div>

             <div v-if="!flashStore.isFlashing && flashStore.progress !== 100" class="space-y-6 max-w-md mx-auto relative z-10">
                <div class="w-20 h-20 mx-auto rounded-full bg-white/5 flex items-center justify-center border border-white/10">
                   <span class="material-symbols-rounded text-4xl text-white/50">play_arrow</span>
                </div>
                <div>
                  <h4 class="text-lg font-bold text-white">Ready to Start</h4>
                  <p class="text-text-secondary text-sm mt-2">Ensure your device is connected and has at least 50% battery before proceeding.</p>
                </div>
                <VisionButton size="lg" class="w-full justify-center shadow-xl shadow-primary/20" @click="startPhase">
                  Start Phase {{ flashStore.currentPhase + 1 }}
                </VisionButton>
             </div>

             <div v-else-if="flashStore.progress === 100" class="space-y-6 max-w-md mx-auto relative z-10">
                <div class="w-20 h-20 mx-auto rounded-full bg-success/10 flex items-center justify-center border border-success/20">
                   <span class="material-symbols-rounded text-4xl text-success">check_circle</span>
                </div>
                <div>
                  <h4 class="text-lg font-bold text-white">Phase Complete</h4>
                  <p class="text-text-secondary text-sm mt-2">Successfully completed {{ currentPhaseInfo.title }}.</p>
                </div>
                <VisionButton size="lg" class="w-full justify-center shadow-xl shadow-success/20" variant="primary" @click="next">
                  Next Phase
                </VisionButton>
             </div>

             <div v-else class="w-full max-w-lg space-y-6 relative z-10">
                <div class="flex justify-between text-sm font-medium text-white">
                  <span class="flex items-center gap-2">
                    <span class="w-2 h-2 rounded-full bg-primary animate-ping"></span>
                    {{ flashStore.currentStep || 'Processing...' }}
                  </span>
                  <span class="font-mono">{{ Math.round(flashStore.progress) }}%</span>
                </div>
                <div class="h-3 bg-surface/50 rounded-full overflow-hidden border border-white/10">
                  <div 
                    class="h-full bg-success transition-all duration-300 ease-out shadow-[0_0_15px_rgba(48,209,88,0.6)] relative overflow-hidden"
                    :style="{ width: `${flashStore.progress}%` }"
                  >
                    <div class="absolute inset-0 bg-white/20 w-full h-full animate-[shimmer_1s_infinite]"></div>
                  </div>
                </div>
                <p class="text-xs text-text-muted animate-pulse">Do not disconnect your device...</p>
             </div>
          </div>

          <!-- Terminal Output (Collapsible or Fixed at bottom) -->
          <div class="h-48 bg-surface/50 backdrop-blur-md border-t border-white/10 flex flex-col">
            <div class="px-4 py-2 bg-white/5 border-b border-white/5 flex justify-between items-center">
              <span class="text-[10px] font-mono uppercase tracking-wider text-text-secondary">Process Log</span>
              <div class="flex gap-1.5">
                <div class="w-2 h-2 rounded-full bg-error/50"></div>
                <div class="w-2 h-2 rounded-full bg-warning/50"></div>
                <div class="w-2 h-2 rounded-full bg-success/50"></div>
              </div>
            </div>
            <div ref="logContainer" class="flex-1 overflow-y-auto p-4 font-mono text-xs space-y-1 custom-scrollbar">
              <div v-if="flashStore.logs.length === 0" class="text-white/20 italic">Waiting for process to start...</div>
              <div v-for="(log, i) in flashStore.logs" :key="i" class="break-all">
                <span class="text-white/30 mr-2">[{{ new Date().toLocaleTimeString() }}]</span>
                <span :class="log.includes('Error') ? 'text-error' : (log.includes('Success') ? 'text-success' : 'text-white/80')">
                  {{ log }}
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </GlassCard>
</template>
