<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';
import GlassInput from '../../ui/GlassInput.vue';

const deviceInfo = ref('');
const requiredVersion = ref('');
const checkResult = ref<boolean | null>(null);
const loading = ref(false);

const fetchInfo = async () => {
  try {
    deviceInfo.value = await invoke('get_device_firmware_info');
  } catch (e) {
    deviceInfo.value = 'Device not connected or error fetching info.';
  }
};

const checkCompliance = async () => {
  if (!requiredVersion.value) return;
  loading.value = true;
  try {
    checkResult.value = await invoke('check_firmware_compliance', { requiredVersion: requiredVersion.value });
  } catch (e) {
    console.error(e);
  } finally {
    loading.value = false;
  }
};

onMounted(fetchInfo);
</script>

<template>
  <GlassCard>
    <div class="flex items-center gap-3 mb-6">
      <div class="p-3 rounded-xl bg-success/20 text-success">
        <span class="material-symbols-rounded text-2xl">verified</span>
      </div>
      <div>
        <h2 class="text-xl font-bold text-white">Firmware Matcher</h2>
        <p class="text-sm text-white/60">Validate device requirements.</p>
      </div>
    </div>

    <div class="space-y-4">
      <div class="bg-black/20 p-3 rounded-lg border border-white/5">
        <h3 class="text-xs font-bold text-white/60 mb-2">Current Device Info</h3>
        <pre class="text-xs font-mono text-white/80 whitespace-pre-wrap">{{ deviceInfo }}</pre>
        <button @click="fetchInfo" class="mt-2 text-xs text-primary hover:underline">Refresh</button>
      </div>

      <div>
        <label class="text-xs text-white/60 mb-2 block">Required Version String</label>
        <div class="flex gap-2">
          <GlassInput v-model="requiredVersion" placeholder="e.g. 11.0.5.1" />
          <VisionButton @click="checkCompliance" :loading="loading" icon="check_circle">Check</VisionButton>
        </div>
      </div>

      <div v-if="checkResult !== null" class="p-3 rounded-lg text-center font-bold text-sm" :class="checkResult ? 'bg-success/20 text-success' : 'bg-error/20 text-error'">
        {{ checkResult ? 'MATCH FOUND' : 'NO MATCH' }}
      </div>
    </div>
  </GlassCard>
</template>
