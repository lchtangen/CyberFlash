<script setup lang="ts">
import { useFlashStore } from '../../stores/flash';
import { computed, watch, ref, nextTick } from 'vue';
import { useNotificationStore } from '../../stores/notifications';
import GlassCard from '../ui/GlassCard.vue';
import VisionButton from '../ui/VisionButton.vue';

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
  <GlassCard class="flex flex-col h-full">
    <div class="flex items-center justify-between mb-6">
      <div>
        <h2 class="text-2xl font-bold text-white tracking-tight">Flash Wizard</h2>
        <p class="text-text-secondary text-sm">Phase {{ flashStore.currentPhase + 1 }}: {{ currentPhaseInfo.title }}</p>
      </div>
      <div class="text-right">
        <span class="text-3xl font-mono font-bold text-primary">{{ Math.round(globalProgress) }}%</span>
      </div>
    </div>

    <!-- Global Progress Bar -->
    <div class="h-1.5 bg-white/10 rounded-full overflow-hidden mb-8">
      <div 
        class="h-full bg-gradient-to-r from-primary to-secondary transition-all duration-500 ease-out"
        :style="{ width: `${globalProgress}%` }"
      ></div>
    </div>

    <!-- Content Area -->
    <div class="flex-1 bg-black/20 rounded-xl p-6 mb-6 border border-white/5 flex flex-col relative overflow-hidden">
      <!-- Background Mesh for active state -->
      <div v-if="flashStore.isFlashing" class="absolute inset-0 bg-primary/5 animate-pulse pointer-events-none"></div>

      <h3 class="text-xl font-semibold text-white mb-2 relative z-10">{{ currentPhaseInfo.title }}</h3>
      <p class="text-text-secondary mb-6 relative z-10">{{ currentPhaseInfo.description }}</p>
      
      <!-- Active Phase Progress -->
      <div v-if="flashStore.isFlashing" class="mb-6 space-y-2 relative z-10">
        <div class="flex justify-between text-xs text-white font-medium">
          <span class="flex items-center gap-2">
            <span class="w-2 h-2 rounded-full bg-primary animate-ping"></span>
            {{ flashStore.currentStep || 'Processing...' }}
          </span>
          <span class="font-mono">{{ Math.round(flashStore.progress) }}%</span>
        </div>
        <div class="h-2 bg-white/10 rounded-full overflow-hidden">
          <div 
            class="h-full bg-success transition-all duration-300 ease-out shadow-[0_0_10px_rgba(48,209,88,0.5)]"
            :style="{ width: `${flashStore.progress}%` }"
          ></div>
        </div>
      </div>

      <!-- Logs -->
      <div 
        ref="logContainer"
        class="flex-1 bg-black/40 rounded-lg p-4 font-mono text-xs text-gray-300 overflow-y-auto custom-scrollbar border border-white/5 shadow-inner"
      >
        <div v-for="(log, i) in flashStore.logs" :key="i" class="mb-1.5 border-l-2 border-white/10 pl-2 hover:border-primary/50 transition-colors">
          <span class="text-white/30 mr-2">[{{ new Date().toLocaleTimeString() }}]</span>
          {{ log }}
        </div>
        <div v-if="flashStore.logs.length === 0" class="h-full flex items-center justify-center text-text-muted italic opacity-50">
          Ready to start Phase {{ flashStore.currentPhase + 1 }}...
        </div>
      </div>
      
      <!-- Phase Indicators -->
      <div class="flex gap-2 mt-6 justify-center relative z-10">
        <div v-for="phase in phases" :key="phase.id" 
             class="w-2 h-2 rounded-full transition-all duration-300"
             :class="[
               phase.id - 1 === flashStore.currentPhase ? 'bg-primary scale-150 shadow-[0_0_8px_rgba(10,132,255,0.8)]' : '',
               phase.id - 1 < flashStore.currentPhase ? 'bg-success' : 'bg-white/20'
             ]"
        ></div>
      </div>
    </div>

    <!-- Actions -->
    <div class="flex justify-end gap-4">
      <VisionButton 
        v-if="!flashStore.isFlashing && flashStore.progress !== 100"
        @click="startPhase"
        variant="primary"
        icon="play_arrow"
        size="lg"
      >
        Start Phase {{ flashStore.currentPhase + 1 }}
      </VisionButton>
      
      <VisionButton 
        v-if="flashStore.progress === 100"
        @click="next"
        variant="success"
        icon="arrow_forward"
        size="lg"
      >
        Next Phase
      </VisionButton>
    </div>
  </GlassCard>
</template>
