<script setup lang="ts">
import { ref, computed } from 'vue';
import { useSettingsStore } from '../../../stores/settings';
import ToggleSwitch from '../../ui/ToggleSwitch.vue';
import ThemeEngine from './ThemeEngine.vue';
import SidebarItem from '../../ui/SidebarItem.vue';
import GlassCard from '../../ui/GlassCard.vue';

const settings = useSettingsStore();
const activeCategory = ref('general');

const categories = [
  { id: 'general', label: 'General', icon: 'tune' },
  { id: 'appearance', label: 'Appearance', icon: 'palette' },
  { id: 'flashing', label: 'Flashing & Safety', icon: 'security' },
  { id: 'adb', label: 'ADB & Fastboot', icon: 'adb' },
  { id: 'performance', label: 'Performance', icon: 'speed' },
  { id: 'automation', label: 'Automation', icon: 'smart_toy' },
  { id: 'network', label: 'Network', icon: 'wifi' },
];

const activeCategoryLabel = computed(() => categories.find(c => c.id === activeCategory.value)?.label);

const isSaving = ref(false);

const handleSave = async () => {
  isSaving.value = true;
  try {
    await settings.saveSettings();
  } catch (e) {
    console.error(e);
  } finally {
    setTimeout(() => isSaving.value = false, 1000);
  }
};
</script>

<template>
  <div class="flex h-full gap-6 overflow-hidden">
    <!-- Sidebar Navigation -->
    <GlassCard noPadding class="w-64 flex-shrink-0 flex flex-col overflow-hidden">
      <div class="p-5 border-b border-white/10 bg-white/5 backdrop-blur-md">
        <h2 class="text-lg font-bold text-white tracking-tight flex items-center gap-2">
          <span class="material-symbols-rounded text-primary">settings</span>
          Settings
        </h2>
        <p class="text-xs text-text-secondary mt-1">Configure CyberFlash</p>
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
      
      <div class="p-4 border-t border-white/10 bg-white/5 backdrop-blur-md">
        <div class="flex items-center gap-3 p-2 rounded-xl bg-black/20 border border-white/5">
          <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-primary to-secondary flex items-center justify-center text-white font-bold text-xs shadow-lg">
            CF
          </div>
          <div>
            <div class="text-xs font-bold text-white">CyberFlash Pro</div>
            <div class="text-[10px] text-text-secondary font-mono">v2.0.1 (Build 2025.12)</div>
          </div>
        </div>
      </div>
    </GlassCard>

    <!-- Main Content Area -->
    <GlassCard noPadding class="flex-1 flex flex-col overflow-hidden relative">
      <!-- Header -->
      <div class="p-6 border-b border-white/10 flex justify-between items-center bg-white/5 backdrop-blur-md z-10">
        <div>
          <h3 class="text-xl font-bold text-white flex items-center gap-2">
            {{ activeCategoryLabel }}
            <span class="px-2 py-0.5 rounded-full bg-primary/10 text-primary text-[10px] font-bold border border-primary/20 uppercase tracking-wider">Config</span>
          </h3>
          <p class="text-sm text-text-secondary mt-1">Manage your {{ activeCategoryLabel?.toLowerCase() }} preferences</p>
        </div>
        <div class="flex items-center gap-3">
          <button 
            @click="handleSave" 
            class="px-5 py-2.5 rounded-xl bg-primary hover:bg-primary-hover text-white text-sm font-bold transition-all shadow-lg shadow-primary/20 flex items-center gap-2 active:scale-95 group"
            :disabled="isSaving"
          >
            <span class="material-symbols-rounded text-lg group-hover:rotate-180 transition-transform duration-500" :class="{'animate-spin': isSaving}">{{ isSaving ? 'sync' : 'save' }}</span>
            {{ isSaving ? 'Saving...' : 'Save Changes' }}
          </button>
        </div>
      </div>

      <!-- Scrollable Content -->
      <div class="flex-1 overflow-y-auto p-8 custom-scrollbar bg-gradient-to-b from-transparent to-black/20">
        
        <!-- General Settings -->
        <div v-if="activeCategory === 'general'" class="space-y-8 animate-slide-up">
          <section>
            <h4 class="text-xs font-bold text-primary uppercase tracking-widest mb-4 flex items-center gap-2">
              <span class="w-8 h-[1px] bg-primary/50"></span>
              User Interface
            </h4>
            <div class="grid grid-cols-1 gap-4">
              <div class="bg-white/5 rounded-2xl p-1 border border-white/5 hover:border-white/10 transition-colors hover-tilt-subtle">
                <div class="bg-black/20 rounded-xl px-4 py-2">
                  <ToggleSwitch v-model="settings.enableAnimations" label="Enable Animations" description="Smooth transitions and motion effects" />
                  <div class="h-px bg-white/5 my-1"></div>
                  <ToggleSwitch v-model="settings.soundEffects" label="Sound Effects" description="Play sounds on interactions and completion" />
                  <div class="h-px bg-white/5 my-1"></div>
                  <ToggleSwitch v-model="settings.hapticFeedback" label="Haptic Feedback" description="Vibrate on important actions" />
                  <div class="h-px bg-white/5 my-1"></div>
                  <ToggleSwitch v-model="settings.compactMode" label="Compact Mode" description="Reduce padding for higher information density" />
                </div>
              </div>
            </div>
          </section>

          <section>
            <h4 class="text-xs font-bold text-primary uppercase tracking-widest mb-4 flex items-center gap-2">
              <span class="w-8 h-[1px] bg-primary/50"></span>
              Workflow
            </h4>
            <div class="bg-white/5 rounded-2xl p-1 border border-white/5 hover:border-white/10 transition-colors hover-tilt-subtle">
              <div class="bg-black/20 rounded-xl px-4 py-2">
                <ToggleSwitch v-model="settings.showTerminalOverlay" label="Show Terminal Overlay" description="Display command output in a floating window" />
                <div class="h-px bg-white/5 my-1"></div>
                <ToggleSwitch v-model="settings.checkUpdates" label="Auto-Check Updates" description="Check for app updates on startup" />
                <div class="h-px bg-white/5 my-1"></div>
                <ToggleSwitch v-model="settings.sendStats" label="Anonymous Statistics" description="Help improve CyberFlash by sending usage data" />
              </div>
            </div>
          </section>
        </div>

        <!-- Appearance Settings -->
        <div v-if="activeCategory === 'appearance'" class="space-y-8 animate-slide-up">
          <ThemeEngine />
          
          <section>
            <h4 class="text-xs font-bold text-primary uppercase tracking-widest mb-4 flex items-center gap-2">
              <span class="w-8 h-[1px] bg-primary/50"></span>
              Advanced Visuals
            </h4>
            <div class="bg-white/5 rounded-2xl p-1 border border-white/5 hover:border-white/10 transition-colors hover-tilt-subtle">
              <div class="bg-black/20 rounded-xl px-4 py-2">
                <ToggleSwitch v-model="settings.showMeshGradient" label="Mesh Gradients" description="Show dynamic background gradients" />
                <div class="h-px bg-white/5 my-1"></div>
                <ToggleSwitch v-model="settings.cardHoverEffects" label="Hover Effects" description="Highlight cards when hovering" />
                <div class="h-px bg-white/5 my-1"></div>
                <ToggleSwitch v-model="settings.buttonGlow" label="Button Glow" description="Add neon glow to primary buttons" />
                <div class="h-px bg-white/5 my-1"></div>
                <ToggleSwitch v-model="settings.rippleEffects" label="Ripple Effects" description="Material-style click ripples" />
              </div>
            </div>
          </section>
        </div>

        <!-- Flashing & Safety -->
        <div v-if="activeCategory === 'flashing'" class="space-y-8 animate-slide-up">
          <section>
            <h4 class="text-xs font-bold text-primary uppercase tracking-widest mb-4 flex items-center gap-2">
              <span class="w-8 h-[1px] bg-primary/50"></span>
              Safety Checks
            </h4>
            <div class="bg-white/5 rounded-2xl p-1 border border-white/5 hover:border-white/10 transition-colors hover-tilt-subtle">
              <div class="bg-black/20 rounded-xl px-4 py-2">
                <ToggleSwitch v-model="settings.verifyMd5Checksum" label="Verify MD5 Checksums" description="Ensure downloaded files are not corrupted" />
                <div class="h-px bg-white/5 my-1"></div>
                <ToggleSwitch v-model="settings.checkBatteryLevel" label="Check Battery Level" description="Prevent flashing if battery is low (<30%)" />
                <div class="h-px bg-white/5 my-1"></div>
                <ToggleSwitch v-model="settings.checkDeviceCompatibility" label="Device Compatibility" description="Verify ROM matches device codename" />
                <div class="h-px bg-white/5 my-1"></div>
                <ToggleSwitch v-model="settings.preventDowngrade" label="Prevent Downgrade" description="Warn before flashing older firmware versions" />
              </div>
            </div>
          </section>

          <section>
            <h4 class="text-xs font-bold text-primary uppercase tracking-widest mb-4 flex items-center gap-2">
              <span class="w-8 h-[1px] bg-primary/50"></span>
              Backup Strategy
            </h4>
            <div class="bg-white/5 rounded-2xl p-1 border border-white/5 hover:border-white/10 transition-colors hover-tilt-subtle">
              <div class="bg-black/20 rounded-xl px-4 py-2">
                <ToggleSwitch v-model="settings.backupEFS" label="Auto-Backup EFS" description="Backup IMEI/Signal partitions before flashing" />
                <div class="h-px bg-white/5 my-1"></div>
                <ToggleSwitch v-model="settings.backupPersist" label="Backup Persist" description="Backup sensor calibration data" />
              </div>
            </div>
          </section>
        </div>

        <!-- ADB & Fastboot -->
        <div v-if="activeCategory === 'adb'" class="space-y-8 animate-slide-up">
          <section>
            <h4 class="text-xs font-bold text-primary uppercase tracking-widest mb-4 flex items-center gap-2">
              <span class="w-8 h-[1px] bg-primary/50"></span>
              Connection
            </h4>
            <div class="bg-white/5 rounded-2xl p-1 border border-white/5 hover:border-white/10 transition-colors hover-tilt-subtle">
              <div class="bg-black/20 rounded-xl px-4 py-2">
                <ToggleSwitch v-model="settings.useInternalServer" label="Use Internal ADB Server" description="Use bundled ADB instead of system ADB" />
                <div class="h-px bg-white/5 my-1"></div>
                <ToggleSwitch v-model="settings.killServerOnExit" label="Kill Server on Exit" description="Stop ADB server when closing app" />
                <div class="h-px bg-white/5 my-1"></div>
                <ToggleSwitch v-model="settings.usbBufferAlignment" label="USB Buffer Alignment" description="Optimize large file transfers (Fastboot)" />
              </div>
            </div>
          </section>

          <section>
            <h4 class="text-xs font-bold text-primary uppercase tracking-widest mb-4 flex items-center gap-2">
              <span class="w-8 h-[1px] bg-primary/50"></span>
              Paths
            </h4>
            <div class="bg-white/5 rounded-2xl p-6 border border-white/5 hover:border-white/10 transition-colors space-y-4">
              <div>
                <label class="text-xs text-text-secondary block mb-2 font-bold uppercase tracking-wider">ADB Binary Path</label>
                <div class="flex gap-2">
                  <input v-model="settings.adbPath" type="text" class="flex-1 bg-black/40 border border-white/10 rounded-xl px-4 py-3 text-sm text-white font-mono focus:outline-none focus:border-primary/50 focus:ring-1 focus:ring-primary/50 transition-all" />
                  <button class="px-4 py-2 bg-white/5 hover:bg-white/10 border border-white/10 rounded-xl text-white text-xs font-bold transition-all hover:scale-105 active:scale-95">Browse</button>
                </div>
              </div>
              <div>
                <label class="text-xs text-text-secondary block mb-2 font-bold uppercase tracking-wider">Fastboot Binary Path</label>
                <div class="flex gap-2">
                  <input v-model="settings.fastbootPath" type="text" class="flex-1 bg-black/40 border border-white/10 rounded-xl px-4 py-3 text-sm text-white font-mono focus:outline-none focus:border-primary/50 focus:ring-1 focus:ring-primary/50 transition-all" />
                  <button class="px-4 py-2 bg-white/5 hover:bg-white/10 border border-white/10 rounded-xl text-white text-xs font-bold transition-all hover:scale-105 active:scale-95">Browse</button>
                </div>
              </div>
            </div>
          </section>
        </div>

        <!-- Performance Settings (New) -->
        <div v-if="activeCategory === 'performance'" class="space-y-8 animate-slide-up">
          <section>
            <h4 class="text-sm font-bold text-primary uppercase tracking-wider mb-4">System Resources</h4>
            <div class="space-y-1 bg-black/20 rounded-xl p-4 border border-white/5">
              <ToggleSwitch v-model="settings.hardwareAcceleration" label="Hardware Acceleration" description="Use GPU for smoother UI rendering" />
              <div class="h-px bg-white/5 my-1"></div>
              <ToggleSwitch v-model="settings.useGpuRendering" label="Force GPU Rendering" description="Offload heavy tasks to graphics card" />
              <div class="h-px bg-white/5 my-1"></div>
              <ToggleSwitch v-model="settings.backgroundProcessing" label="Background Processing" description="Continue tasks when window is minimized" />
              <div class="h-px bg-white/5 my-1"></div>
              <ToggleSwitch v-model="settings.lowSpecMode" label="Low Spec Mode" description="Reduce visual effects for better performance on older hardware" />
            </div>
          </section>

          <section>
            <h4 class="text-sm font-bold text-primary uppercase tracking-wider mb-4">Process Management</h4>
            <div class="space-y-4 bg-black/20 rounded-xl p-4 border border-white/5">
              <div>
                <label class="text-xs text-text-secondary block mb-2">Process Priority</label>
                <select v-model="settings.processPriority" class="w-full bg-black/40 border border-white/10 rounded-lg px-3 py-2 text-sm text-white focus:border-primary/50 outline-none">
                  <option value="normal">Normal</option>
                  <option value="high">High (Recommended)</option>
                  <option value="realtime">Realtime (Experimental)</option>
                </select>
              </div>
              
              <div>
                <label class="text-xs text-text-secondary block mb-2">Max Concurrent Downloads</label>
                <input type="range" v-model="settings.maxConcurrentDownloads" min="1" max="10" class="w-full accent-primary h-1 bg-white/10 rounded-lg appearance-none cursor-pointer">
                <div class="flex justify-between text-xs text-text-muted mt-1">
                  <span>1</span>
                  <span class="text-white font-bold">{{ settings.maxConcurrentDownloads }}</span>
                  <span>10</span>
                </div>
              </div>
            </div>
          </section>

          <section>
            <h4 class="text-sm font-bold text-primary uppercase tracking-wider mb-4">Cache & Storage</h4>
            <div class="space-y-1 bg-black/20 rounded-xl p-4 border border-white/5">
              <ToggleSwitch v-model="settings.autoClearCache" label="Auto-Clear Cache" description="Remove temporary files on exit" />
              <div class="h-px bg-white/5 my-1"></div>
              <div class="pt-2">
                <label class="text-xs text-text-secondary block mb-2">Cache Size Limit (MB)</label>
                <div class="flex gap-2">
                  <input type="number" v-model="settings.cacheSizeLimit" class="flex-1 bg-black/40 border border-white/10 rounded-lg px-3 py-2 text-sm text-white focus:border-primary/50 outline-none">
                  <button class="px-4 py-2 bg-white/10 hover:bg-white/20 rounded-lg text-xs text-white transition-colors">Clear Now</button>
                </div>
              </div>
            </div>
          </section>
        </div>

        <!-- Automation Settings -->
        <div v-if="activeCategory === 'automation'" class="space-y-8 animate-slide-up">
          <section>
            <h4 class="text-sm font-bold text-primary uppercase tracking-wider mb-4">AI Configuration</h4>
            <div class="space-y-4 bg-black/20 rounded-xl p-4 border border-white/5">
              <ToggleSwitch v-model="settings.aiEnabled" label="Enable AI Assistant" description="Allow Gemini to help with tasks" />
              
              <div v-if="settings.aiEnabled" class="space-y-4 pt-2">
                <div>
                  <label class="text-xs text-text-secondary block mb-2">Gemini API Key</label>
                  <input 
                    v-model="settings.geminiApiKey" 
                    type="password" 
                    placeholder="Enter your Google AI Studio Key" 
                    class="w-full bg-black/40 border border-white/10 rounded-lg px-3 py-2 text-sm text-white focus:border-primary/50 outline-none"
                  >
                  <p class="text-[10px] text-text-muted mt-1">Required for Gemini 1.5 Flash integration.</p>
                </div>
                
                <div>
                  <label class="text-xs text-text-secondary block mb-2">Model Selection</label>
                  <select v-model="settings.aiModel" class="w-full bg-black/40 border border-white/10 rounded-lg px-3 py-2 text-sm text-white focus:border-primary/50 outline-none">
                    <option value="gemini-1.5-flash">Gemini 1.5 Flash (Fast & Efficient)</option>
                    <option value="gemini-1.5-pro">Gemini 1.5 Pro (More Capable)</option>
                  </select>
                </div>

                <div class="h-px bg-white/5 my-2"></div>
                
                <ToggleSwitch v-model="settings.autoAnalyzeLogs" label="Auto-Analyze Logs" description="AI will check logs for errors automatically" />
                <div class="h-px bg-white/5 my-1"></div>
                <ToggleSwitch v-model="settings.suggestNextSteps" label="Suggest Next Steps" description="Proactive guidance during flash process" />
              </div>
            </div>
          </section>

          <section>
            <h4 class="text-sm font-bold text-primary uppercase tracking-wider mb-4">Post-Flash Actions</h4>
            <div class="space-y-1 bg-black/20 rounded-xl p-4 border border-white/5">
              <ToggleSwitch v-model="settings.wipeCache" label="Wipe Cache/Dalvik" description="Clear cache after flashing" />
              <div class="h-px bg-white/5 my-1"></div>
              <ToggleSwitch v-model="settings.rebootAfterFlash" label="Auto Reboot" description="Reboot device when queue finishes" />
              <div class="h-px bg-white/5 my-1"></div>
              <ToggleSwitch v-model="settings.disableVerity" label="Disable Verity/Vbmeta" description="Patch vbmeta to prevent bootloops on custom ROMs" />
              <div class="h-px bg-white/5 my-1"></div>
              <ToggleSwitch v-model="settings.disableEncryption" label="Disable Encryption (DFE)" description="Decrypt internal storage (Requires Format Data)" />
            </div>
          </section>
        </div>

        <!-- Network -->
        <div v-if="activeCategory === 'network'" class="space-y-8 animate-slide-up">
           <section>
            <h4 class="text-sm font-bold text-primary uppercase tracking-wider mb-4">Proxy & Connection</h4>
            <div class="space-y-1 bg-black/20 rounded-xl p-4 border border-white/5">
              <ToggleSwitch v-model="settings.proxyEnabled" label="Enable Proxy" description="Route traffic through a proxy server" />
              
              <div v-if="settings.proxyEnabled" class="mt-4 pt-4 border-t border-white/10 animate-in fade-in slide-in-from-top-2">
                <label class="text-xs text-text-secondary block mb-1">Proxy Address</label>
                <input v-model="settings.proxyAddress" placeholder="http://127.0.0.1:8080" type="text" class="w-full bg-surface/50 border border-white/10 rounded-lg px-3 py-2 text-sm text-white font-mono focus:outline-none focus:border-primary/50" />
              </div>
            </div>
          </section>
        </div>

      </div>
    </GlassCard>
  </div>
</template>

<style scoped>
/* Custom Scrollbar for the panel */
.custom-scrollbar::-webkit-scrollbar {
  width: 6px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 3px;
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
