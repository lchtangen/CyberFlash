import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

interface ProgressUpdate {
  phase_id: number;
  step_index: number;
  total_steps: number;
  message: string;
  percentage: number;
}

export const useFlashStore = defineStore('flash', () => {
  const currentPhase = ref(0) // 0-indexed, so Phase 1 is index 0
  const isFlashing = ref(false)
  const logs = ref<string[]>([])
  const progress = ref(0)
  const currentStep = ref('')

  async function startFlash() {
    if (isFlashing.value) return;
    
    isFlashing.value = true;
    logs.value = [];
    progress.value = 0;
    
    const phaseId = currentPhase.value + 1;
    logs.value.push(`Starting Phase ${phaseId}...`);

    const unlisten = await listen<ProgressUpdate>('flash-progress', (event) => {
      progress.value = event.payload.percentage;
      currentStep.value = event.payload.message;
      logs.value.push(`[${Math.round(event.payload.percentage)}%] ${event.payload.message}`);
    });

    const unlistenDownload = await listen<number>('download-progress', (event) => {
      progress.value = event.payload;
      // Don't spam logs with download progress
    });

    const unlistenSideload = await listen<string>('sideload-progress', (event) => {
      // ADB sideload output is usually "serving: 'path/to/rom.zip'  (~45%)"
      // We can try to parse percentage or just log it
      const msg = event.payload;
      currentStep.value = msg;
      logs.value.push(`[ADB] ${msg}`);
      
      // Simple percentage parser for standard ADB output
      if (msg.includes('%')) {
        const match = msg.match(/(\d+)%/);
        if (match) {
          progress.value = parseInt(match[1]);
        }
      }
    });

    try {
      await invoke('start_flash_process', { phaseId });
      logs.value.push(`Phase ${phaseId} Complete!`);
      progress.value = 100;
    } catch (e) {
      logs.value.push(`Error: ${e}`);
    } finally {
      isFlashing.value = false;
      unlisten();
      unlistenDownload();
      unlistenSideload();
    }
  }

  function nextPhase() {
    currentPhase.value++
    progress.value = 0
    currentStep.value = ''
  }

  return {
    currentPhase,
    isFlashing,
    logs,
    progress,
    currentStep,
    startFlash,
    nextPhase
  }
})
