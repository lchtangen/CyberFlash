<script setup lang="ts">
import { ref, computed } from 'vue';
import GlassInput from '../components/ui/GlassInput.vue';
import SidebarItem from '../components/ui/SidebarItem.vue';

// Automation
import ConfigGenerator from '../components/features/automation/ConfigGenerator.vue';
import ZeroTouchPanel from '../components/features/automation/ZeroTouchPanel.vue';
import CloudBuildSync from '../components/features/automation/CloudBuildSync.vue';
import MultiDeviceGrid from '../components/features/automation/MultiDeviceGrid.vue';
import RecoveryScriptGen from '../components/features/automation/RecoveryScriptGen.vue';
import ApkBatchInstaller from '../components/features/automation/ApkBatchInstaller.vue';
import Debloater from '../components/features/automation/Debloater.vue';
import ModuleManager from '../components/features/automation/ModuleManager.vue';
import PropEditor from '../components/features/automation/PropEditor.vue';
import GAppsIntegrator from '../components/features/automation/GAppsIntegrator.vue';
import FirmwareMatcher from '../components/features/automation/FirmwareMatcher.vue';

// Security
import HashVerifier from '../components/features/security/HashVerifier.vue';
import EFSGuard from '../components/features/security/EFSGuard.vue';
import PrivacyDashboard from '../components/features/security/PrivacyDashboard.vue';
import SafetyNetFixer from '../components/features/security/SafetyNetFixer.vue';
import BootloaderLocker from '../components/features/security/BootloaderLocker.vue';
import KillSwitch from '../components/features/security/KillSwitch.vue';

// System
import PartitionResizer from '../components/features/system/PartitionResizer.vue';
import KernelSUInstaller from '../components/features/system/KernelSUInstaller.vue';
import DualBootMgr from '../components/features/system/DualBootMgr.vue';
import PayloadStreamer from '../components/features/system/PayloadStreamer.vue';

// AI
import SentimentAnalyzer from '../components/features/ai/SentimentAnalyzer.vue';
import PerformanceTuner from '../components/features/ai/PerformanceTuner.vue';
import BatteryAI from '../components/features/ai/BatteryAI.vue';
import NetworkOptimizer from '../components/features/ai/NetworkOptimizer.vue';

// Cloud
import CommunityRepo from '../components/features/cloud/CommunityRepo.vue';
import ConfigSharer from '../components/features/cloud/ConfigSharer.vue';
import DevProfileSync from '../components/features/cloud/DevProfileSync.vue';

const activeCategory = ref('automation');
const searchQuery = ref('');

const categories = [
  { id: 'automation', label: 'Automation', icon: 'smart_toy' },
  { id: 'security', label: 'Security', icon: 'security' },
  { id: 'system', label: 'System Ops', icon: 'settings_system_daydream' },
  { id: 'ai', label: 'AI Tools', icon: 'psychology' },
  { id: 'cloud', label: 'Social & Cloud', icon: 'cloud' },
];

// Helper to check if a tool matches search
const matchesSearch = (keywords: string[]) => {
  if (!searchQuery.value) return true;
  const query = searchQuery.value.toLowerCase();
  return keywords.some(k => k.toLowerCase().includes(query));
};

const activeCategoryLabel = computed(() => categories.find(c => c.id === activeCategory.value)?.label);
</script>

<template>
  <div class="flex h-full gap-6 overflow-hidden p-6">
    <!-- Sidebar Navigation -->
    <div class="w-64 flex-shrink-0 bg-surface/30 border border-white/10 rounded-2xl backdrop-blur-xl flex flex-col overflow-hidden">
      <div class="p-4 border-b border-white/10">
        <h2 class="text-lg font-bold text-white tracking-tight">Toolbox</h2>
        <p class="text-xs text-text-secondary">Utilities & Generators</p>
      </div>
      
      <div class="p-3 border-b border-white/10">
        <GlassInput 
          v-model="searchQuery" 
          placeholder="Search tools..." 
          icon="search"
          class="w-full text-sm"
        />
      </div>

      <div class="flex-1 overflow-y-auto p-2 space-y-1 custom-scrollbar">
        <SidebarItem 
          v-for="cat in categories" 
          :key="cat.id"
          :icon="cat.icon"
          :label="cat.label"
          :active="activeCategory === cat.id && !searchQuery"
          @click="activeCategory = cat.id; searchQuery = ''"
        />
      </div>
    </div>

    <!-- Main Content Area -->
    <div class="flex-1 flex flex-col min-w-0 bg-surface/30 border border-white/10 rounded-2xl backdrop-blur-xl overflow-hidden">
      <!-- Header -->
      <div class="p-6 border-b border-white/10 flex justify-between items-center bg-white/5">
        <div>
          <h2 class="text-xl font-bold text-white tracking-tight">
            {{ searchQuery ? `Search Results for "${searchQuery}"` : activeCategoryLabel }}
          </h2>
          <p class="text-sm text-text-secondary">
            {{ searchQuery ? 'Tools matching your query' : 'Select a tool to configure' }}
          </p>
        </div>
      </div>

      <!-- Scrollable Content -->
      <div class="flex-1 overflow-y-auto p-6 custom-scrollbar">
        
        <!-- Automation Tab -->
        <div v-show="(!searchQuery && activeCategory === 'automation') || searchQuery" class="grid grid-cols-1 xl:grid-cols-2 gap-6">
          <MultiDeviceGrid v-if="matchesSearch(['multi', 'device', 'grid', 'automation'])" class="xl:col-span-2" />
          <ZeroTouchPanel v-if="matchesSearch(['zero', 'touch', 'provisioning', 'automation'])" class="xl:col-span-2" />
          <CloudBuildSync v-if="matchesSearch(['cloud', 'build', 'sync', 'github', 'automation'])" class="xl:col-span-2" />
          <RecoveryScriptGen v-if="matchesSearch(['recovery', 'script', 'twrp', 'automation'])" class="xl:col-span-2" />
          <ApkBatchInstaller v-if="matchesSearch(['apk', 'batch', 'installer', 'automation'])" />
          <Debloater v-if="matchesSearch(['debloater', 'remove', 'apps', 'automation'])" />
          <ModuleManager v-if="matchesSearch(['module', 'magisk', 'kernelsu', 'automation'])" />
          <PropEditor v-if="matchesSearch(['prop', 'editor', 'build.prop', 'automation'])" />
          <GAppsIntegrator v-if="matchesSearch(['gapps', 'google', 'apps', 'automation'])" />
          <FirmwareMatcher v-if="matchesSearch(['firmware', 'matcher', 'automation'])" />
          <ConfigGenerator v-if="matchesSearch(['config', 'generator', 'yaml', 'automation'])" />
        </div>

        <!-- Security Tab -->
        <div v-show="(!searchQuery && activeCategory === 'security') || searchQuery" class="grid grid-cols-1 xl:grid-cols-2 gap-6">
          <PrivacyDashboard v-if="matchesSearch(['privacy', 'dashboard', 'permissions', 'security'])" class="xl:col-span-2" />
          <HashVerifier v-if="matchesSearch(['hash', 'verifier', 'md5', 'sha256', 'security'])" />
          <EFSGuard v-if="matchesSearch(['efs', 'guard', 'backup', 'imei', 'security'])" />
          <SafetyNetFixer v-if="matchesSearch(['safetynet', 'fix', 'integrity', 'security'])" />
          <BootloaderLocker v-if="matchesSearch(['bootloader', 'locker', 'relock', 'security'])" />
          <KillSwitch v-if="matchesSearch(['kill', 'switch', 'panic', 'security'])" />
        </div>

        <!-- System Tab -->
        <div v-show="(!searchQuery && activeCategory === 'system') || searchQuery" class="grid grid-cols-1 xl:grid-cols-2 gap-6">
          <PartitionResizer v-if="matchesSearch(['partition', 'resizer', 'system'])" />
          <KernelSUInstaller v-if="matchesSearch(['kernelsu', 'root', 'system'])" />
          <DualBootMgr v-if="matchesSearch(['dual', 'boot', 'slot', 'a/b', 'system'])" />
          <PayloadStreamer v-if="matchesSearch(['payload', 'streamer', 'extract', 'system'])" />
        </div>

        <!-- AI Tab -->
        <div v-show="(!searchQuery && activeCategory === 'ai') || searchQuery" class="grid grid-cols-1 xl:grid-cols-2 gap-6">
          <SentimentAnalyzer v-if="matchesSearch(['sentiment', 'analyzer', 'ai'])" />
          <PerformanceTuner v-if="matchesSearch(['performance', 'tuner', 'ai'])" />
          <BatteryAI v-if="matchesSearch(['battery', 'ai', 'health'])" />
          <NetworkOptimizer v-if="matchesSearch(['network', 'optimizer', 'ai'])" />
        </div>

        <!-- Cloud Tab -->
        <div v-show="(!searchQuery && activeCategory === 'cloud') || searchQuery" class="grid grid-cols-1 xl:grid-cols-2 gap-6">
          <CommunityRepo v-if="matchesSearch(['community', 'repo', 'roms', 'cloud'])" class="xl:col-span-2" />
          <ConfigSharer v-if="matchesSearch(['config', 'sharer', 'cloud'])" />
          <DevProfileSync v-if="matchesSearch(['dev', 'profile', 'sync', 'cloud'])" />
        </div>

      </div>
    </div>
  </div>
</template>
