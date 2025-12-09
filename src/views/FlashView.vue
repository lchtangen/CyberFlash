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
    <div class="w-64 flex-shrink-0 bg-surface/30 border border-white/10 rounded-2xl backdrop-blur-xl flex flex-col overflow-hidden">
      <div class="p-4 border-b border-white/10">
        <h2 class="text-lg font-bold text-white tracking-tight">Operations</h2>
        <p class="text-xs text-text-secondary">Flash & Manage</p>
      </div>
      
      <div class="flex-1 overflow-y-auto p-2 space-y-1 custom-scrollbar">
        <SidebarItem 
          v-for="cat in categories" 
          :key="cat.id"
          :icon="cat.icon"
          :label="cat.label"
          :active="activeCategory === cat.id"
          @click="activeCategory = cat.id"
        />
      </div>

      <div class="p-4 border-t border-white/10">
        <button 
          @click="showBootloopDoctor = true"
          class="w-full py-2 px-3 bg-error/10 hover:bg-error/20 border border-error/20 rounded-lg text-xs font-bold text-error transition-colors flex items-center justify-center gap-2"
        >
          <span class="material-symbols-rounded text-sm">medical_services</span>
          Bootloop Doctor
        </button>
      </div>
    </div>

    <!-- Main Content Area -->
    <div class="flex-1 flex flex-col min-w-0 bg-surface/30 border border-white/10 rounded-2xl backdrop-blur-xl overflow-hidden">
      <!-- Header -->
      <div class="p-6 border-b border-white/10 flex justify-between items-center bg-white/5">
        <div>
          <h2 class="text-xl font-bold text-white tracking-tight">{{ activeCategoryLabel }}</h2>
          <p class="text-sm text-text-secondary">
            {{ activeCategory === 'wizard' ? 'Guided flashing process' : 'Advanced operations' }}
          </p>
        </div>
      </div>

      <!-- Scrollable Content -->
      <div class="flex-1 overflow-y-auto p-6 custom-scrollbar">
        
        <!-- Wizard Tab -->
        <div v-if="activeCategory === 'wizard'" class="space-y-6">
          <FlashWizard />
        </div>

        <!-- ROMs Tab -->
        <div v-if="activeCategory === 'roms'" class="space-y-6">
          <SmartRomRecommender />
          <RomSelector @romSelected="handleRomSelection" />
        </div>

        <!-- CLI Tab -->
        <div v-if="activeCategory === 'cli'" class="space-y-6">
          <NaturalLanguageCLI />
        </div>

        <!-- FaC Tab -->
        <div v-if="activeCategory === 'fac'" class="space-y-6">
          <FlashAsCode />
        </div>

        <!-- Sideload Tab -->
        <div v-if="activeCategory === 'sideload'" class="space-y-6">
          <SideloadAssistant />
        </div>

        <!-- Backup Tab -->
        <div v-if="activeCategory === 'backup'" class="space-y-6">
          <BackupManager />
        </div>

        <!-- Logs Tab -->
        <div v-if="activeCategory === 'logs'" class="space-y-6">
          <LogSentinel />
          <LogAnalyst />
        </div>

        <!-- Downloads Tab -->
        <div v-if="activeCategory === 'downloads'" class="space-y-6">
          <DownloadCenter />
        </div>

      </div>
    </div>
  </div>
</template>
