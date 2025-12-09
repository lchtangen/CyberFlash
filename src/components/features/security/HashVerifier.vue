<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';
import GlassInput from '../../ui/GlassInput.vue';

const filePath = ref('');
const expectedHash = ref('');
const algorithm = ref('sha256');
const calculatedHash = ref('');
const isMatch = ref<boolean | null>(null);
const isProcessing = ref(false);

const selectFile = async () => {
  try {
    const file = await open({
      multiple: false
    });
    if (file) {
      filePath.value = file as string;
      calculatedHash.value = '';
      isMatch.value = null;
    }
  } catch (e) {
    console.error(e);
  }
};

const calculate = async () => {
  if (!filePath.value) return;
  isProcessing.value = true;
  try {
    calculatedHash.value = await invoke('calculate_file_hash', { 
      filePath: filePath.value, 
      algorithm: algorithm.value 
    });
    
    if (expectedHash.value) {
      isMatch.value = calculatedHash.value.toLowerCase() === expectedHash.value.toLowerCase();
    }
  } catch (e) {
    console.error(e);
  } finally {
    isProcessing.value = false;
  }
};

const verify = async () => {
  if (!filePath.value || !expectedHash.value) return;
  isProcessing.value = true;
  try {
    isMatch.value = await invoke('verify_file_hash', { 
      filePath: filePath.value, 
      expectedHash: expectedHash.value,
      algorithm: algorithm.value
    });
    
    if (!calculatedHash.value) {
      // Also calculate to show user
      calculatedHash.value = await invoke('calculate_file_hash', { 
        filePath: filePath.value, 
        algorithm: algorithm.value 
      });
    }
  } catch (e) {
    console.error(e);
  } finally {
    isProcessing.value = false;
  }
};
</script>

<template>
  <GlassCard>
    <div class="flex items-center gap-3 mb-6">
      <div class="p-3 rounded-xl bg-primary/20 text-primary">
        <span class="material-symbols-rounded text-2xl">fingerprint</span>
      </div>
      <div>
        <h2 class="text-xl font-bold text-white">Hash Verifier</h2>
        <p class="text-sm text-white/60">Verify file integrity (MD5/SHA256).</p>
      </div>
    </div>

    <div class="space-y-4">
      <div 
        @click="selectFile"
        class="border border-dashed border-white/20 rounded-lg p-4 text-center cursor-pointer hover:bg-white/5 transition-colors"
      >
        <div v-if="!filePath" class="text-text-muted text-sm">
          Click to select file
        </div>
        <div v-else class="text-white text-xs font-mono break-all">
          {{ filePath }}
        </div>
      </div>

      <div class="flex gap-4">
        <div class="w-1/3">
          <label class="text-xs text-white/60 mb-2 block">Algorithm</label>
          <select v-model="algorithm" class="w-full bg-black/20 border border-white/10 rounded-lg p-2 text-white text-sm">
            <option value="sha256">SHA-256</option>
            <option value="md5">MD5</option>
          </select>
        </div>
        <div class="flex-1">
          <label class="text-xs text-white/60 mb-2 block">Expected Hash (Optional)</label>
          <GlassInput v-model="expectedHash" placeholder="Paste hash here..." />
        </div>
      </div>

      <div class="flex gap-2">
        <VisionButton @click="calculate" :loading="isProcessing" icon="calculate" class="flex-1">
          Calculate
        </VisionButton>
        <VisionButton @click="verify" :loading="isProcessing" icon="verified_user" variant="secondary" :disabled="!expectedHash" class="flex-1">
          Verify
        </VisionButton>
      </div>

      <div v-if="calculatedHash" class="bg-black/20 p-3 rounded-lg border border-white/5">
        <h3 class="text-xs font-bold text-white/60 mb-1">Calculated Hash</h3>
        <p class="text-xs font-mono text-white/80 break-all select-all">{{ calculatedHash }}</p>
      </div>

      <div v-if="isMatch !== null" class="p-3 rounded-lg text-center font-bold text-sm" :class="isMatch ? 'bg-success/20 text-success' : 'bg-error/20 text-error'">
        {{ isMatch ? 'HASH MATCHED' : 'HASH MISMATCH' }}
      </div>
    </div>
  </GlassCard>
</template>
