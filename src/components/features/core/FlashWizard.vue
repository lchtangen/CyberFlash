<script setup lang="ts">
import { useFlashStore } from '../../../stores/flash';
import { computed, watch, ref, nextTick } from 'vue';
import { useNotificationStore } from '../../../stores/notifications';
import { useDynamicIslandStore } from '../../../stores/dynamicIsland';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';
import HolographicTerminal from '../../ui/HolographicTerminal.vue';

const flashStore = useFlashStore();
const notificationStore = useNotificationStore();
const islandStore = useDynamicIslandStore();
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
  
  // Update Dynamic Island
  islandStore.setActivity({
    id: 'flashing',
    type: 'process',
    icon: 'auto_fix_high',
    title: `Phase ${flashStore.currentPhase + 1}: ${currentPhaseInfo.value.title}`,
    subtitle: 'Processing...',
    progress: 0,
    color: 'text-primary',
    bg: 'bg-primary/20',
    border: 'border-primary/30'
  });

  notificationStore.addNotification({
    title: `Phase ${flashStore.currentPhase + 1} Started`,
    message: currentPhaseInfo.value.title,
    type: 'info'
  });
};

const next = () => {
  if (flashStore.currentPhase < phases.length - 1) {
    flashStore.nextPhase();
    
    // Update Island for next phase
    islandStore.setActivity({
      id: 'flashing',
      type: 'process',
      icon: 'auto_fix_high',
      title: `Phase ${flashStore.currentPhase + 1}: ${currentPhaseInfo.value.title}`,
      subtitle: 'Processing...',
      progress: 0,
      color: 'text-primary',
      bg: 'bg-primary/20',
      border: 'border-primary/30'
    });
  } else {
    // Finished
    islandStore.clearActivity();
    islandStore.showNotification({
      id: 'flash-complete',
      type: 'success',
      title: 'Flash Complete',
      subtitle: 'All phases finished successfully.',
      icon: 'check_circle'
    });
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
  // Update Island Progress
  if (islandStore.activeActivity?.id === 'flashing') {
    islandStore.updateProgress(newVal);
  }

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
        <div class="w-72 flex-shrink-0 overflow-y-auto custom-scrollbar pr-2 space-y-3">
          <div 
            v-for="(phase, index) in phases" 
            :key="phase.id"
            class="p-4 rounded-xl border transition-all duration-300 relative overflow-hidden group"
            :class="[
              index === flashStore.currentPhase 
                ? 'bg-primary/10 border-primary/30 shadow-[0_0_15px_rgba(0,240,255,0.1)]' 
                : index < flashStore.currentPhase 
                  ? 'bg-success/5 border-success/20 opacity-60' 
                  : 'bg-surface/30 border-white/5 opacity-40'
            ]"
          >
            <!-- Active Indicator Line -->
            <div v-if="index === flashStore.currentPhase" class="absolute left-0 top-0 bottom-0 w-1 bg-primary shadow-[0_0_10px_rgba(0,240,255,0.5)]"></div>

            <div class="flex items-center justify-between mb-1">
              <h3 class="font-bold text-sm" :class="index === flashStore.currentPhase ? 'text-white' : 'text-white/70'">{{ phase.title }}</h3>
              <span v-if="index < flashStore.currentPhase" class="material-symbols-rounded text-success text-sm">check_circle</span>
              <span v-else-if="index === flashStore.currentPhase" class="w-2 h-2 rounded-full bg-primary animate-pulse shadow-[0_0_8px_rgba(0,240,255,0.8)]"></span>
            </div>
            <p class="text-xs text-text-secondary leading-relaxed">{{ phase.description }}</p>
          </div>
        </div>

        <!-- Main Action Area -->
        <div class="flex-1 flex flex-col gap-6">
          <!-- Current Step Detail -->
          <div class="bg-surface/30 border border-white/10 rounded-xl p-6 backdrop-blur-md relative overflow-hidden group">
             <div class="absolute top-0 right-0 w-32 h-32 bg-primary/10 blur-[60px] rounded-full pointer-events-none group-hover:bg-primary/20 transition-colors duration-500"></div>
             
             <h3 class="text-lg font-bold text-white mb-2 flex items-center gap-2">
               <span class="material-symbols-rounded text-primary">info</span>
               Current Operation
             </h3>
             <p class="text-sm text-text-secondary mb-6 leading-relaxed">
               {{ currentPhaseInfo.description }}. Ensure your device is connected and has sufficient battery (50%+). Do not disconnect the cable during this process.
             </p>

             <div class="flex gap-3">
               <VisionButton 
                 @click="startPhase" 
                 :disabled="flashStore.isFlashing" 
                 :loading="flashStore.isFlashing"
                 variant="primary"
                 size="lg"
                 class="flex-1 shadow-xl shadow-primary/10"
               >
                 {{ flashStore.isFlashing ? 'Processing...' : 'Start Phase' }}
               </VisionButton>
               
               <VisionButton 
                 @click="next" 
                 :disabled="!flashStore.isFlashing && flashStore.progress !== 100"
                 variant="secondary"
                 size="lg"
               >
                 Skip / Next
               </VisionButton>
             </div>
          </div>

          <!-- Terminal Output -->
          <HolographicTerminal 
            :logs="flashStore.logs" 
            title="FLASH_CONSOLE" 
            class="flex-1 min-h-0" 
          />
        </div>
      </div>
    </div>
  </GlassCard>
</template>
