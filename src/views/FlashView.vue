<script setup lang="ts">
import { ref, computed } from 'vue';
import SidebarItem from '../components/ui/SidebarItem.vue';

// Components
import FlashWizard from '../components/features/core/FlashWizard.vue';
import RomSelector from '../components/features/core/RomSelector.vue';
import SideloadAssistant from '../components/features/system/SideloadAssistant.vue';
import BackupManager from '../components/features/system/BackupManager.vue';
import LogAnalyst from '../components/features/system/LogAnalyst.vue';
import DownloadCenter from '../components/features/cloud/DownloadCenter.vue';
import DriverHealthCheck from '../components/features/system/DriverHealthCheck.vue';
import PermissionGate from '../components/features/security/PermissionGate.vue';
import AppUpdater from '../components/features/cloud/AppUpdater.vue';
import AIGuardian from '../components/features/ai/AIGuardian.vue';
import BrickGuardian from '../components/features/security/BrickGuardian.vue';
import VisualBootloopDoctor from '../components/features/system/VisualBootloopDoctor.vue';
import NaturalLanguageCLI from '../components/features/ai/NaturalLanguageCLI.vue';
import SmartRomRecommender from '../components/features/ai/SmartRomRecommender.vue';
import LogSentinel from '../components/features/system/LogSentinel.vue';
import FlashAsCode from '../components/features/automation/FlashAsCode.vue';
import { useDeviceStore } from '../stores/device';

const showBootloopDoctor = ref(false);
const deviceStore = useDeviceStore();
const targetRom = ref('');
const targetAndroidVersion = ref('');

const handleRomSelection = (metadata: any) => {
  targetRom.value = metadata.name;
  if (metadata.name.includes('14') || metadata.name.includes('21')) targetAndroidVersion.value = '14';
  else if (metadata.name.includes('13') || metadata.name.includes('20')) targetAndroidVersion.value = '13';
  else targetAndroidVersion.value = 'unknown';
};

const activeCategory = ref('wizard');

const categories = [
  { id: 'wizard', label: 'Flash Wizard', icon: 'auto_fix_high' },
  { id: 'roms', label: 'ROM Selector', icon: 'install_mobile' },
  { id: 'cli', label: 'Smart CLI', icon: 'terminal' },
  { id: 'fac', label: 'Flash-as-Code', icon: 'code' },
  { id: 'sideload', label: 'Sideload', icon: 'system_update' },
  { id: 'backup', label: 'Backup & Restore', icon: 'backup' },
  { id: 'logs', label: 'Log Analysis', icon: 'bug_report' },
  { id: 'downloads', label: 'Downloads', icon: 'download' },
];

const activeCategoryLabel = computed(() => categories.find(c => c.id === activeCategory.value)?.label);
</script>

<template>
  <div class="flex h-full gap-6 overflow-hidden p-6">
    <!-- Global Guards (Hidden) -->
    <DriverHealthCheck />
    <PermissionGate />
    <AppUpdater />
    <AIGuardian />
    <BrickGuardian 
      :deviceModel="deviceStore.deviceModel || 'OnePlus 7 Pro'" 
      :currentFirmware="'OOS 11.0.9.1'" 
      :targetRom="targetRom" 
      :targetAndroidVersion="targetAndroidVersion" 
    />
    <VisualBootloopDoctor :is-open="showBootloopDoctor" @close="showBootloopDoctor = false" />

    <!-- Sidebar Navigation -->
    <GlassCard noPadding class="w-64 flex-shrink-0 flex flex-col overflow-hidden ring-1 ring-white/5 shadow-2xl shadow-black/20">
      <div class="p-5 border-b border-white/10 bg-surface/30 backdrop-blur-xl">
        <h2 class="text-lg font-bold text-white tracking-tight flex items-center gap-2">
          <span class="material-symbols-rounded text-primary drop-shadow-[0_0_8px_rgba(0,240,255,0.5)]">bolt</span>
          Operations
        </h2>
        <p class="text-xs text-text-secondary mt-1 font-medium tracking-wide">Flash & Manage</p>
      </div>
      
      <div class="flex-1 overflow-y-auto p-3 space-y-1 custom-scrollbar">
        <SidebarItem 
          v-for="cat in categories" 
          :key="cat.id"
          :icon="cat.icon"
          :label="cat.label"
          :active="activeCategory === cat.id"
          @click="activeCategory = cat.id"
        />
      </div>

      <div class="p-4 border-t border-white/10 bg-surface/30 backdrop-blur-xl">
        <button 
          @click="showBootloopDoctor = true"
          class="w-full py-2.5 px-3 bg-error/10 hover:bg-error/20 border border-error/20 rounded-xl text-xs font-bold text-error transition-all duration-300 flex items-center justify-center gap-2 hover:scale-[1.02] active:scale-95 shadow-lg shadow-error/5 backdrop-blur-md"
        >
          <span class="material-symbols-rounded text-sm">medical_services</span>
          Bootloop Doctor
        </button>
      </div>
    </GlassCard>

    <!-- Main Content Area -->
    <GlassCard noPadding class="flex-1 flex flex-col min-w-0 overflow-hidden relative ring-1 ring-white/5 shadow-2xl shadow-black/20">
      <!-- Header -->
      <div class="p-6 border-b border-white/10 flex justify-between items-center bg-white/5 backdrop-blur-md z-10">
        <div>
          <h2 class="text-xl font-bold text-white tracking-tight flex items-center gap-2">
            {{ activeCategoryLabel }}
            <span v-if="activeCategory === 'wizard'" class="px-2 py-0.5 rounded-full bg-primary/20 text-primary text-[10px] font-bold border border-primary/20 uppercase tracking-wider">Guided</span>
          </h2>
          <p class="text-sm text-text-secondary mt-1">
            {{ activeCategory === 'wizard' ? 'Guided flashing process' : 'Advanced operations' }}
          </p>
        </div>
      </div>

      <!-- Scrollable Content -->
      <div class="flex-1 overflow-y-auto p-8 custom-scrollbar bg-gradient-to-b from-transparent to-black/20">
        
        <!-- Wizard Tab -->
        <div v-if="activeCategory === 'wizard'" class="space-y-8 animate-slide-up">
          <div class="hover-tilt-subtle">
            <FlashWizard />
          </div>
        </div>

        <!-- ROMs Tab -->
        <div v-if="activeCategory === 'roms'" class="space-y-8 animate-slide-up">
          <div class="hover-tilt-subtle">
            <SmartRomRecommender />
          </div>
          <div class="hover-tilt-subtle">
            <RomSelector @romSelected="handleRomSelection" />
          </div>
        </div>

        <!-- CLI Tab -->
        <div v-if="activeCategory === 'cli'" class="space-y-6 animate-slide-up">
          <NaturalLanguageCLI class="h-full shadow-2xl shadow-black/50 rounded-xl overflow-hidden border border-white/10" />
        </div>

        <!-- FaC Tab -->
        <div v-if="activeCategory === 'fac'" class="space-y-6 animate-slide-up">
          <FlashAsCode class="hover-tilt-subtle" />
        </div>

        <!-- Sideload Tab -->
        <div v-if="activeCategory === 'sideload'" class="space-y-6 animate-slide-up">
          <SideloadAssistant class="hover-tilt-subtle" />
        </div>

        <!-- Backup Tab -->
        <div v-if="activeCategory === 'backup'" class="space-y-6 animate-slide-up">
          <BackupManager class="hover-tilt-subtle" />
        </div>

        <!-- Logs Tab -->
        <div v-if="activeCategory === 'logs'" class="space-y-6 animate-slide-up">
          <LogSentinel class="hover-tilt-subtle" />
          <LogAnalyst class="hover-tilt-subtle" />
        </div>

        <!-- Downloads Tab -->
        <div v-if="activeCategory === 'downloads'" class="space-y-6 animate-slide-up">
          <DownloadCenter class="hover-tilt-subtle" />
        </div>

      </div>
    </GlassCard>
  </div>
</template>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 10px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.2);
}

@keyframes slide-up {
  from { opacity: 0; transform: translateY(20px); }
  to { opacity: 1; transform: translateY(0); }
}

.animate-slide-up {
  animation: slide-up 0.5s cubic-bezier(0.16, 1, 0.3, 1) forwards;
}

.hover-tilt-subtle {
  transition: transform 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275), box-shadow 0.4s ease;
}

.hover-tilt-subtle:hover {
  transform: translateY(-4px) scale(1.005);
  box-shadow: 0 20px 40px -10px rgba(0, 0, 0, 0.4);
  z-index: 10;
}
</style>
