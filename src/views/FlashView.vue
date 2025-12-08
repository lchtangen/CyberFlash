<script setup lang="ts">
import DevicePanel from '../components/features/DevicePanel.vue';
import FlashWizard from '../components/features/FlashWizard.vue';
import RomSelector from '../components/features/RomSelector.vue';
import SlotManager from '../components/features/SlotManager.vue';
import PartitionVisualizer from '../components/features/PartitionVisualizer.vue';
import SideloadAssistant from '../components/features/SideloadAssistant.vue';
import DeviceFingerprinter from '../components/features/DeviceFingerprinter.vue';
import FactoryResetSafe from '../components/features/FactoryResetSafe.vue';
import BackupManager from '../components/features/BackupManager.vue';
import LogAnalyst from '../components/features/LogAnalyst.vue';
import BootloaderUnlockFlow from '../components/features/BootloaderUnlockFlow.vue';
import RecoveryInstaller from '../components/features/RecoveryInstaller.vue';
import MagiskInjector from '../components/features/MagiskInjector.vue';
import DownloadCenter from '../components/features/DownloadCenter.vue';
import ThemeEngine from '../components/features/ThemeEngine.vue';
import DriverHealthCheck from '../components/features/DriverHealthCheck.vue';
import PermissionGate from '../components/features/PermissionGate.vue';
import AppUpdater from '../components/features/AppUpdater.vue';
import LiveTerminal from '../components/features/LiveTerminal.vue';
import AIGuardian from '../components/features/AIGuardian.vue';
</script>

<template>
  <div class="p-6 h-full flex flex-col overflow-y-auto custom-scrollbar">
    <DriverHealthCheck />
    <PermissionGate />
    <AppUpdater />
    <AIGuardian />
    <header class="mb-8">
      <h2 class="text-2xl font-bold text-white tracking-tight">Flash Firmware</h2>
      <p class="text-text-secondary text-sm">Install new ROMs safely</p>
    </header>

    <div class="flex-1 flex gap-6">
      <div class="flex-1 flex flex-col gap-6">
        <!-- Wizard -->
        <FlashWizard />
        
        <!-- ROM Selection -->
        <RomSelector />

        <!-- Download Center -->
        <DownloadCenter />

        <!-- Backup Manager -->
        <BackupManager />

        <!-- Sideload Assistant -->
        <SideloadAssistant />

        <!-- Log Analyst -->
        <LogAnalyst />
        
        <!-- Live Terminal -->
        <LiveTerminal />
      </div>

      <!-- Sidebar -->
      <div class="w-80 space-y-6">
        <DevicePanel />

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
