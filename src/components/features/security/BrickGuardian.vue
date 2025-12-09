<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import GlassCard from '../../ui/GlassCard.vue';

const props = defineProps<{
  deviceModel: string;
  currentFirmware: string;
  targetRom: string;
  targetAndroidVersion: string;
}>();

interface BrickRisk {
  level: 'Safe' | 'Warning' | 'Critical';
  message: string;
  details?: string;
}

const risk = ref<BrickRisk | null>(null);
const isLoading = ref(false);

async function checkRisk() {
  if (!props.deviceModel || !props.targetRom) return;
  
  isLoading.value = true;
  try {
    risk.value = await invoke<BrickRisk>('check_brick_risk', {
      context: {
        device_model: props.deviceModel,
        current_firmware: props.currentFirmware,
        target_rom: props.targetRom,
        target_android_version: props.targetAndroidVersion
      }
    });
  } catch (e) {
    console.error("Failed to check brick risk:", e);
  } finally {
    isLoading.value = false;
  }
}

watch(() => props, checkRisk, { deep: true });
onMounted(checkRisk);
</script>

<template>
  <GlassCard v-if="risk" class="transition-all duration-300" :class="{
    'border-success/50 bg-success/5': risk.level === 'Safe',
    'border-warning/50 bg-warning/5': risk.level === 'Warning',
    'border-error/50 bg-error/10': risk.level === 'Critical'
  }">
    <div class="flex items-start gap-4">
      <div class="p-3 rounded-full" :class="{
        'bg-success/20 text-success': risk.level === 'Safe',
        'bg-warning/20 text-warning': risk.level === 'Warning',
        'bg-error/20 text-error': risk.level === 'Critical'
      }">
        <span class="material-symbols-rounded text-2xl">
          {{ risk.level === 'Safe' ? 'verified_user' : risk.level === 'Warning' ? 'warning' : 'dangerous' }}
        </span>
      </div>
      
      <div class="flex-1">
        <h3 class="text-lg font-bold" :class="{
          'text-success': risk.level === 'Safe',
          'text-warning': risk.level === 'Warning',
          'text-error': risk.level === 'Critical'
        }">
          {{ risk.level === 'Safe' ? 'Brick Protection Active' : risk.message }}
        </h3>
        
        <p class="text-sm text-white/80 mt-1" v-if="risk.details">
          {{ risk.details }}
        </p>
        <p class="text-sm text-white/60 mt-1" v-else>
          System analysis complete. No known brick vectors detected for this configuration.
        </p>
      </div>
    </div>
  </GlassCard>
</template>
