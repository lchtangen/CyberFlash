<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useNotificationStore } from '../../../stores/notifications';

const notificationStore = useNotificationStore();
const isListening = ref(false);
const transcript = ref('');
const recognition = ref<any>(null);

// Initialize Speech Recognition
onMounted(() => {
  if ('webkitSpeechRecognition' in window || 'SpeechRecognition' in window) {
    const SpeechRecognition = (window as any).SpeechRecognition || (window as any).webkitSpeechRecognition;
    recognition.value = new SpeechRecognition();
    recognition.value.continuous = false;
    recognition.value.interimResults = false;
    recognition.value.lang = 'en-US';

    recognition.value.onresult = (event: any) => {
      const last = event.results.length - 1;
      const text = event.results[last][0].transcript;
      transcript.value = text;
      processCommand(text);
    };

    recognition.value.onerror = (event: any) => {
      console.error('Speech recognition error', event.error);
      isListening.value = false;
      notificationStore.addNotification({
        title: 'Voice Error',
        message: `Could not understand audio: ${event.error}`,
        type: 'error'
      });
    };

    recognition.value.onend = () => {
      isListening.value = false;
    };
  } else {
    console.warn('Speech Recognition not supported in this browser/webview');
  }
});

const toggleListening = () => {
  if (!recognition.value) {
    notificationStore.addNotification({
      title: 'Not Supported',
      message: 'Voice commands are not supported on this system.',
      type: 'warning'
    });
    return;
  }

  if (isListening.value) {
    recognition.value.stop();
    isListening.value = false;
  } else {
    recognition.value.start();
    isListening.value = true;
    transcript.value = 'Listening...';
  }
};

const processCommand = async (text: string) => {
  const cmd = text.toLowerCase();
  notificationStore.addNotification({
    title: 'Voice Command',
    message: `Heard: "${text}"`,
    type: 'info'
  });

  try {
    if (cmd.includes('reboot') && cmd.includes('recovery')) {
      await invoke('reboot_device', { mode: 'recovery' });
    } else if (cmd.includes('reboot') && cmd.includes('bootloader')) {
      await invoke('reboot_device', { mode: 'bootloader' });
    } else if (cmd.includes('reboot') && cmd.includes('system')) {
      await invoke('reboot_device', { mode: 'system' });
    } else if (cmd.includes('flash') || cmd.includes('install')) {
      // Navigate to flash view?
      // For now just notify
      notificationStore.addNotification({
        title: 'Action Required',
        message: 'Please select a ROM to flash manually for safety.',
        type: 'warning'
      });
    } else {
      notificationStore.addNotification({
        title: 'Unknown Command',
        message: 'Try "Reboot to Recovery" or "Reboot System"',
        type: 'warning'
      });
    }
  } catch (e) {
    notificationStore.addNotification({
      title: 'Command Failed',
      message: String(e),
      type: 'error'
    });
  }
};
</script>

<template>
  <div class="fixed bottom-6 right-6 z-50">
    <button 
      @click="toggleListening"
      class="w-14 h-14 rounded-full flex items-center justify-center shadow-lg transition-all duration-300 backdrop-blur-xl border border-white/10"
      :class="isListening ? 'bg-error text-white animate-pulse shadow-error/50' : 'bg-surface/80 text-primary hover:bg-surface hover:scale-110'"
    >
      <span class="material-symbols-rounded text-2xl">
        {{ isListening ? 'mic' : 'mic_none' }}
      </span>
    </button>
    
    <!-- Transcript Bubble -->
    <div v-if="isListening || transcript" class="absolute bottom-16 right-0 w-64 p-3 rounded-xl bg-black/60 backdrop-blur-md border border-white/10 text-white text-sm mb-2 transition-opacity duration-500" :class="transcript ? 'opacity-100' : 'opacity-0'">
      {{ transcript }}
    </div>
  </div>
</template>
