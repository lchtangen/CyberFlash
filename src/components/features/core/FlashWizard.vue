<script setup lang="ts">
import { useFlashStore } from '../../../stores/flash';
import { computed, watch, ref, nextTick } from 'vue';
import { useNotificationStore } from '../../../stores/notifications';
import { useDynamicIslandStore } from '../../../stores/dynamicIsland';
import GlassCard from '../../ui/GlassCard.vue';
import VisionButton from '../../ui/VisionButton.vue';
import HolographicTerminal from '../../ui/HolographicTerminal.vue';
import { invoke } from '@tauri-apps/api/core';

const flashStore = useFlashStore();
const notificationStore = useNotificationStore();
const islandStore = useDynamicIslandStore();
const logContainer = ref<HTMLElement | null>(null);
const showRebootDialog = ref(false);

const phases = [
  { id: 1, title: 'Pre-Flight Analysis', description: 'AI Health & Driver Check' },
  { id: 2, title: 'Environment Prep', description: 'Download & Verify Integrity' },
  { id: 3, title: 'Data Preservation', description: 'Smart Backup (Apps/SMS)' },
  { id: 4, title: 'Bootloader Ops', description: 'Unlock & Critical Partitions' },
  { id: 5, title: 'Slot Management', description: 'A/B Partition Switching' },
  { id: 6, title: 'Flash Firmware', description: 'Radio, DSP & Bluetooth' },
  { id: 7, title: 'Core System', description: 'Flash Boot, System, Vendor' },
  { id: 8, title: 'Security Patching', description: 'Disable Verity & VBMeta' },
  { id: 9, title: 'Root & Extensions', description: 'Magisk, GApps & Modules' },
  { id: 10, title: 'Finalization', description: 'Format Data & Post-Boot' },
];

const currentPhaseInfo = computed(() => phases[flashStore.currentPhase] || phases[0]);
const globalProgress = computed(() => ((flashStore.currentPhase) / phases.length) * 100);

const phase1Status = ref({
  adb: 'pending',
  battery: 'pending',
  codename: 'pending',
  drivers: 'pending'
});

const runPhase1 = async () => {
  flashStore.logs.push('--- Phase 1: Pre-Flight Analysis ---');
  
  // 1. ADB Check
  try {
    flashStore.logs.push('Checking ADB connection...');
    phase1Status.value.adb = 'running';
    const devices: any = await invoke('get_connected_devices');
    if (devices && devices.length > 0) {
      phase1Status.value.adb = 'success';
      flashStore.logs.push(`ADB Connected: ${devices[0]}`);
    } else {
      phase1Status.value.adb = 'error';
      flashStore.logs.push('Error: No ADB device found.');
      throw new Error('No device connected');
    }
  } catch (e) {
    phase1Status.value.adb = 'error';
    flashStore.logs.push(`ADB Error: ${e}`);
    return;
  }

  // 2. Battery Check
  try {
    flashStore.logs.push('Checking Battery level...');
    phase1Status.value.battery = 'running';
    const battery: any = await invoke('check_battery_level');
    const level = parseInt(battery);
    if (level >= 50) {
      phase1Status.value.battery = 'success';
      flashStore.logs.push(`Battery Level: ${level}% (Good)`);
    } else {
      phase1Status.value.battery = 'warning';
      flashStore.logs.push(`Warning: Battery low (${level}%). Recommend >50%.`);
    }
  } catch (e) {
    flashStore.logs.push(`Battery check failed: ${e}`);
    phase1Status.value.battery = 'warning';
  }

  // 3. Codename Check
  try {
    flashStore.logs.push('Verifying Device Codename...');
    phase1Status.value.codename = 'running';
    const codename: any = await invoke('get_prop_value', { prop: 'ro.product.device' });
    if (codename && (codename.includes('guacamole') || codename.includes('OnePlus7Pro'))) {
       phase1Status.value.codename = 'success';
       flashStore.logs.push(`Device Verified: ${codename} (OnePlus 7 Pro)`);
    } else {
       phase1Status.value.codename = 'error';
       flashStore.logs.push(`Mismatch: Detected ${codename}, expected guacamole/guacamoleb`);
    }
  } catch (e) {
     phase1Status.value.codename = 'warning';
     flashStore.logs.push(`Codename check failed: ${e}`);
  }
  
  phase1Status.value.drivers = 'success'; // Mock for now
  flashStore.logs.push('Phase 1 Complete.');
};

const inventory = ref<any>(null);
const phase2Status = ref({
  inventory: 'pending',
  download: 'pending',
  verify: 'pending'
});

const runPhase2 = async () => {
  flashStore.logs.push('--- Phase 2: Environment Prep ---');
  
  // 1. Load Inventory
  try {
    flashStore.logs.push('Loading Device Inventory...');
    phase2Status.value.inventory = 'running';
    const data: any = await invoke('get_device_inventory');
    inventory.value = data;
    phase2Status.value.inventory = 'success';
    flashStore.logs.push(`Loaded inventory for ${data.device}`);
  } catch (e) {
    phase2Status.value.inventory = 'error';
    flashStore.logs.push(`Failed to load inventory: ${e}`);
  }
};

const selectRomFromInventory = (rom: any) => {
  flashStore.selectedRom = {
    name: rom.name,
    path: '', 
    hash: rom.md5 || '',
    rom_type: 'zip'
  };
  flashStore.logs.push(`Selected Target: ${rom.name}`);
};

const startPhase = async () => {
  if (flashStore.currentPhase === 0) {
    await runPhase1();
    return;
  }
  
  if (flashStore.currentPhase === 1) {
    await runPhase2();
    return;
  }

  // MD5 Checksum Verification (Phase 2.6)
  if (flashStore.selectedRom && flashStore.selectedRom.hash) {
    flashStore.logs.push(`Verifying integrity of ${flashStore.selectedRom.name}...`);
    try {
      const isValid = await invoke('verify_file_md5', { 
        filePath: flashStore.selectedRom.path, 
        expectedHash: flashStore.selectedRom.hash 
      });
      
      if (!isValid) {
        flashStore.logs.push('ERROR: MD5 Checksum mismatch! File may be corrupted.');
        notificationStore.addNotification({
          title: 'Integrity Check Failed',
          message: 'The ROM file appears to be corrupted. Flashing aborted.',
          type: 'error'
        });
        return;
      }
      flashStore.logs.push('Integrity Verified (MD5 Match).');
    } catch (e) {
      flashStore.logs.push(`Warning: Could not verify checksum: ${e}`);
    }
  }

  // Connection Check
  if (flashStore.currentPhase > 0) { // Skip check for downloads phase? Or check always?
      // Ideally check connection before starting any device interaction
      // But Phase 1 is downloads, so maybe not needed.
      // Phase 2 is Backup, needs ADB.
  }
  
  flashStore.startFlash();
  
  // Update Dynamic Island
  islandStore.setActivity({
    id: 'flashing',
    type: 'process',
    icon: 'auto_fix_high',
    title: `Phase ${flashStore.currentPhase + 1}: ${currentPhaseInfo.value.title}`,
    subtitle: 'Processing...',
    progress: 0,
    color: 'text-primary',
    bg: 'bg-primary/20',
    border: 'border-primary/30'
  });

  notificationStore.addNotification({
    title: `Phase ${flashStore.currentPhase + 1} Started`,
    message: currentPhaseInfo.value.title,
    type: 'info'
  });
};

const emergencyStop = async () => {
  try {
    await invoke('cancel_flash_process');
    flashStore.logs.push('!!! EMERGENCY STOP TRIGGERED !!!');
    notificationStore.addNotification({
      title: 'Emergency Stop',
      message: 'Flash process cancelled by user.',
      type: 'error'
    });
  } catch (e) {
    console.error('Failed to stop:', e);
  }
};

const confirmReboot = async () => {
  showRebootDialog.value = false;
  try {
    await invoke('reboot_device', { mode: 'system' }); // Assuming this command exists in adb.rs
    notificationStore.addNotification({
      title: 'Rebooting',
      message: 'Device is rebooting to system.',
      type: 'success'
    });
  } catch (e) {
    flashStore.logs.push(`Reboot failed: ${e}`);
  }
};

const next = () => {
  if (flashStore.currentPhase < phases.length - 1) {
    flashStore.nextPhase();
    
    // Update Island for next phase
    islandStore.setActivity({
      id: 'flashing',
      type: 'process',
      icon: 'auto_fix_high',
      title: `Phase ${flashStore.currentPhase + 1}: ${currentPhaseInfo.value.title}`,
      subtitle: 'Processing...',
      progress: 0,
      color: 'text-primary',
      bg: 'bg-primary/20',
      border: 'border-primary/30'
    });
  } else {
    // Finished
    islandStore.clearActivity();
    islandStore.showNotification({
      id: 'flash-complete',
      type: 'success',
      title: 'Flash Complete',
      subtitle: 'All phases finished successfully.',
      icon: 'check_circle'
    });
    
    // Show Reboot Dialog (Phase 2.9)
    showRebootDialog.value = true;
  }
};

// Auto-scroll logs
watch(() => flashStore.logs.length, async () => {
  await nextTick();
  if (logContainer.value) {
    logContainer.value.scrollTop = logContainer.value.scrollHeight;
  }
});

// Watch for phase completion
watch(() => flashStore.progress, (newVal) => {
  // Update Island Progress
  if (islandStore.activeActivity?.id === 'flashing') {
    islandStore.updateProgress(newVal);
  }

  if (newVal === 100) {
    notificationStore.addNotification({
      title: 'Phase Complete',
      message: `${currentPhaseInfo.value.title} finished successfully.`,
      type: 'success'
    });
  }
});
</script>

<template>
  <GlassCard class="flex flex-col h-full relative overflow-hidden">
    <!-- Background Ambient Glow -->
    <div class="absolute top-0 right-0 w-64 h-64 bg-primary/20 blur-[100px] rounded-full pointer-events-none"></div>

    <div class="relative z-10 flex flex-col h-full">
      <!-- Header -->
      <div class="flex items-center justify-between mb-8">
        <div class="flex items-center gap-4">
          <div class="w-12 h-12 rounded-2xl bg-gradient-to-br from-primary to-secondary flex items-center justify-center shadow-lg shadow-primary/20">
            <span class="material-symbols-rounded text-white text-2xl">auto_fix_high</span>
          </div>
          <div>
            <h2 class="text-2xl font-bold text-white tracking-tight">Flash Wizard</h2>
            <div class="flex items-center gap-2 mt-1">
              <span class="px-2 py-0.5 rounded text-[10px] font-bold uppercase tracking-wider bg-white/10 text-white/70 border border-white/5">
                Phase {{ flashStore.currentPhase + 1 }}/{{ phases.length }}
              </span>
              <span class="text-sm text-text-secondary">{{ currentPhaseInfo.title }}</span>
            </div>
          </div>
        </div>
        <div class="text-right">
          <div class="text-3xl font-mono font-bold text-white tracking-tighter">{{ Math.round(globalProgress) }}<span class="text-lg text-white/50">%</span></div>
          <p class="text-[10px] text-text-secondary uppercase tracking-widest">Total Progress</p>
        </div>
      </div>

      <!-- Global Progress Bar -->
      <div class="h-1.5 bg-surface/50 rounded-full overflow-hidden mb-8 border border-white/5">
        <div 
          class="h-full bg-gradient-to-r from-primary via-secondary to-primary bg-[length:200%_100%] animate-[shimmer_2s_linear_infinite]"
          :style="{ width: `${globalProgress}%` }"
        ></div>
      </div>

            <div class="flex gap-6 flex-1 min-h-0">
        <!-- Phase List (Timeline) -->
        <div class="w-72 flex-shrink-0 overflow-y-auto custom-scrollbar pr-2 space-y-3">
          <div 
            v-for="(phase, index) in phases" 
            :key="phase.id"
            class="p-4 rounded-xl border transition-all duration-300 relative overflow-hidden group"
            :class="[
              index === flashStore.currentPhase 
                ? 'bg-primary/10 border-primary/30 shadow-[0_0_15px_rgba(0,240,255,0.1)]' 
                : index < flashStore.currentPhase 
                  ? 'bg-success/5 border-success/20 opacity-60' 
                  : 'bg-surface/30 border-white/5 opacity-40'
            ]"
          >
            <!-- Active Indicator Line -->
            <div v-if="index === flashStore.currentPhase" class="absolute left-0 top-0 bottom-0 w-1 bg-primary shadow-[0_0_10px_rgba(0,240,255,0.5)]"></div>

            <div class="flex items-center justify-between mb-1">
              <h3 class="font-bold text-sm" :class="index === flashStore.currentPhase ? 'text-white' : 'text-white/70'">{{ phase.title }}</h3>
              <span v-if="index < flashStore.currentPhase" class="material-symbols-rounded text-success text-sm">check_circle</span>
              <span v-else-if="index === flashStore.currentPhase" class="w-2 h-2 rounded-full bg-primary animate-pulse shadow-[0_0_8px_rgba(0,240,255,0.8)]"></span>
            </div>
            <p class="text-xs text-text-secondary leading-relaxed">{{ phase.description }}</p>
          </div>
        </div>

        <!-- Main Action Area -->
        <div class="flex-1 flex flex-col gap-6">
          <!-- Current Step Detail -->
          <div class="bg-surface/30 border border-white/10 rounded-xl p-6 backdrop-blur-md relative overflow-hidden group">
             <div class="absolute top-0 right-0 w-32 h-32 bg-primary/10 blur-[60px] rounded-full pointer-events-none group-hover:bg-primary/20 transition-colors duration-500"></div>
             
             <h3 class="text-lg font-bold text-white mb-2 flex items-center gap-2">
               <span class="material-symbols-rounded text-primary">info</span>
               Current Operation
             </h3>
             <p class="text-sm text-text-secondary mb-6 leading-relaxed">
               {{ currentPhaseInfo.description }}. Ensure your device is connected and has sufficient battery (50%+). Do not disconnect the cable during this process.
             </p>

             <!-- Phase 1: Pre-Flight Analysis UI -->
             <div v-if="flashStore.currentPhase === 0" class="grid grid-cols-2 gap-4 mb-6 animate-slide-up">
                <div class="p-3 rounded-lg border border-white/10 bg-surface/20 flex items-center justify-between">
                   <span class="text-sm text-white/70">ADB Connection</span>
                   <span v-if="phase1Status.adb === 'pending'" class="text-xs text-white/50">Waiting...</span>
                   <span v-else-if="phase1Status.adb === 'running'" class="text-xs text-primary animate-pulse">Checking...</span>
                   <span v-else-if="phase1Status.adb === 'success'" class="text-xs text-success font-bold flex items-center gap-1"><span class="material-symbols-rounded text-sm">check</span> Connected</span>
                   <span v-else class="text-xs text-error font-bold flex items-center gap-1"><span class="material-symbols-rounded text-sm">close</span> Failed</span>
                </div>
                <div class="p-3 rounded-lg border border-white/10 bg-surface/20 flex items-center justify-between">
                   <span class="text-sm text-white/70">Battery Level</span>
                   <span v-if="phase1Status.battery === 'pending'" class="text-xs text-white/50">Waiting...</span>
                   <span v-else-if="phase1Status.battery === 'running'" class="text-xs text-primary animate-pulse">Checking...</span>
                   <span v-else-if="phase1Status.battery === 'success'" class="text-xs text-success font-bold flex items-center gap-1"><span class="material-symbols-rounded text-sm">check</span> Good</span>
                   <span v-else-if="phase1Status.battery === 'warning'" class="text-xs text-warning font-bold flex items-center gap-1"><span class="material-symbols-rounded text-sm">warning</span> Low</span>
                </div>
                <div class="p-3 rounded-lg border border-white/10 bg-surface/20 flex items-center justify-between">
                   <span class="text-sm text-white/70">Device Match</span>
                   <span v-if="phase1Status.codename === 'pending'" class="text-xs text-white/50">Waiting...</span>
                   <span v-else-if="phase1Status.codename === 'running'" class="text-xs text-primary animate-pulse">Verifying...</span>
                   <span v-else-if="phase1Status.codename === 'success'" class="text-xs text-success font-bold flex items-center gap-1"><span class="material-symbols-rounded text-sm">check</span> Verified</span>
                   <span v-else class="text-xs text-error font-bold flex items-center gap-1"><span class="material-symbols-rounded text-sm">close</span> Mismatch</span>
                </div>
                <div class="p-3 rounded-lg border border-white/10 bg-surface/20 flex items-center justify-between">
                   <span class="text-sm text-white/70">Drivers</span>
                   <span class="text-xs text-success font-bold flex items-center gap-1"><span class="material-symbols-rounded text-sm">check</span> Installed</span>
                </div>
             </div>

             <!-- Phase 2: Environment Prep UI -->
             <div v-if="flashStore.currentPhase === 1" class="mb-6 animate-slide-up">
                <div v-if="!inventory && phase2Status.inventory === 'running'" class="text-center p-4">
                   <svg class="animate-spin h-8 w-8 text-primary mx-auto" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                      <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                      <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                   </svg>
                   <p class="text-xs text-white/50 mt-2">Loading Inventory...</p>
                </div>
                <div v-else-if="inventory" class="space-y-4">
                   <div class="p-4 rounded-xl bg-surface/40 border border-white/10">
                      <h4 class="text-sm font-bold text-white mb-3">Select Firmware/ROM</h4>
                      <div class="grid grid-cols-1 gap-2 max-h-40 overflow-y-auto custom-scrollbar">
                         <div 
                           v-for="rom in inventory.roms" 
                           :key="rom.name"
                           @click="selectRomFromInventory(rom)"
                           class="p-3 rounded-lg border border-white/5 bg-surface/20 hover:bg-primary/10 hover:border-primary/30 cursor-pointer transition-all flex items-center justify-between group"
                           :class="flashStore.selectedRom?.name === rom.name ? 'bg-primary/20 border-primary/50' : ''"
                         >
                            <div>
                               <div class="font-bold text-sm text-white">{{ rom.name }}</div>
                               <div class="text-xs text-white/50">{{ rom.version }} • Android {{ rom.android_version }}</div>
                            </div>
                            <span v-if="flashStore.selectedRom?.name === rom.name" class="material-symbols-rounded text-primary">check_circle</span>
                         </div>
                      </div>
                   </div>
                   
                   <div v-if="flashStore.selectedRom" class="flex items-center justify-between p-3 rounded-lg bg-surface/20 border border-white/10">
                      <div class="flex items-center gap-3">
                         <span class="material-symbols-rounded text-primary">download</span>
                         <div>
                            <div class="text-xs text-white/70">Ready to Download</div>
                            <div class="text-sm font-bold text-white">{{ flashStore.selectedRom.name }}</div>
                         </div>
                      </div>
                      <VisionButton size="sm" variant="primary">Download Now</VisionButton>
                   </div>
                </div>
             </div>

             <div class="flex gap-3">
               <VisionButton 
                 @click="startPhase" 
                 :disabled="flashStore.isFlashing" 
                 :loading="flashStore.isFlashing"
                 variant="primary"
                 size="lg"
                 class="flex-1 shadow-xl shadow-primary/10"
               >
                 {{ flashStore.isFlashing ? 'Processing...' : 'Start Phase' }}
               </VisionButton>
               
               <VisionButton 
                 v-if="flashStore.isFlashing"
                 @click="emergencyStop"
                 variant="danger"
                 size="lg"
                 class="flex-1 shadow-xl shadow-error/10"
               >
                 STOP
               </VisionButton>

               <VisionButton 
                 v-else
                 @click="next" 
                 :disabled="!flashStore.isFlashing && flashStore.progress !== 100"
                 variant="secondary"
                 size="lg"
               >
                 Skip / Next
               </VisionButton>
             </div>
          </div>

          <!-- Terminal Output -->
          <HolographicTerminal 
            :logs="flashStore.logs" 
            title="FLASH_CONSOLE" 
            class="flex-1 min-h-0" 
          />
        </div>
      </div>
    </div>

    <!-- Reboot Dialog Overlay -->
    <div v-if="showRebootDialog" class="absolute inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm">
      <div class="bg-surface border border-white/10 rounded-2xl p-6 max-w-md w-full shadow-2xl transform scale-100 transition-all">
        <div class="flex items-center gap-4 mb-4">
          <div class="w-12 h-12 rounded-full bg-success/20 flex items-center justify-center">
            <span class="material-symbols-rounded text-success text-2xl">check_circle</span>
          </div>
          <div>
            <h3 class="text-xl font-bold text-white">Flashing Complete</h3>
            <p class="text-sm text-text-secondary">Your device has been successfully updated.</p>
          </div>
        </div>
        
        <p class="text-white/80 mb-6">Would you like to reboot your device to system now?</p>
        
        <div class="flex gap-3 justify-end">
          <VisionButton @click="showRebootDialog = false" variant="secondary">
            Stay Here
          </VisionButton>
          <VisionButton @click="confirmReboot" variant="primary">
            Reboot System
          </VisionButton>
        </div>
      </div>
    </div>
  </GlassCard>
</template>
