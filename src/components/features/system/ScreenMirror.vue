<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '@/components/ui/GlassCard.vue';
import VisionButton from '@/components/ui/VisionButton.vue';
import { useDeviceStore } from '@/stores/device';

const deviceStore = useDeviceStore();
const isInstalled = ref(false);
const isRunning = ref(false);
const isTakingScreenshot = ref(false);
const bitrate = ref(8); // Mbps
const maxFps = ref(60);
const record = ref(false);
const audio = ref(true);
const errorMsg = ref('');
const successMsg = ref('');
const showWireless = ref(false);
const wirelessIp = ref('');
const isConnecting = ref(false);

onMounted(async () => {
  isInstalled.value = await invoke('check_scrcpy_installed');
});

const toggleWireless = async () => {
  if (!deviceStore.serial) {
    errorMsg.value = "No device connected. Connect via USB first to enable wireless.";
    return;
  }
  
  isConnecting.value = true;
  try {
    // 1. Get IP
    const ip = await invoke('get_ip_address', { serial: deviceStore.serial });
    if (!ip) throw new Error("Could not get device IP");
    wirelessIp.value = ip as string;
    
    // 2. Enable TCP/IP
    await invoke('enable_wireless_debugging');
    
    // 3. Connect
    await invoke('connect_wireless', { ip: wirelessIp.value });
    
    successMsg.value = `Connected wirelessly to ${wirelessIp.value}`;
    showWireless.value = true;
  } catch (e) {
    errorMsg.value = String(e);
  } finally {
    isConnecting.value = false;
  }
};

const takeScreenshot = async () => {
  if (!deviceStore.serial) {
    errorMsg.value = "No device selected";
    return;
  }
  
  isTakingScreenshot.value = true;
  errorMsg.value = '';
  successMsg.value = '';
  
  try {
    const path = await invoke('take_screenshot', { serial: deviceStore.serial });
    successMsg.value = `Saved: ${path}`;
    // Clear success message after 3 seconds
    setTimeout(() => { successMsg.value = ''; }, 3000);
  } catch (e) {
    errorMsg.value = String(e);
  } finally {
    isTakingScreenshot.value = false;
  }
};

const startMirror = async () => {
  if (!deviceStore.serial) {
    errorMsg.value = "No device selected";
    return;
  }
  
  isRunning.value = true;
  errorMsg.value = '';
  successMsg.value = '';
  
  try {
    await invoke('start_scrcpy', {
      serial: deviceStore.serial,
      bitrate: bitrate.value,
      maxFps: maxFps.value,
      record: record.value,
      audio: audio.value
    });
  } catch (e) {
    errorMsg.value = String(e);
  } finally {
    isRunning.value = false;
  }
};
</script>

<template>
  <GlassCard class="relative overflow-hidden group">
    <div class="absolute inset-0 bg-gradient-to-br from-purple-500/10 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
    
    <div class="relative z-10 flex flex-col h-full">
      <div class="flex items-center justify-between mb-4">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-xl bg-purple-500/20 flex items-center justify-center text-purple-400">
            <span class="i-lucide-monitor-smartphone w-6 h-6"></span>
          </div>
          <div>
            <h3 class="text-lg font-bold text-white">Screen Mirror</h3>
            <p class="text-xs text-white/50">Powered by Scrcpy</p>
          </div>
        </div>
        <div v-if="isInstalled" class="px-2 py-1 rounded-md bg-success/10 text-success text-[10px] font-bold uppercase tracking-wider border border-success/20">
          Ready
        </div>
                <div v-else class="px-2 py-1 rounded-md bg-error/10 text-error text-[10px] font-bold uppercase tracking-wider border border-error/20">
          Missing
        </div>
      </div>

      <div v-if="!isInstalled" class="flex-1 flex flex-col items-center justify-center text-center p-4">
        <p class="text-sm text-white/70 mb-4">
          Scrcpy is not detected on your system. Please install it to use screen mirroring.
        </p>
        <div class="bg-black/30 rounded-lg p-3 w-full text-left">
          <p class="text-xs text-white/40 mb-1">Ubuntu / Debian:</p>
          <code class="text-xs text-primary font-mono">sudo apt install scrcpy</code>
        </div>
      </div>

      <div v-else class="flex-1 flex flex-col gap-4">
        <!-- Controls -->
        <div class="grid grid-cols-2 gap-4">
          <div class="space-y-2">
            <label class="text-xs text-white/50 uppercase font-bold">Bitrate (Mbps)</label>
            <input v-model="bitrate" type="number" min="1" max="50" class="w-full bg-black/20 border border-white/10 rounded-lg px-3 py-2 text-sm text-white focus:border-primary focus:outline-none transition-colors" />
          </div>
          <div class="space-y-2">
            <label class="text-xs text-white/50 uppercase font-bold">Max FPS</label>
            <input v-model="maxFps" type="number" min="30" max="120" class="w-full bg-black/20 border border-white/10 rounded-lg px-3 py-2 text-sm text-white focus:border-primary focus:outline-none transition-colors" />
          </div>
        </div>

        <div class="flex items-center gap-4">
          <label class="flex items-center gap-2 cursor-pointer">
            <input v-model="record" type="checkbox" class="form-checkbox rounded bg-white/10 border-white/20 text-primary focus:ring-0" />
            <span class="text-sm text-white/80">Record Screen</span>
          </label>
          <label class="flex items-center gap-2 cursor-pointer">
            <input v-model="audio" type="checkbox" class="form-checkbox rounded bg-white/10 border-white/20 text-primary focus:ring-0" />
            <span class="text-sm text-white/80">Forward Audio</span>
          </label>
        </div>

        <!-- Actions -->
        <div class="grid grid-cols-2 gap-3 mt-auto">
          <VisionButton @click="startMirror" :loading="isRunning" variant="primary" class="w-full">
            <span class="material-symbols-rounded mr-2">play_arrow</span>
            Start Mirror
          </VisionButton>
          
          <VisionButton @click="takeScreenshot" :loading="isTakingScreenshot" variant="secondary" class="w-full">
            <span class="material-symbols-rounded mr-2">camera_alt</span>
            Screenshot
          </VisionButton>
        </div>
        
        <VisionButton @click="toggleWireless" :loading="isConnecting" variant="ghost" class="w-full text-xs">
          <span class="material-symbols-rounded mr-2">wifi</span>
          {{ showWireless ? 'Wireless Active' : 'Enable Wireless Mode' }}
        </VisionButton>

        <!-- Status Messages -->
        <div v-if="errorMsg" class="p-3 rounded-lg bg-error/10 border border-error/20 text-error text-xs">
          {{ errorMsg }}
        </div>
        <div v-if="successMsg" class="p-3 rounded-lg bg-success/10 border border-success/20 text-success text-xs">
          {{ successMsg }}
        </div>
      </div>
    </div>
  </GlassCard>
</template>
