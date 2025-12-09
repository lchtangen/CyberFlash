<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useSystemStore } from '../../../stores/system';
import { useNotificationStore } from '../../../stores/notifications';
import VisionButton from '../../ui/VisionButton.vue';

interface DriverStatus {
  platform: string;
  adb_installed: boolean;
  fastboot_installed: boolean;
  udev_rules_ok: boolean;
  drivers_ok: boolean;
  message: string;
}

const systemStore = useSystemStore();
const notificationStore = useNotificationStore();
const status = ref<DriverStatus | null>(null);
const loading = ref(true);
const fixing = ref(false);
const fixResult = ref<string | null>(null);

// Only show if this is the active modal
const showModal = computed(() => systemStore.activeModal === 'drivers');

const checkDrivers = async () => {
  loading.value = true;
  try {
    status.value = await invoke<DriverStatus>('check_drivers');
    if (!status.value.adb_installed || !status.value.fastboot_installed || !status.value.udev_rules_ok || !status.value.drivers_ok) {
      systemStore.setActiveModal('drivers');
      notificationStore.addNotification({
        title: 'Driver Issues Found',
        message: 'Some components are missing or misconfigured.',
        type: 'warning'
      });
    } else {
      // If all good, ensure we don't block
      if (systemStore.activeModal === 'drivers') {
        systemStore.setActiveModal(null);
      }
    }
  } catch (e) {
    console.error(e);
    notificationStore.addNotification({
      title: 'Driver Check Failed',
      message: String(e),
      type: 'error'
    });
  } finally {
    loading.value = false;
    systemStore.setDriverCheckComplete(true);
  }
};

const fixDrivers = async () => {
  fixing.value = true;
  fixResult.value = null;
  try {
    const result = await invoke<string>('fix_drivers');
    if (result.startsWith('LINUX_CMD:')) {
      fixResult.value = result.replace('LINUX_CMD:', '');
    } else {
      notificationStore.addNotification({
        title: 'Fix Initiated',
        message: result,
        type: 'success'
      });
      // Re-check after a delay
      setTimeout(checkDrivers, 5000);
    }
  } catch (e) {
    notificationStore.addNotification({
      title: 'Auto-Fix Failed',
      message: String(e),
      type: 'error'
    });
  } finally {
    fixing.value = false;
  }
};

const copyCommand = () => {
  if (fixResult.value) {
    navigator.clipboard.writeText(fixResult.value);
    notificationStore.addNotification({
      title: 'Copied',
      message: 'Command copied to clipboard',
      type: 'success'
    });
  }
};

onMounted(() => {
  checkDrivers();
});

const close = () => {
  systemStore.setActiveModal(null);
};
</script>

<template>
  <div v-if="showModal" class="fixed inset-0 z-[60] flex items-center justify-center bg-black/80 backdrop-blur-xl">
    <div class="bg-surface/80 border border-white/10 ring-1 ring-white/10 rounded-2xl p-6 max-w-lg w-full shadow-2xl backdrop-blur-xl">
      <h2 class="text-2xl font-bold text-white mb-4 flex items-center gap-2">
        <span class="material-symbols-rounded text-warning">warning</span>
        System Health Check
      </h2>
      
      <div v-if="loading" class="text-center py-8">
        <div class="w-8 h-8 border-2 border-primary border-t-transparent rounded-full animate-spin mx-auto mb-2"></div>
        <span class="text-sm text-white/60">Scanning system configuration...</span>
      </div>
      
      <div v-else-if="status" class="space-y-6">
        <div class="bg-white/5 rounded-xl p-4 border border-white/5">
          <p class="text-white/80 text-sm mb-4">{{ status.message }}</p>
          
          <div class="space-y-3">
            <div class="flex justify-between items-center">
              <span class="text-sm text-white/60">Platform</span>
              <span class="text-sm font-mono text-white bg-white/10 px-2 py-0.5 rounded">{{ status.platform }}</span>
            </div>
            <div class="flex justify-between items-center">
              <span class="text-sm text-white/60">ADB Binary</span>
              <div class="flex items-center gap-2">
                <span :class="status.adb_installed ? 'text-success' : 'text-error'" class="text-sm font-medium">
                  {{ status.adb_installed ? 'Installed' : 'Missing' }}
                </span>
                <span class="material-symbols-rounded text-lg" :class="status.adb_installed ? 'text-success' : 'text-error'">
                  {{ status.adb_installed ? 'check_circle' : 'cancel' }}
                </span>
              </div>
            </div>
            <div class="flex justify-between items-center">
              <span class="text-sm text-white/60">Fastboot Binary</span>
              <div class="flex items-center gap-2">
                <span :class="status.fastboot_installed ? 'text-success' : 'text-error'" class="text-sm font-medium">
                  {{ status.fastboot_installed ? 'Installed' : 'Missing' }}
                </span>
                <span class="material-symbols-rounded text-lg" :class="status.fastboot_installed ? 'text-success' : 'text-error'">
                  {{ status.fastboot_installed ? 'check_circle' : 'cancel' }}
                </span>
              </div>
            </div>
            <div v-if="status.platform === 'linux'" class="flex justify-between items-center">
              <span class="text-sm text-white/60">Udev Rules</span>
              <div class="flex items-center gap-2">
                <span :class="status.udev_rules_ok ? 'text-success' : 'text-error'" class="text-sm font-medium">
                  {{ status.udev_rules_ok ? 'Configured' : 'Missing' }}
                </span>
                <span class="material-symbols-rounded text-lg" :class="status.udev_rules_ok ? 'text-success' : 'text-error'">
                  {{ status.udev_rules_ok ? 'check_circle' : 'cancel' }}
                </span>
              </div>
            </div>
          </div>
        </div>

        <!-- Fix Result (Linux Command) -->
        <div v-if="fixResult" class="bg-black/40 rounded-xl p-4 border border-primary/30">
          <p class="text-xs text-primary mb-2 font-bold uppercase">Action Required</p>
          <p class="text-sm text-white/80 mb-3">Please run this command in your terminal to apply the fix:</p>
          <div class="flex gap-2">
            <code class="flex-1 bg-black/50 p-3 rounded-lg text-xs font-mono text-white/90 break-all border border-white/10">
              {{ fixResult }}
            </code>
            <VisionButton @click="copyCommand" variant="secondary" icon="content_copy" class="!p-3" />
          </div>
        </div>

        <div class="flex justify-end gap-3 pt-2">
          <VisionButton 
            @click="close" 
            variant="secondary"
          >
            Ignore
          </VisionButton>
          <VisionButton 
            @click="fixDrivers" 
            :loading="fixing"
            :disabled="status.adb_installed && status.fastboot_installed && status.udev_rules_ok"
            icon="auto_fix_high"
          >
            Auto-Fix Issues
          </VisionButton>
        </div>
      </div>
    </div>
  </div>
</template>
