<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';
import ToggleSwitch from '../../ui/ToggleSwitch.vue';
import GlassInput from '../../ui/GlassInput.vue';

interface ZeroTouchState {
  is_enabled: boolean;
  target_device_serial: string;
  profile_path: string;
  auto_start_delay: number;
}

const state = ref<ZeroTouchState>({
  is_enabled: false,
  target_device_serial: '',
  profile_path: '',
  auto_start_delay: 5
});

const isLoading = ref(false);
const countdown = ref<number | null>(null);
const detectedDevice = ref('');
const unlistenFns = ref<Function[]>([]);

async function loadState() {
  try {
    const rawState = await invoke<any>('get_zero_touch_state');
    state.value = {
      is_enabled: rawState.is_enabled,
      target_device_serial: rawState.target_device_serial || '',
      profile_path: rawState.profile_path || '',
      auto_start_delay: rawState.auto_start_delay
    };
    
    // Start the background service
    await invoke('start_zero_touch_service');
  } catch (e) {
    console.error(e);
  }
}

async function saveState() {
  isLoading.value = true;
  try {
    await invoke('set_zero_touch_state', { state: state.value });
  } catch (e) {
    console.error(e);
  } finally {
    isLoading.value = false;
  }
}

function cancelCountdown() {
  invoke('cancel_zero_touch');
  countdown.value = null;
}

onMounted(async () => {
  await loadState();
  
  unlistenFns.value.push(await listen<string>('zero-touch-detected', (e) => {
    detectedDevice.value = e.payload;
  }));
  
  unlistenFns.value.push(await listen<number>('zero-touch-countdown', (e) => {
    countdown.value = e.payload;
  }));
  
  unlistenFns.value.push(await listen('zero-touch-started', () => {
    countdown.value = null;
    // Maybe redirect to flash view or show progress
  }));
  
  unlistenFns.value.push(await listen('zero-touch-cancelled', () => {
    countdown.value = null;
  }));
});

onUnmounted(() => {
  unlistenFns.value.forEach(fn => fn());
});
</script>

<template>
  <GlassCard class="relative overflow-hidden">
    <!-- Countdown Overlay -->
    <div v-if="countdown !== null" class="absolute inset-0 z-50 bg-surface/90 backdrop-blur-xl flex flex-col items-center justify-center animate-in fade-in">
      <div class="text-6xl font-bold text-primary mb-4 animate-pulse">{{ countdown }}</div>
      <h3 class="text-xl font-medium text-white mb-2">Auto-Flash Initiated</h3>
      <p class="text-white/60 mb-8">Device: {{ detectedDevice }}</p>
      <VisionButton variant="danger" @click="cancelCountdown" icon="close">CANCEL NOW</VisionButton>
    </div>

    <div class="flex items-center gap-3 mb-6">
      <div class="p-3 rounded-xl bg-primary/20 text-primary">
        <span class="material-symbols-rounded text-2xl">bolt</span>
      </div>
      <div>
        <h2 class="text-xl font-bold text-white">Zero-Touch Flash</h2>
        <p class="text-sm text-white/60">Auto-flash devices upon connection.</p>
      </div>
    </div>

    <div class="space-y-6">
      <div class="flex items-center justify-between p-4 rounded-xl bg-white/5 border border-white/10">
        <div>
          <div class="font-medium text-white">Enable Zero-Touch</div>
          <div class="text-xs text-white/60">Automatically detect and flash devices</div>
        </div>
        <ToggleSwitch v-model="state.is_enabled" @change="saveState" />
      </div>

      <div v-if="state.is_enabled" class="space-y-4 animate-in fade-in slide-in-from-top-2">
        <div>
          <label class="text-sm text-white/60 mb-2 block">Target Device Serial (Optional)</label>
          <GlassInput v-model="state.target_device_serial" placeholder="Any Device" @change="saveState" />
        </div>

        <div>
          <label class="text-sm text-white/60 mb-2 block">Flash Profile Path</label>
          <div class="flex gap-2">
            <GlassInput v-model="state.profile_path" placeholder="/path/to/config.yaml" class="flex-1" @change="saveState" />
            <VisionButton icon="folder_open" variant="secondary">Browse</VisionButton>
          </div>
        </div>

        <div>
          <label class="text-sm text-white/60 mb-2 block">Auto-Start Delay (Seconds)</label>
          <input 
            type="range" 
            v-model.number="state.auto_start_delay" 
            min="0" 
            max="30" 
            class="w-full accent-primary"
            @change="saveState"
          />
          <div class="text-right text-xs text-white/60">{{ state.auto_start_delay }}s</div>
        </div>
      </div>
    </div>
  </GlassCard>
</template>
