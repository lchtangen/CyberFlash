<script setup lang="ts">
import DevicePanel from '../components/features/core/DevicePanel.vue';
import FlashWizard from '../components/features/core/FlashWizard.vue';
import RomSelector from '../components/features/core/RomSelector.vue';
import SlotManager from '../components/features/system/SlotManager.vue';
import PartitionVisualizer from '../components/features/system/PartitionVisualizer.vue';
import SideloadAssistant from '../components/features/system/SideloadAssistant.vue';
import DeviceFingerprinter from '../components/features/security/DeviceFingerprinter.vue';
import FactoryResetSafe from '../components/features/system/FactoryResetSafe.vue';
import BackupManager from '../components/features/system/BackupManager.vue';
import LogAnalyst from '../components/features/system/LogAnalyst.vue';
import BootloaderUnlockFlow from '../components/features/security/BootloaderUnlockFlow.vue';
import RecoveryInstaller from '../components/features/system/RecoveryInstaller.vue';
import MagiskInjector from '../components/features/automation/MagiskInjector.vue';
import DownloadCenter from '../components/features/cloud/DownloadCenter.vue';
import ThemeEngine from '../components/features/core/ThemeEngine.vue';
import DriverHealthCheck from '../components/features/system/DriverHealthCheck.vue';
import PermissionGate from '../components/features/security/PermissionGate.vue';
import AppUpdater from '../components/features/cloud/AppUpdater.vue';
import LiveTerminal from '../components/features/system/LiveTerminal.vue';
import AIGuardian from '../components/features/ai/AIGuardian.vue';
import PredictiveScoreCard from '../components/features/ai/PredictiveScoreCard.vue';
import LogSentinel from '../components/features/system/LogSentinel.vue';
import FlashAsCode from '../components/features/automation/FlashAsCode.vue';
import VisualBootloopDoctor from '../components/features/system/VisualBootloopDoctor.vue';
import NaturalLanguageCLI from '../components/features/ai/NaturalLanguageCLI.vue';
import SmartRomRecommender from '../components/features/ai/SmartRomRecommender.vue';
import BrickGuardian from '../components/features/security/BrickGuardian.vue';
import { useDeviceStore } from '../stores/device';
import { ref } from 'vue';

const showBootloopDoctor = ref(false);
const deviceStore = useDeviceStore();
const targetRom = ref('');
const targetAndroidVersion = ref('');

const handleRomSelection = (metadata: any) => {
  targetRom.value = metadata.name;
  // Simple heuristic for version from name
  if (metadata.name.includes('14') || metadata.name.includes('21')) targetAndroidVersion.value = '14';
  else if (metadata.name.includes('13') || metadata.name.includes('20')) targetAndroidVersion.value = '13';
  else targetAndroidVersion.value = 'unknown';
};
</script>

<template>
  <div class="p-6 h-full flex flex-col overflow-y-auto custom-scrollbar">
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
    
    <header class="mb-8 flex justify-between items-end">
      <div>
        <h2 class="text-2xl font-bold text-white tracking-tight">Flash Firmware</h2>
        <p class="text-text-secondary text-sm">Install new ROMs safely</p>
      </div>
      <button @click="showBootloopDoctor = true" class="px-4 py-2 bg-white/5 hover:bg-white/10 border border-white/10 rounded-lg text-sm font-medium text-white transition-colors flex items-center gap-2">
        <span class="i-lucide-eye"></span>
        Bootloop Doctor
      </button>
    </header>

    <div class="flex-1 flex gap-6">
      <div class="flex-1 flex flex-col gap-6">
        <!-- Natural Language CLI -->
        <NaturalLanguageCLI />

        <!-- Wizard -->
        <FlashWizard />

        <!-- Flash-as-Code -->
        <FlashAsCode />
        
        <!-- ROM Selection -->
        <RomSelector @romSelected="handleRomSelection" />

        <!-- Smart ROM Recommender -->
        <SmartRomRecommender />

        <!-- Download Center -->
        <DownloadCenter />

        <!-- Backup Manager -->
        <BackupManager />

        <!-- Sideload Assistant -->
        <SideloadAssistant />

        <!-- Log Analyst -->
        <LogSentinel />
        <LogAnalyst />
        
        <!-- Live Terminal -->
        <LiveTerminal />
      </div>

      <!-- Sidebar -->
      <div class="w-80 space-y-6">
        <DevicePanel />

        <PredictiveScoreCard />

        <DeviceFingerprinter />
        
        <BootloaderUnlockFlow />

        <RecoveryInstaller />

        <MagiskInjector />

        <SlotManager />

        <PartitionVisualizer />

        <FactoryResetSafe />

        <ThemeEngine />
      </div>
    </div>
  </div>
</template>
