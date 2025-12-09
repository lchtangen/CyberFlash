<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useDynamicIslandStore } from '../../../stores/dynamicIsland';
import GlassCard from '../../ui/GlassCard.vue';
import ToggleSwitch from '../../ui/ToggleSwitch.vue';

const islandStore = useDynamicIslandStore();

// State
const wirelessAdb = ref(false);
const wirelessIp = ref('');
const dpiValue = ref('420');
const chargeLimit = ref(80);
const shellCmd = ref('');
const scrcpyBitrate = ref('8M');
const logDuration = ref('10s');
const pkgName = ref('');

const reboot = async (mode: string) => {
  islandStore.showNotification({
    id: 'reboot',
    type: 'process',
    title: `Rebooting to ${mode}...`,
    subtitle: 'Waiting for device',
    icon: 'restart_alt',
    bg: 'bg-warning/20',
    border: 'border-warning/30',
    color: 'text-warning'
  }, 4000);
  
  try {
    await invoke('reboot_device', { mode });
  } catch (e) {
    console.error(e);
  }
};

const runShell = async () => {
  if (!shellCmd.value) return;
  
  islandStore.showNotification({
    id: 'shell',
    type: 'process',
    title: 'Executing Shell Command',
    subtitle: shellCmd.value,
    icon: 'terminal',
    bg: 'bg-white/10',
    border: 'border-white/20',
    color: 'text-white'
  }, 2000);
  
  try {
    await invoke('run_adb_shell', { command: shellCmd.value });
    shellCmd.value = '';
  } catch (e) {
    console.error(e);
  }
};

const syncClipboard = async (direction: 'get' | 'set') => {
  // Not implemented in backend yet, keeping mock for now or implementing later
  islandStore.showNotification({
    id: 'clipboard',
    type: 'success',
    title: 'Clipboard Synced',
    subtitle: direction === 'get' ? 'Copied from device' : 'Sent to device',
    icon: 'content_paste',
    bg: 'bg-success/20',
    border: 'border-success/30',
    color: 'text-success'
  }, 2000);
};

</script>

<template>
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 animate-slide-up">
    
    <!-- 1. Power Menu -->
      <GlassCard class="p-5 flex flex-col gap-4 hover-tilt group">
        <div class="flex items-center gap-3 text-white mb-1">
          <div class="w-8 h-8 rounded-lg bg-primary/20 flex items-center justify-center text-primary group-hover:scale-110 transition-transform">
            <span class="material-symbols-rounded">power_settings_new</span>
          </div>
          <span class="text-sm font-bold tracking-wide">Power Menu</span>
        </div>
        <div class="grid grid-cols-2 gap-2">
          <button @click="reboot('system')" class="p-2.5 rounded-lg bg-white/5 hover:bg-white/10 border border-white/5 text-xs font-medium text-white transition-all hover:border-white/20 active:scale-95">System</button>
          <button @click="reboot('recovery')" class="p-2.5 rounded-lg bg-white/5 hover:bg-white/10 border border-white/5 text-xs font-medium text-white transition-all hover:border-white/20 active:scale-95">Recovery</button>
          <button @click="reboot('bootloader')" class="p-2.5 rounded-lg bg-white/5 hover:bg-white/10 border border-white/5 text-xs font-medium text-white transition-all hover:border-white/20 active:scale-95">Bootloader</button>
          <button @click="reboot('edl')" class="p-2.5 rounded-lg bg-error/10 hover:bg-error/20 border border-error/10 text-xs font-bold text-error transition-all hover:border-error/30 active:scale-95">EDL Mode</button>
        </div>
      </GlassCard>

      <!-- 2. Wireless ADB -->
      <GlassCard class="p-5 flex flex-col gap-4 hover-tilt group">
        <div class="flex items-center justify-between text-white mb-1">
          <div class="flex items-center gap-3">
            <div class="w-8 h-8 rounded-lg bg-success/20 flex items-center justify-center text-success group-hover:scale-110 transition-transform">
              <span class="material-symbols-rounded">wifi_tethering</span>
            </div>
            <span class="text-sm font-bold tracking-wide">Wireless ADB</span>
          </div>
          <ToggleSwitch v-model="wirelessAdb" class="scale-90 origin-right" />
        </div>
        <div v-if="wirelessAdb" class="bg-black/40 rounded-lg p-3 text-center border border-success/20 shadow-[0_0_15px_rgba(48,209,88,0.1)]">
          <code class="text-xs text-success font-mono tracking-wider">{{ wirelessIp }}</code>
        </div>
        <div v-else class="text-xs text-white/40 text-center py-2 font-medium">
          Connect via USB first to enable
        </div>
      </GlassCard>

      <!-- 3. Scrcpy Launcher -->
      <GlassCard class="p-5 flex flex-col gap-4 hover-tilt group">
        <div class="flex items-center gap-3 text-white mb-1">
          <div class="w-8 h-8 rounded-lg bg-blue-400/20 flex items-center justify-center text-blue-400 group-hover:scale-110 transition-transform">
            <span class="material-symbols-rounded">cast_connected</span>
          </div>
          <span class="text-sm font-bold tracking-wide">Screen Mirror</span>
        </div>
        <div class="flex gap-2">
          <div class="relative flex-1">
             <select v-model="scrcpyBitrate" class="w-full bg-white/5 border border-white/10 rounded-lg text-xs text-white p-2.5 outline-none appearance-none focus:border-primary/50 transition-colors">
              <option value="4M" class="bg-surface">4 Mbps (Low)</option>
              <option value="8M" class="bg-surface">8 Mbps (Med)</option>
              <option value="16M" class="bg-surface">16 Mbps (High)</option>
            </select>
            <span class="material-symbols-rounded absolute right-2 top-1/2 -translate-y-1/2 text-white/30 text-sm pointer-events-none">expand_more</span>
          </div>
          
          <button class="px-4 py-2 bg-primary text-white text-xs font-bold rounded-lg shadow-lg shadow-primary/20 hover:bg-primary-hover transition-all active:scale-95">
            START
          </button>
        </div>
      </GlassCard>

      <!-- 4. DPI Changer -->
      <GlassCard class="p-5 flex flex-col gap-4 hover-tilt group">
        <div class="flex items-center gap-3 text-white mb-1">
          <div class="w-8 h-8 rounded-lg bg-purple-400/20 flex items-center justify-center text-purple-400 group-hover:scale-110 transition-transform">
            <span class="material-symbols-rounded">aspect_ratio</span>
          </div>
          <span class="text-sm font-bold tracking-wide">DPI Changer</span>
        </div>
        <div class="flex gap-2">
          <input v-model="dpiValue" type="number" class="bg-white/5 border border-white/10 rounded-lg text-xs text-white p-2.5 w-full outline-none focus:border-purple-400/50 transition-colors placeholder-white/20" placeholder="e.g. 420">
          <button class="px-4 bg-white/10 hover:bg-white/20 text-white text-xs font-bold rounded-lg border border-white/10 transition-all active:scale-95">
            SET
          </button>
        </div>
      </GlassCard>

      <!-- 5. Log Snap -->
      <GlassCard class="p-5 flex flex-col gap-4 hover-tilt group">
        <div class="flex items-center gap-3 text-white mb-1">
          <div class="w-8 h-8 rounded-lg bg-orange-400/20 flex items-center justify-center text-orange-400 group-hover:scale-110 transition-transform">
            <span class="material-symbols-rounded">bug_report</span>
          </div>
          <span class="text-sm font-bold tracking-wide">Log Snap</span>
        </div>
        <div class="flex gap-2">
          <div class="relative w-24">
            <select v-model="logDuration" class="w-full bg-white/5 border border-white/10 rounded-lg text-xs text-white p-2.5 outline-none appearance-none focus:border-orange-400/50 transition-colors">
              <option value="10s" class="bg-surface">10s</option>
              <option value="30s" class="bg-surface">30s</option>
              <option value="1m" class="bg-surface">1m</option>
            </select>
             <span class="material-symbols-rounded absolute right-2 top-1/2 -translate-y-1/2 text-white/30 text-sm pointer-events-none">expand_more</span>
          </div>
          <button class="flex-1 bg-white/10 hover:bg-white/20 text-white text-xs font-bold rounded-lg border border-white/10 transition-all flex items-center justify-center gap-2 active:scale-95">
            <span class="material-symbols-rounded text-[16px]">download</span>
            SAVE
          </button>
        </div>
      </GlassCard>

      <!-- 6. Clipboard Sync -->
      <GlassCard class="p-5 flex flex-col gap-4 hover-tilt group">
        <div class="flex items-center gap-3 text-white mb-1">
          <div class="w-8 h-8 rounded-lg bg-pink-400/20 flex items-center justify-center text-pink-400 group-hover:scale-110 transition-transform">
            <span class="material-symbols-rounded">content_paste</span>
          </div>
          <span class="text-sm font-bold tracking-wide">Clipboard</span>
        </div>
        <div class="flex gap-2">
          <button @click="syncClipboard('get')" class="flex-1 p-2.5 bg-white/5 hover:bg-white/10 rounded-lg border border-white/10 text-[10px] font-bold uppercase tracking-wider text-white transition-all active:scale-95">
            Phone &rarr; PC
          </button>
          <button @click="syncClipboard('set')" class="flex-1 p-2.5 bg-white/5 hover:bg-white/10 rounded-lg border border-white/10 text-[10px] font-bold uppercase tracking-wider text-white transition-all active:scale-95">
            PC &rarr; Phone
          </button>
        </div>
      </GlassCard>

      <!-- 7. Quick Shell -->
      <GlassCard class="p-5 flex flex-col gap-4 col-span-1 md:col-span-2 hover-tilt group">
        <div class="flex items-center gap-3 text-white mb-1">
          <div class="w-8 h-8 rounded-lg bg-green-400/20 flex items-center justify-center text-green-400 group-hover:scale-110 transition-transform">
            <span class="material-symbols-rounded">terminal</span>
          </div>
          <span class="text-sm font-bold tracking-wide">Quick Shell</span>
        </div>
        <div class="flex gap-2">
          <div class="flex-1 relative group/input">
            <span class="absolute left-3 top-1/2 -translate-y-1/2 text-green-400/50 text-xs font-mono font-bold">$</span>
            <input 
              v-model="shellCmd" 
              @keyup.enter="runShell"
              type="text" 
              class="w-full bg-black/30 border border-white/10 rounded-lg text-xs text-white py-2.5 pl-7 pr-3 font-mono outline-none focus:border-green-400/50 transition-colors placeholder-white/20" 
              placeholder="adb shell command..."
            >
          </div>
          <button @click="runShell" class="px-5 bg-green-500/20 hover:bg-green-500/30 text-green-400 text-xs font-bold rounded-lg border border-green-500/20 transition-all active:scale-95">
            RUN
          </button>
        </div>
      </GlassCard>

      <!-- 8. Battery Limiter (Root) -->
      <GlassCard class="p-5 flex flex-col gap-4 hover-tilt group">
        <div class="flex items-center justify-between text-white mb-1">
          <div class="flex items-center gap-3">
            <div class="w-8 h-8 rounded-lg bg-yellow-400/20 flex items-center justify-center text-yellow-400 group-hover:scale-110 transition-transform">
              <span class="material-symbols-rounded">battery_charging_full</span>
            </div>
            <span class="text-sm font-bold tracking-wide">Charge Limit</span>
          </div>
          <span class="text-xs font-mono font-bold text-yellow-400 bg-yellow-400/10 px-2 py-1 rounded">{{ chargeLimit }}%</span>
        </div>
        <div class="relative h-6 flex items-center">
            <input 
            v-model="chargeLimit" 
            type="range" 
            min="50" 
            max="100" 
            step="5"
            class="w-full h-1.5 bg-white/10 rounded-lg appearance-none cursor-pointer [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-4 [&::-webkit-slider-thumb]:h-4 [&::-webkit-slider-thumb]:bg-yellow-400 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:shadow-[0_0_10px_rgba(250,204,21,0.5)] [&::-webkit-slider-thumb]:transition-transform [&::-webkit-slider-thumb]:hover:scale-125"
            >
        </div>
      </GlassCard>

      <!-- 9. File Push (Quick) -->
      <GlassCard class="p-5 flex flex-col gap-4 hover-tilt group">
        <div class="flex items-center gap-3 text-white mb-1">
          <div class="w-8 h-8 rounded-lg bg-cyan-400/20 flex items-center justify-center text-cyan-400 group-hover:scale-110 transition-transform">
            <span class="material-symbols-rounded">upload_file</span>
          </div>
          <span class="text-sm font-bold tracking-wide">Quick Push</span>
        </div>
        <div class="border-2 border-dashed border-white/10 rounded-xl bg-white/5 h-20 flex flex-col items-center justify-center text-center cursor-pointer hover:bg-white/10 hover:border-cyan-400/30 transition-all group/drop">
          <span class="material-symbols-rounded text-white/30 text-2xl mb-1 group-hover/drop:text-cyan-400 group-hover/drop:scale-110 transition-all">drag_indicator</span>
          <span class="text-[10px] text-white/40 font-medium uppercase tracking-wider">Drop files to /sdcard/</span>
        </div>
      </GlassCard>

      <!-- 10. Package Manager (Mini) -->
      <GlassCard class="p-5 flex flex-col gap-4 hover-tilt group">
        <div class="flex items-center gap-3 text-white mb-1">
          <div class="w-8 h-8 rounded-lg bg-red-400/20 flex items-center justify-center text-red-400 group-hover:scale-110 transition-transform">
            <span class="material-symbols-rounded">apps</span>
          </div>
          <span class="text-sm font-bold tracking-wide">App Actions</span>
        </div>
        <div class="flex gap-2">
          <input v-model="pkgName" type="text" class="bg-white/5 border border-white/10 rounded-lg text-xs text-white p-2.5 w-full outline-none focus:border-red-400/50 transition-colors placeholder-white/20" placeholder="com.example.app">
        </div>
        <div class="flex gap-1">
          <button class="flex-1 py-1.5 bg-white/5 hover:bg-red-500/20 hover:text-red-400 rounded text-[10px] font-bold text-white/70 border border-white/5 transition-all">Uninstall</button>
          <button class="flex-1 py-1.5 bg-white/5 hover:bg-white/10 rounded text-[10px] font-bold text-white/70 border border-white/5 transition-all">Disable</button>
          <button class="flex-1 py-1.5 bg-white/5 hover:bg-white/10 rounded text-[10px] font-bold text-white/70 border border-white/5 transition-all">Clear</button>
        </div>
      </GlassCard>

  </div>
</template>
