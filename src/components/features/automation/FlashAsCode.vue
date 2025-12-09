<template>
  <div class="relative overflow-hidden rounded-2xl border border-white/10 bg-surface/30 backdrop-blur-xl transition-all duration-300 hover:bg-surface/40 p-6">
    <div class="flex items-center justify-between mb-4">
      <div>
        <h3 class="text-lg font-semibold text-white flex items-center gap-2">
          <span class="i-lucide-file-code text-primary"></span>
          Flash-as-Code
        </h3>
        <p class="text-sm text-white/60">Define your flash workflow in YAML</p>
      </div>
      <div class="flex gap-2">
        <button 
          @click="loadExample"
          class="px-3 py-1.5 rounded-lg bg-white/5 hover:bg-white/10 text-xs font-medium text-white/80 transition-colors"
        >
          Load Example
        </button>
        <button 
          @click="executePlan"
          :disabled="!isValid || isExecuting"
          class="px-4 py-1.5 rounded-lg bg-primary hover:bg-primary-hover disabled:opacity-50 disabled:cursor-not-allowed text-sm font-bold text-white transition-all shadow-lg shadow-primary/20"
        >
          <span v-if="isExecuting" class="i-lucide-loader-2 animate-spin mr-2"></span>
          {{ isExecuting ? 'Executing...' : 'Run Plan' }}
        </button>
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- Editor -->
      <div class="relative group">
        <div class="absolute inset-0 bg-gradient-to-br from-primary/5 to-transparent rounded-xl pointer-events-none"></div>
        <textarea
          v-model="yamlContent"
          class="w-full h-64 bg-black/40 border border-white/10 rounded-xl p-4 font-mono text-sm text-white/90 focus:outline-none focus:border-primary/50 resize-none custom-scrollbar"
          placeholder="# Paste your flash_config.yaml here..."
          @input="validateConfig"
        ></textarea>
        <div v-if="validationError" class="absolute bottom-4 left-4 right-4 bg-error/10 border border-error/20 text-error text-xs p-2 rounded-lg backdrop-blur-md">
          {{ validationError }}
        </div>
        <div v-else-if="isValid" class="absolute bottom-4 right-4 bg-success/10 border border-success/20 text-success text-xs px-2 py-1 rounded-lg backdrop-blur-md flex items-center gap-1">
          <span class="i-lucide-check-circle w-3 h-3"></span> Valid Config
        </div>
      </div>

      <!-- Execution Plan / Status -->
      <div class="bg-black/20 border border-white/5 rounded-xl p-4 h-64 overflow-y-auto custom-scrollbar">
        <h4 class="text-xs font-bold text-white/50 uppercase tracking-wider mb-3">Execution Plan</h4>
        
        <div v-if="!parsedConfig" class="h-full flex flex-col items-center justify-center text-white/20">
          <span class="i-lucide-workflow w-8 h-8 mb-2"></span>
          <span class="text-sm">No plan loaded</span>
        </div>

        <div v-else class="space-y-2">
          <div 
            v-for="(step, index) in parsedConfig.steps" 
            :key="index"
            class="flex items-center gap-3 p-2 rounded-lg transition-colors"
            :class="{
              'bg-white/5': currentStepIndex !== index,
              'bg-primary/10 border border-primary/20': currentStepIndex === index,
              'opacity-50': currentStepIndex > index
            }"
          >
            <div class="w-6 h-6 rounded-full flex items-center justify-center text-xs font-bold shrink-0"
              :class="{
                'bg-white/10 text-white/60': currentStepIndex < index,
                'bg-primary text-white animate-pulse': currentStepIndex === index,
                'bg-success text-white': currentStepIndex > index
              }"
            >
              <span v-if="currentStepIndex > index" class="i-lucide-check"></span>
              <span v-else>{{ index + 1 }}</span>
            </div>
            
            <div class="flex-1 min-w-0">
              <div class="flex items-center justify-between">
                <span class="text-sm font-medium text-white truncate">{{ formatStepType(step.type) }}</span>
                <span class="text-[10px] font-mono text-white/40">{{ getStepDetail(step) }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

const yamlContent = ref('');
const validationError = ref<string | null>(null);
const isValid = ref(false);
const isExecuting = ref(false);
const parsedConfig = ref<any>(null);
const currentStepIndex = ref(-1);

const EXAMPLE_CONFIG = `name: "Clean Install LineageOS"
device: "oneplus7pro"
version: "1.0"
steps:
  - type: wipe
    params:
      partitions: ["userdata", "cache"]
  - type: flash_rom
    params:
      file: "lineage-21.0-20240101-nightly-guacamole-signed.zip"
      slot: "a"
  - type: reboot
    params:
      mode: "recovery"
  - type: sideload
    params:
      file: "MindTheGapps-14.0.0-arm64.zip"
  - type: reboot
    params:
      mode: "system"`;

function loadExample() {
  yamlContent.value = EXAMPLE_CONFIG;
  validateConfig();
}

async function validateConfig() {
  if (!yamlContent.value.trim()) {
    isValid.value = false;
    validationError.value = null;
    parsedConfig.value = null;
    return;
  }

  try {
    const config = await invoke('validate_flash_config', { yamlContent: yamlContent.value });
    parsedConfig.value = config;
    isValid.value = true;
    validationError.value = null;
  } catch (e: any) {
    isValid.value = false;
    validationError.value = e.toString();
    parsedConfig.value = null;
  }
}

function formatStepType(type: string) {
  return type.split('_').map(word => word.charAt(0).toUpperCase() + word.slice(1)).join(' ');
}

function getStepDetail(step: any) {
  if (step.type === 'wipe') return `[${step.params.partitions.join(', ')}]`;
  if (step.type === 'flash_rom') return step.params.file;
  if (step.type === 'reboot') return `-> ${step.params.mode}`;
  if (step.type === 'sideload') return step.params.file;
  return '';
}

async function executePlan() {
  if (!isValid.value || !parsedConfig.value) return;
  
  isExecuting.value = true;
  currentStepIndex.value = 0;

  // Listen for progress updates
  const unlistenUpdate = await listen('flash-plan-update', (event: any) => {
    const payload = event.payload;
    if (payload.status === 'running') {
      currentStepIndex.value = payload.step_index;
    } else if (payload.status === 'success') {
      currentStepIndex.value = payload.step_index + 1;
    }
  });

  const unlistenComplete = await listen('flash-plan-complete', () => {
    isExecuting.value = false;
    currentStepIndex.value = parsedConfig.value.steps.length; // All done
    unlistenUpdate();
    unlistenComplete();
  });

  try {
    await invoke('execute_flash_plan', { config: parsedConfig.value });
  } catch (e) {
    console.error(e);
    isExecuting.value = false;
    unlistenUpdate();
    unlistenComplete();
  }
}
</script>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: rgba(255, 255, 255, 0.05);
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.2);
  border-radius: 2px;
}
</style>
