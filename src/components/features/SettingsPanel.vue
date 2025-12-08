<script setup lang="ts">
import { ref, computed } from 'vue';
import { useSettingsStore } from '../../stores/settings';
import ToggleSwitch from '../ui/ToggleSwitch.vue';
import ThemeEngine from './ThemeEngine.vue';

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
    <div class="w-64 flex-shrink-0 bg-surface/30 border border-white/10 rounded-2xl backdrop-blur-xl flex flex-col overflow-hidden">
      <div class="p-4 border-b border-white/10">
        <h2 class="text-lg font-bold text-white tracking-tight">Settings</h2>
        <p class="text-xs text-text-secondary">Configure CyberFlash</p>
      </div>
      
      <div class="flex-1 overflow-y-auto p-2 space-y-1 custom-scrollbar">
        <button 
          v-for="cat in categories" 
          :key="cat.id"
          @click="activeCategory = cat.id"
          class="w-full flex items-center gap-3 px-4 py-3 rounded-xl transition-all duration-200 text-left group relative overflow-hidden"
          :class="activeCategory === cat.id ? 'bg-primary text-white shadow-lg shadow-primary/20' : 'text-text-secondary hover:bg-white/5 hover:text-white'"
        >
          <span class="material-symbols-rounded transition-transform duration-300 group-hover:scale-110" :class="activeCategory === cat.id ? 'text-white' : 'text-text-secondary group-hover:text-white'">{{ cat.icon }}</span>
          <span class="font-medium">{{ cat.label }}</span>
          
          <!-- Active Indicator -->
          <div v-if="activeCategory === cat.id" class="absolute right-0 top-1/2 -translate-y-1/2 w-1 h-8 bg-white/50 rounded-l-full"></div>
        </button>
      </div>
      
      <div class="p-4 border-t border-white/10 bg-white/5">
        <div class="flex items-center gap-3">
          <div class="w-8 h-8 rounded-full bg-gradient-to-br from-primary to-secondary flex items-center justify-center text-white font-bold text-xs">
            CF
          </div>
          <div>
            <div class="text-xs font-bold text-white">CyberFlash Pro</div>
            <div class="text-[10px] text-text-secondary">v2.0.1 (Build 2025.12)</div>
          </div>
        </div>
      </div>
    </div>

    <!-- Main Content Area -->
    <div class="flex-1 bg-surface/30 border border-white/10 rounded-2xl backdrop-blur-xl flex flex-col overflow-hidden relative">
      <!-- Header -->
      <div class="p-6 border-b border-white/10 flex justify-between items-center bg-white/5">
        <div>
          <h3 class="text-xl font-bold text-white">{{ activeCategoryLabel }}</h3>
          <p class="text-sm text-text-secondary">Manage your {{ activeCategoryLabel?.toLowerCase() }} preferences</p>
        </div>
        <div class="flex items-center gap-3">
          <button 
            @click="handleSave" 
            class="px-4 py-2 rounded-lg bg-primary hover:bg-primary-hover text-white text-sm font-bold transition-all shadow-lg shadow-primary/20 flex items-center gap-2 active:scale-95"
            :disabled="isSaving"
          >
            <span class="material-symbols-rounded text-lg" :class="{'animate-spin': isSaving}">{{ isSaving ? 'sync' : 'save' }}</span>
            {{ isSaving ? 'Saving...' : 'Save Changes' }}
          </button>
          <button class="p-2 rounded-full hover:bg-white/10 text-text-secondary hover:text-white transition-colors">
            <span class="material-symbols-rounded">help</span>
          </button>
        </div>
      </div>

      <!-- Scrollable Content -->
      <div class="flex-1 overflow-y-auto p-6 custom-scrollbar">
        
        <!-- General Settings -->
        <div v-if="activeCategory === 'general'" class="space-y-8 animate-in fade-in slide-in-from-bottom-4 duration-300">
          <section>
            <h4 class="text-sm font-bold text-primary uppercase tracking-wider mb-4">User Interface</h4>
            <div class="space-y-1 bg-black/20 rounded-xl p-4 border border-white/5">
              <ToggleSwitch v-model="settings.enableAnimations" label="Enable Animations" description="Smooth transitions and motion effects" />
              <div class="h-px bg-white/5 my-1"></div>
              <ToggleSwitch v-model="settings.soundEffects" label="Sound Effects" description="Play sounds on interactions and completion" />
              <div class="h-px bg-white/5 my-1"></div>
              <ToggleSwitch v-model="settings.hapticFeedback" label="Haptic Feedback" description="Vibrate on important actions" />
              <div class="h-px bg-white/5 my-1"></div>
              <ToggleSwitch v-model="settings.compactMode" label="Compact Mode" description="Reduce padding for higher information density" />
            </div>
          </section>

          <section>
            <h4 class="text-sm font-bold text-primary uppercase tracking-wider mb-4">Workflow</h4>
            <div class="space-y-1 bg-black/20 rounded-xl p-4 border border-white/5">
              <ToggleSwitch v-model="settings.showTerminalOverlay" label="Show Terminal Overlay" description="Display command output in a floating window" />
              <div class="h-px bg-white/5 my-1"></div>
              <ToggleSwitch v-model="settings.checkUpdates" label="Auto-Check Updates" description="Check for app updates on startup" />
              <div class="h-px bg-white/5 my-1"></div>
              <ToggleSwitch v-model="settings.sendStats" label="Anonymous Statistics" description="Help improve CyberFlash by sending usage data" />
            </div>
          </section>
        </div>

        <!-- Appearance Settings -->
        <div v-if="activeCategory === 'appearance'" class="space-y-8 animate-in fade-in slide-in-from-bottom-4 duration-300">
          <ThemeEngine />
          
          <section>
            <h4 class="text-sm font-bold text-primary uppercase tracking-wider mb-4">Advanced Visuals</h4>
            <div class="space-y-1 bg-black/20 rounded-xl p-4 border border-white/5">
              <ToggleSwitch v-model="settings.showMeshGradient" label="Mesh Gradients" description="Show dynamic background gradients" />
              <div class="h-px bg-white/5 my-1"></div>
              <ToggleSwitch v-model="settings.cardHoverEffects" label="Hover Effects" description="Highlight cards when hovering" />
              <div class="h-px bg-white/5 my-1"></div>
              <ToggleSwitch v-model="settings.buttonGlow" label="Button Glow" description="Add neon glow to primary buttons" />
              <div class="h-px bg-white/5 my-1"></div>
              <ToggleSwitch v-model="settings.rippleEffects" label="Ripple Effects" description="Material-style click ripples" />
            </div>
          </section>
        </div>

        <!-- Flashing & Safety -->
        <div v-if="activeCategory === 'flashing'" class="space-y-8 animate-in fade-in slide-in-from-bottom-4 duration-300">
          <section>
            <h4 class="text-sm font-bold text-primary uppercase tracking-wider mb-4">Safety Checks</h4>
            <div class="space-y-1 bg-black/20 rounded-xl p-4 border border-white/5">
              <ToggleSwitch v-model="settings.verifyMd5Checksum" label="Verify MD5 Checksums" description="Ensure downloaded files are not corrupted" />
              <div class="h-px bg-white/5 my-1"></div>
              <ToggleSwitch v-model="settings.checkBatteryLevel" label="Check Battery Level" description="Prevent flashing if battery is low (<30%)" />
              <div class="h-px bg-white/5 my-1"></div>
              <ToggleSwitch v-model="settings.checkDeviceCompatibility" label="Device Compatibility" description="Verify ROM matches device codename" />
              <div class="h-px bg-white/5 my-1"></div>
              <ToggleSwitch v-model="settings.preventDowngrade" label="Prevent Downgrade" description="Warn before flashing older firmware versions" />
            </div>
          </section>

          <section>
            <h4 class="text-sm font-bold text-primary uppercase tracking-wider mb-4">Backup Strategy</h4>
            <div class="space-y-1 bg-black/20 rounded-xl p-4 border border-white/5">
              <ToggleSwitch v-model="settings.backupEFS" label="Auto-Backup EFS" description="Backup IMEI/Signal partitions before flashing" />
              <div class="h-px bg-white/5 my-1"></div>
              <ToggleSwitch v-model="settings.backupPersist" label="Backup Persist" description="Backup sensor calibration data" />
            </div>
          </section>
        </div>

        <!-- ADB & Fastboot -->
        <div v-if="activeCategory === 'adb'" class="space-y-8 animate-in fade-in slide-in-from-bottom-4 duration-300">
          <section>
            <h4 class="text-sm font-bold text-primary uppercase tracking-wider mb-4">Connection</h4>
            <div class="space-y-1 bg-black/20 rounded-xl p-4 border border-white/5">
              <ToggleSwitch v-model="settings.useInternalServer" label="Use Internal ADB Server" description="Use bundled ADB instead of system ADB" />
              <div class="h-px bg-white/5 my-1"></div>
              <ToggleSwitch v-model="settings.killServerOnExit" label="Kill Server on Exit" description="Stop ADB server when closing app" />
              <div class="h-px bg-white/5 my-1"></div>
              <ToggleSwitch v-model="settings.usbBufferAlignment" label="USB Buffer Alignment" description="Optimize large file transfers (Fastboot)" />
            </div>
          </section>

          <section>
            <h4 class="text-sm font-bold text-primary uppercase tracking-wider mb-4">Paths</h4>
            <div class="space-y-4 bg-black/20 rounded-xl p-4 border border-white/5">
              <div>
                <label class="text-xs text-text-secondary block mb-1">ADB Binary Path</label>
                <div class="flex gap-2">
                  <input v-model="settings.adbPath" type="text" class="flex-1 bg-surface/50 border border-white/10 rounded-lg px-3 py-2 text-sm text-white font-mono focus:outline-none focus:border-primary/50" />
                  <button class="px-3 py-2 bg-white/10 hover:bg-white/20 rounded-lg text-white text-xs font-bold transition-colors">Browse</button>
                </div>
              </div>
              <div>
                <label class="text-xs text-text-secondary block mb-1">Fastboot Binary Path</label>
                <div class="flex gap-2">
                  <input v-model="settings.fastbootPath" type="text" class="flex-1 bg-surface/50 border border-white/10 rounded-lg px-3 py-2 text-sm text-white font-mono focus:outline-none focus:border-primary/50" />
                  <button class="px-3 py-2 bg-white/10 hover:bg-white/20 rounded-lg text-white text-xs font-bold transition-colors">Browse</button>
                </div>
              </div>
            </div>
          </section>
        </div>

        <!-- Automation -->
        <!-- Performance Settings (New) -->
        <div v-if="activeCategory === 'performance'" class="space-y-8 animate-in fade-in slide-in-from-bottom-4 duration-300">
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
        <div v-if="activeCategory === 'automation'" class="space-y-8 animate-in fade-in slide-in-from-bottom-4 duration-300">
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
        <div v-if="activeCategory === 'network'" class="space-y-8 animate-in fade-in slide-in-from-bottom-4 duration-300">
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
    </div>
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
</style>
