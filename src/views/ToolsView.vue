<script setup lang="ts">
import { ref, computed } from 'vue';
import GlassInput from '../components/ui/GlassInput.vue';
import SidebarItem from '../components/ui/SidebarItem.vue';
import GlassCard from '../components/ui/GlassCard.vue';

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

// Utilities
import QuickActionsGrid from '../components/features/utilities/QuickActionsGrid.vue';

const activeCategory = ref('utilities');
const searchQuery = ref('');

const categories = [
  { id: 'utilities', label: 'Quick Actions', icon: 'bolt', color: 'text-yellow-400' },
  { id: 'automation', label: 'Automation', icon: 'smart_toy', color: 'text-blue-400' },
  { id: 'security', label: 'Security', icon: 'security', color: 'text-red-400' },
  { id: 'system', label: 'System Ops', icon: 'settings_system_daydream', color: 'text-purple-400' },
  { id: 'ai', label: 'AI Tools', icon: 'psychology', color: 'text-pink-400' },
  { id: 'cloud', label: 'Social & Cloud', icon: 'cloud', color: 'text-cyan-400' },
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
  <div class="flex h-full gap-6 overflow-hidden">
    <!-- Sidebar Navigation -->
    <GlassCard noPadding class="w-64 flex-shrink-0 flex flex-col overflow-hidden ring-1 ring-white/5 shadow-2xl shadow-black/20">
      <div class="p-5 border-b border-white/10 bg-surface/30 backdrop-blur-xl">
        <h2 class="text-lg font-bold text-white tracking-tight flex items-center gap-2">
          <span class="material-symbols-rounded text-warning drop-shadow-[0_0_8px_rgba(255,214,10,0.5)]">construction</span>
          Toolbox
        </h2>
        <p class="text-xs text-text-secondary mt-1 font-medium tracking-wide">Utilities & Generators</p>
      </div>
      
      <div class="p-3 border-b border-white/10 bg-surface/30 backdrop-blur-xl">
        <GlassInput 
          v-model="searchQuery" 
          placeholder="Search tools..." 
          icon="search"
          class="w-full text-sm"
        />
      </div>

      <div class="flex-1 overflow-y-auto p-3 space-y-1 custom-scrollbar">
        <SidebarItem 
          v-for="cat in categories" 
          :key="cat.id"
          :icon="cat.icon"
          :label="cat.label"
          :color="cat.color"
          :active="activeCategory === cat.id && !searchQuery"
          @click="activeCategory = cat.id; searchQuery = ''"
        />
      </div>
    </GlassCard>

    <!-- Main Content Area -->
    <GlassCard noPadding class="flex-1 flex flex-col min-w-0 overflow-hidden relative ring-1 ring-white/5 shadow-2xl shadow-black/20">
      <!-- Header -->
      <div class="p-6 border-b border-white/10 flex justify-between items-center bg-surface/30 backdrop-blur-xl z-10">
        <div>
          <h2 class="text-xl font-bold text-white tracking-tight flex items-center gap-2">
            {{ searchQuery ? `Search Results for "${searchQuery}"` : activeCategoryLabel }}
            <span v-if="!searchQuery" class="px-2 py-0.5 rounded-full bg-white/10 text-white/70 text-[10px] font-bold border border-white/10 uppercase tracking-wider shadow-[0_0_10px_rgba(255,255,255,0.1)]">Collection</span>
          </h2>
          <p class="text-sm text-text-secondary mt-1 font-medium">
            {{ searchQuery ? 'Tools matching your query' : 'Select a tool to configure' }}
          </p>
        </div>
      </div>

      <!-- Scrollable Content -->
      <div class="flex-1 overflow-y-auto p-8 custom-scrollbar relative">
        <!-- Mesh Background -->
        <div class="absolute inset-0 mesh-gradient-bg opacity-20 pointer-events-none"></div>
        
        <div class="relative z-10">
          <!-- Utilities Tab (Quick Actions) -->
        <div v-show="(!searchQuery && activeCategory === 'utilities') || searchQuery">
          <QuickActionsGrid v-if="matchesSearch(['quick', 'actions', 'utilities', 'adb', 'reboot'])" />
        </div>

        <!-- Automation Tab -->
        <div v-show="(!searchQuery && activeCategory === 'automation') || searchQuery" class="grid grid-cols-1 xl:grid-cols-2 gap-6 animate-slide-up">
          <MultiDeviceGrid v-if="matchesSearch(['multi', 'device', 'grid', 'automation'])" class="xl:col-span-2 hover-tilt" />
          <ZeroTouchPanel v-if="matchesSearch(['zero', 'touch', 'provisioning', 'automation'])" class="xl:col-span-2 hover-tilt" />
          <CloudBuildSync v-if="matchesSearch(['cloud', 'build', 'sync', 'github', 'automation'])" class="xl:col-span-2 hover-tilt" />
          <RecoveryScriptGen v-if="matchesSearch(['recovery', 'script', 'twrp', 'automation'])" class="xl:col-span-2 hover-tilt" />
          <ApkBatchInstaller v-if="matchesSearch(['apk', 'batch', 'installer', 'automation'])" class="hover-tilt" />
          <Debloater v-if="matchesSearch(['debloater', 'remove', 'apps', 'automation'])" class="hover-tilt" />
          <ModuleManager v-if="matchesSearch(['module', 'magisk', 'kernelsu', 'automation'])" class="hover-tilt" />
          <PropEditor v-if="matchesSearch(['prop', 'editor', 'build.prop', 'automation'])" class="hover-tilt" />
          <GAppsIntegrator v-if="matchesSearch(['gapps', 'google', 'apps', 'automation'])" class="hover-tilt" />
          <FirmwareMatcher v-if="matchesSearch(['firmware', 'matcher', 'automation'])" class="hover-tilt" />
          <ConfigGenerator v-if="matchesSearch(['config', 'generator', 'yaml', 'automation'])" class="hover-tilt" />
        </div>

        <!-- Security Tab -->
        <div v-show="(!searchQuery && activeCategory === 'security') || searchQuery" class="grid grid-cols-1 xl:grid-cols-2 gap-6 animate-slide-up">
          <PrivacyDashboard v-if="matchesSearch(['privacy', 'dashboard', 'permissions', 'security'])" class="xl:col-span-2 hover-tilt" />
          <HashVerifier v-if="matchesSearch(['hash', 'verifier', 'md5', 'sha256', 'security'])" class="hover-tilt" />
          <EFSGuard v-if="matchesSearch(['efs', 'guard', 'backup', 'imei', 'security'])" class="hover-tilt" />
          <SafetyNetFixer v-if="matchesSearch(['safetynet', 'fix', 'integrity', 'security'])" class="hover-tilt" />
          <BootloaderLocker v-if="matchesSearch(['bootloader', 'locker', 'relock', 'security'])" class="hover-tilt" />
          <KillSwitch v-if="matchesSearch(['kill', 'switch', 'panic', 'security'])" class="hover-tilt" />
        </div>

        <!-- System Tab -->
        <div v-show="(!searchQuery && activeCategory === 'system') || searchQuery" class="grid grid-cols-1 xl:grid-cols-2 gap-6 animate-slide-up">
          <PartitionResizer v-if="matchesSearch(['partition', 'resizer', 'system'])" class="hover-tilt" />
          <KernelSUInstaller v-if="matchesSearch(['kernelsu', 'root', 'system'])" class="hover-tilt" />
          <DualBootMgr v-if="matchesSearch(['dual', 'boot', 'slot', 'a/b', 'system'])" class="hover-tilt" />
          <PayloadStreamer v-if="matchesSearch(['payload', 'streamer', 'extract', 'system'])" class="hover-tilt" />
        </div>

        <!-- AI Tab -->
        <div v-show="(!searchQuery && activeCategory === 'ai') || searchQuery" class="grid grid-cols-1 xl:grid-cols-2 gap-6 animate-slide-up">
          <SentimentAnalyzer v-if="matchesSearch(['sentiment', 'analyzer', 'ai'])" class="hover-tilt" />
          <PerformanceTuner v-if="matchesSearch(['performance', 'tuner', 'ai'])" class="hover-tilt" />
          <BatteryAI v-if="matchesSearch(['battery', 'ai', 'health'])" class="hover-tilt" />
          <NetworkOptimizer v-if="matchesSearch(['network', 'optimizer', 'ai'])" class="hover-tilt" />
        </div>

        <!-- Cloud Tab -->
        <div v-show="(!searchQuery && activeCategory === 'cloud') || searchQuery" class="grid grid-cols-1 xl:grid-cols-2 gap-6 animate-slide-up">
          <CommunityRepo v-if="matchesSearch(['community', 'repo', 'roms', 'cloud'])" class="xl:col-span-2 hover-tilt" />
          <ConfigSharer v-if="matchesSearch(['config', 'sharer', 'cloud'])" class="hover-tilt" />
          <DevProfileSync v-if="matchesSearch(['dev', 'profile', 'sync', 'cloud'])" class="hover-tilt" />
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
  animation: slide-up 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards;
}

.hover-tilt {
  transition: transform 0.3s ease, box-shadow 0.3s ease;
}

.hover-tilt:hover {
  transform: translateY(-5px) scale(1.01);
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.3), 0 10px 10px -5px rgba(0, 0, 0, 0.2);
  z-index: 10;
}
</style>
