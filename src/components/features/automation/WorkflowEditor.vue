<script setup lang="ts">
import { ref, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import GlassCard from '@/components/ui/GlassCard.vue';
import VisionButton from '@/components/ui/VisionButton.vue';

interface FlashStep {
  type: string;
  params: Record<string, any>;
}

const steps = ref<FlashStep[]>([]);
const isExecuting = ref(false);
const isRecording = ref(false);
const continueOnError = ref(false);
const executionLogs = ref<any[]>([]);
let unlistenMacro: (() => void) | null = null;

const availableStepTypes = [
  { label: 'Reboot Device', type: 'reboot', defaultParams: { mode: 'system' } },
  { label: 'Wipe Partition', type: 'wipe', defaultParams: { partitions: ['userdata'] } },
  { label: 'Flash Image', type: 'flash_image', defaultParams: { partition: 'boot', file: '', slot: null } },
  { label: 'Flash Zip (Update)', type: 'flash_zip', defaultParams: { file: '' } },
  { label: 'Flash Recovery', type: 'flash_recovery', defaultParams: { file: '' } },
  { label: 'Sideload Zip', type: 'sideload', defaultParams: { file: '' } },
  { label: 'Wait', type: 'wait', defaultParams: { seconds: 5 } },
];

const toggleRecording = async () => {
  isRecording.value = !isRecording.value;
  if (isRecording.value) {
    unlistenMacro = await listen('macro-event', (event: any) => {
      const payload = event.payload;
      steps.value.push({
        type: payload.step_type,
        params: payload.params
      });
    });
  } else {
    if (unlistenMacro) {
      unlistenMacro();
      unlistenMacro = null;
    }
  }
};

onUnmounted(() => {
  if (unlistenMacro) unlistenMacro();
});

const addStep = (typeDef: any) => {
  steps.value.push({
    type: typeDef.type,
    params: JSON.parse(JSON.stringify(typeDef.defaultParams)), // Deep copy
  });
};

const removeStep = (index: number) => {
  steps.value.splice(index, 1);
};

const moveStep = (index: number, direction: 'up' | 'down') => {
  if (direction === 'up' && index > 0) {
    const temp = steps.value[index];
    steps.value[index] = steps.value[index - 1];
    steps.value[index - 1] = temp;
  } else if (direction === 'down' && index < steps.value.length - 1) {
    const temp = steps.value[index];
    steps.value[index] = steps.value[index + 1];
    steps.value[index + 1] = temp;
  }
};

const executeWorkflow = async () => {
  if (steps.value.length === 0) return;
  
  isExecuting.value = true;
  executionLogs.value = [];
  
  const config = {
    name: "Visual Workflow",
    device: "auto",
    version: "1.0",
    continue_on_error: continueOnError.value,
    steps: steps.value.map(s => {
      // Transform to backend expected format
      // The backend expects enum variants which serde deserializes from { type: "...", ...params }
      // But our backend enum is #[serde(tag = "type", content = "params")]
      // So we need to structure it as { type: "wipe", params: { ... } }
      // Wait, let's check the Rust struct again.
      return {
        type: s.type,
        params: s.params
      };
    })
  };

  try {
    // We need to listen for events, but for now let's just fire it
    await invoke('execute_flash_plan', { config });
  } catch (error) {
    console.error("Execution failed:", error);
    executionLogs.value.push({ status: 'error', message: String(error) });
  } finally {
    isExecuting.value = false;
  }
};

const exportWorkflow = () => {
  const config = {
    name: "Exported Workflow",
    device: "auto",
    version: "1.0",
    continue_on_error: continueOnError.value,
    steps: steps.value.map(s => ({ type: s.type, params: s.params }))
  };
  
  const blob = new Blob([JSON.stringify(config, null, 2)], { type: 'application/json' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = `workflow_${new Date().toISOString().slice(0,10)}.cf-flow`;
  a.click();
  URL.revokeObjectURL(url);
};

const importWorkflow = (event: Event) => {
  const file = (event.target as HTMLInputElement).files?.[0];
  if (!file) return;
  
  const reader = new FileReader();
  reader.onload = (e) => {
    try {
      const config = JSON.parse(e.target?.result as string);
      if (config.steps && Array.isArray(config.steps)) {
        steps.value = config.steps;
        continueOnError.value = config.continue_on_error || false;
      }
    } catch (err) {
      console.error("Failed to parse workflow", err);
    }
  };
  reader.readAsText(file);
};

const loadTemplate = (templateName: string) => {
  if (templateName === 'debloat') {
    steps.value = [
      { type: 'wait', params: { seconds: 2 } },
      { type: 'sideload', params: { file: '/path/to/debloat_script.zip' } }, // Placeholder
      { type: 'reboot', params: { mode: 'system' } }
    ];
  } else if (templateName === 'backup') {
    steps.value = [
      { type: 'wait', params: { seconds: 1 } },
      // Backup logic usually involves adb backup which is not a step type yet, maybe shell command?
      // For now, just a placeholder wait
      { type: 'wait', params: { seconds: 5 } }
    ];
  }
};

</script>

<template>
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 h-full">
    <!-- Toolbox -->
    <GlassCard class="lg:col-span-1 flex flex-col gap-4">
      <h3 class="text-lg font-semibold text-white flex items-center gap-2">
        <span class="i-lucide-box text-primary"></span>
        Toolbox
      </h3>
      
      <!-- Templates -->
      <div class="p-3 rounded-xl bg-white/5 border border-white/5">
        <div class="text-xs font-bold text-white/50 uppercase mb-2">Templates</div>
        <div class="flex gap-2">
          <button @click="loadTemplate('debloat')" class="px-3 py-1.5 rounded-lg bg-primary/10 hover:bg-primary/20 text-primary text-xs font-medium transition-colors">
            Debloat
          </button>
          <button @click="loadTemplate('backup')" class="px-3 py-1.5 rounded-lg bg-primary/10 hover:bg-primary/20 text-primary text-xs font-medium transition-colors">
            Backup
          </button>
        </div>
      </div>

      <div class="grid grid-cols-1 gap-2 overflow-y-auto custom-scrollbar pr-2">
        <button
          v-for="item in availableStepTypes"
          :key="item.type"
          @click="addStep(item)"
          class="flex items-center gap-3 p-3 rounded-xl bg-white/5 hover:bg-white/10 border border-white/5 hover:border-primary/30 transition-all text-left group"
        >
          <div class="w-8 h-8 rounded-lg bg-surface flex items-center justify-center text-primary group-hover:scale-110 transition-transform">
            <span class="i-lucide-plus"></span>
          </div>
          <div>
            <div class="font-medium text-white">{{ item.label }}</div>
            <div class="text-xs text-white/50">{{ item.type }}</div>
          </div>
        </button>
      </div>
    </GlassCard>

    <!-- Canvas / List -->
    <GlassCard class="lg:col-span-2 flex flex-col h-full">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-lg font-semibold text-white flex items-center gap-2">
          <span class="i-lucide-workflow text-primary"></span>
          Workflow Canvas
        </h3>
        <div class="flex gap-2">
          <label class="flex items-center gap-2 px-3 py-1.5 rounded-lg bg-white/5 hover:bg-white/10 cursor-pointer select-none transition-colors">
            <input type="checkbox" v-model="continueOnError" class="w-4 h-4 rounded border-white/20 bg-black/50 text-primary focus:ring-primary/50">
            <span class="text-xs font-medium text-white/80">Continue on Error</span>
          </label>
          
          <div class="h-8 w-px bg-white/10 mx-1"></div>

          <button 
            @click="exportWorkflow"
            class="px-3 py-1.5 rounded-lg bg-white/5 hover:bg-white/10 text-xs font-medium text-white/80 transition-colors flex items-center gap-2"
          >
            <span class="i-lucide-download w-3 h-3"></span> Export
          </button>
          <label class="px-3 py-1.5 rounded-lg bg-white/5 hover:bg-white/10 text-xs font-medium text-white/80 transition-colors flex items-center gap-2 cursor-pointer">
            <span class="i-lucide-upload w-3 h-3"></span> Import
            <input type="file" accept=".json,.cf-flow" class="hidden" @change="importWorkflow">
          </label>

          <div class="h-8 w-px bg-white/10 mx-1"></div>

          <button 
            @click="toggleRecording"
            :class="[
              'px-3 py-1.5 rounded-lg text-sm font-medium transition-all flex items-center gap-2',
              isRecording ? 'bg-error text-white animate-pulse' : 'bg-white/5 text-white/70 hover:bg-white/10'
            ]"
          >
            <span class="w-2 h-2 rounded-full bg-current"></span>
            {{ isRecording ? 'Recording...' : 'Record Macro' }}
          </button>
          <VisionButton @click="executeWorkflow" :disabled="isExecuting || steps.length === 0">
            <span v-if="isExecuting" class="i-lucide-loader-2 animate-spin mr-2"></span>
            {{ isExecuting ? 'Running...' : 'Execute Plan' }}
          </VisionButton>
        </div>
      </div>

      <div class="flex-1 overflow-y-auto custom-scrollbar space-y-3 pr-2 bg-black/20 rounded-xl p-4 border border-white/5">
        <div v-if="steps.length === 0" class="h-full flex flex-col items-center justify-center text-white/30">
          <span class="i-lucide-mouse-pointer-click w-12 h-12 mb-2"></span>
          <p>Select tools from the left to build your workflow</p>
        </div>

        <div
          v-for="(step, index) in steps"
          :key="index"
          class="relative group bg-surface/50 border border-white/10 rounded-xl p-4 hover:border-primary/30 transition-all"
        >
          <div class="flex items-center justify-between mb-3">
            <div class="flex items-center gap-2">
              <span class="w-6 h-6 rounded-full bg-primary/20 text-primary text-xs flex items-center justify-center font-bold">
                {{ index + 1 }}
              </span>
              <span class="font-medium text-white uppercase tracking-wider text-sm">{{ step.type }}</span>
            </div>
            <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
              <button @click="moveStep(index, 'up')" class="p-1 hover:text-primary text-white/50" :disabled="index === 0">
                <span class="i-lucide-arrow-up"></span>
              </button>
              <button @click="moveStep(index, 'down')" class="p-1 hover:text-primary text-white/50" :disabled="index === steps.length - 1">
                <span class="i-lucide-arrow-down"></span>
              </button>
              <button @click="removeStep(index)" class="p-1 hover:text-error text-white/50 ml-2">
                <span class="i-lucide-trash-2"></span>
              </button>
            </div>
          </div>

          <!-- Dynamic Params Form -->
          <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
            <div v-for="(value, key) in step.params" :key="key" class="flex flex-col gap-1">
              <label class="text-xs text-white/50 uppercase">{{ key }}</label>
              <input
                v-if="typeof value === 'string' || typeof value === 'number'"
                v-model="step.params[key]"
                class="bg-black/30 border border-white/10 rounded-lg px-3 py-1.5 text-sm text-white focus:border-primary/50 focus:outline-none"
              />
              <div v-else-if="Array.isArray(value)" class="flex flex-col gap-1">
                 <input
                  :value="value.join(', ')"
                  @input="step.params[key] = ($event.target as HTMLInputElement).value.split(',').map(s => s.trim())"
                  class="bg-black/30 border border-white/10 rounded-lg px-3 py-1.5 text-sm text-white focus:border-primary/50 focus:outline-none"
                  placeholder="Comma separated values"
                />
              </div>
            </div>
          </div>
        </div>
      </div>
    </GlassCard>
  </div>
</template>
