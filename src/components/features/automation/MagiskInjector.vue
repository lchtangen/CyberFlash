<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { useNotificationStore } from '../../../stores/notifications';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';

const steps = ['Select', 'Push', 'Patch', 'Flash'];
const currentStep = ref(0);
const bootImgPath = ref('');
const patchedImgPath = ref('');
const isProcessing = ref(false);
const logs = ref<string[]>([]);
const notificationStore = useNotificationStore();

const log = (msg: string) => logs.value.push(`> ${msg}`);

const selectBootImg = async () => {
  try {
    const file = await open({
      filters: [{ name: 'Boot Image', extensions: ['img'] }]
    });
    if (file) {
      bootImgPath.value = Array.isArray(file) ? file[0] : file;
      log(`Selected: ${bootImgPath.value}`);
      currentStep.value = 1;
    }
  } catch (e) {
    log(`Error: ${e}`);
  }
};

const pushToDevice = async () => {
  isProcessing.value = true;
  try {
    log('Pushing boot.img to /sdcard/Download/...');
    await invoke('push_file', { 
      localPath: bootImgPath.value, 
      remotePath: '/sdcard/Download/boot.img' 
    });
    log('Push complete.');
    currentStep.value = 2;
  } catch (e) {
    log(`Push Failed: ${e}`);
  } finally {
    isProcessing.value = false;
  }
};

const checkForPatchedFile = async () => {
  isProcessing.value = true;
  try {
    log('Searching for magisk_patched_*.img...');
    const output = await invoke<string>('run_adb_shell', { 
      command: 'ls /sdcard/Download/magisk_patched_*.img' 
    });
    
    const files = output.trim().split('\n').filter(f => f.includes('magisk_patched'));
    if (files.length > 0) {
      // Get the latest one (last in list usually)
      const remoteFile = files[files.length - 1].trim();
      log(`Found: ${remoteFile}`);
      
      // Pull it
      const localDest = bootImgPath.value.replace('.img', '_patched.img');
      log(`Pulling to ${localDest}...`);
      await invoke('pull_file', { 
        remotePath: remoteFile, 
        localPath: localDest 
      });
      
      patchedImgPath.value = localDest;
      log('Pull complete.');
      
      // Cleanup remote
      await invoke('run_adb_shell', { command: `rm ${remoteFile} /sdcard/Download/boot.img` });
      log('Cleaned up remote files.');
      
      currentStep.value = 3;
    } else {
      log('No patched file found yet. Did you patch it in the Magisk App?');
    }
  } catch (e) {
    log(`Check Failed: ${e}`);
  } finally {
    isProcessing.value = false;
  }
};

const flashPatched = async () => {
  isProcessing.value = true;
  try {
    log('Rebooting to Bootloader...');
    await invoke('reboot_device', { mode: 'bootloader' });
    
    // Wait a bit (in real app, use wait_for_device)
    log('Waiting for fastboot...');
    await new Promise(r => setTimeout(r, 5000));
    
    log('Flashing boot partition...');
    await invoke('flash_partition', { 
      partition: 'boot', 
      imagePath: patchedImgPath.value 
    });
    
    log('Flash Complete! Rebooting...');
    await invoke('fastboot_boot', { imagePath: '' }); // Just reboot
    
    notificationStore.addNotification({
      title: 'Rooted!',
      message: 'Device successfully patched and flashed.',
      type: 'success'
    });
    
    currentStep.value = 0; // Reset
  } catch (e) {
    log(`Flash Failed: ${e}`);
  } finally {
    isProcessing.value = false;
  }
};
</script>

<template>
  <GlassCard>
    <div class="flex items-center gap-3 mb-6">
      <div class="p-3 rounded-xl bg-success/20 text-success">
        <span class="material-symbols-rounded text-2xl">android</span>
      </div>
      <div>
        <h2 class="text-xl font-bold text-white">Magisk Auto-Patcher</h2>
        <p class="text-sm text-white/60">Semi-automated root injection flow.</p>
      </div>
    </div>

    <!-- Stepper -->
    <div class="flex justify-between mb-8 relative">
      <div class="absolute top-1/2 left-0 w-full h-0.5 bg-white/10 -z-10"></div>
      <div 
        v-for="(step, index) in steps" 
        :key="step"
        class="flex flex-col items-center gap-2 bg-surface px-2"
      >
        <div 
          class="w-8 h-8 rounded-full flex items-center justify-center text-xs font-bold transition-colors duration-300"
          :class="index <= currentStep ? 'bg-success text-white' : 'bg-white/10 text-white/40'"
        >
          {{ index + 1 }}
        </div>
        <span class="text-xs font-medium" :class="index <= currentStep ? 'text-white' : 'text-white/40'">{{ step }}</span>
      </div>
    </div>

    <!-- Content Area -->
    <div class="min-h-[200px] flex flex-col items-center justify-center text-center space-y-6">
      
      <!-- Step 0: Select -->
      <div v-if="currentStep === 0" class="w-full">
        <div 
          @click="selectBootImg"
          class="border-2 border-dashed border-white/20 rounded-xl p-8 hover:bg-white/5 cursor-pointer transition-all"
        >
          <span class="material-symbols-rounded text-4xl text-white/40 mb-2">upload_file</span>
          <p class="text-white font-medium">Click to select original boot.img</p>
          <p class="text-xs text-white/40 mt-1">Extracted from your ROM zip</p>
        </div>
      </div>

      <!-- Step 1: Push -->
      <div v-if="currentStep === 1">
        <p class="text-white mb-4">Ready to push <code class="bg-white/10 px-1 rounded">{{ bootImgPath.split('/').pop() }}</code> to device.</p>
        <VisionButton @click="pushToDevice" :loading="isProcessing" icon="upload">Push to /sdcard</VisionButton>
      </div>

      <!-- Step 2: Patch Instruction -->
      <div v-if="currentStep === 2" class="max-w-md">
        <div class="bg-warning/10 border border-warning/20 p-4 rounded-xl mb-6 text-left">
          <h4 class="text-warning font-bold mb-2 flex items-center gap-2">
            <span class="material-symbols-rounded">touch_app</span> Manual Action Required
          </h4>
          <ol class="list-decimal list-inside text-sm text-white/80 space-y-1">
            <li>Open <b>Magisk App</b> on your phone.</li>
            <li>Tap <b>Install</b> (top card).</li>
            <li>Select <b>"Select and Patch a File"</b>.</li>
            <li>Choose <b>boot.img</b> from <b>Downloads</b>.</li>
            <li>Wait for "Output file is written to..."</li>
          </ol>
        </div>
        <VisionButton @click="checkForPatchedFile" :loading="isProcessing" icon="search">I've Finished Patching</VisionButton>
      </div>

      <!-- Step 3: Flash -->
      <div v-if="currentStep === 3">
        <div class="bg-success/10 border border-success/20 p-4 rounded-xl mb-6">
          <p class="text-success font-bold">Patched Image Retrieved!</p>
          <p class="text-xs text-white/60 mt-1 break-all">{{ patchedImgPath }}</p>
        </div>
        <VisionButton @click="flashPatched" :loading="isProcessing" variant="danger" icon="flash_on">Flash Patched Boot</VisionButton>
      </div>

    </div>

    <!-- Logs -->
    <div class="mt-6 p-3 rounded-lg bg-black/40 font-mono text-[10px] text-white/60 h-24 overflow-y-auto custom-scrollbar">
      <div v-for="(l, i) in logs" :key="i">{{ l }}</div>
    </div>

  </GlassCard>
</template>
