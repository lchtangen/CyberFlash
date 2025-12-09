<script setup lang="ts">
import { ref, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useNotificationStore } from '../../../stores/notifications';

defineProps<{
  isOpen: boolean;
}>();

const emit = defineEmits(['close']);

const notificationStore = useNotificationStore();
const videoRef = ref<HTMLVideoElement | null>(null);
const canvasRef = ref<HTMLCanvasElement | null>(null);
const stream = ref<MediaStream | null>(null);
const isAnalyzing = ref(false);
const loopDetected = ref(false);
const confidence = ref(0);
const frameHistory = ref<{ hash: string; time: number }[]>([]);
const rescueTriggered = ref(false);

interface TranslatedText {
  original: string;
  translated: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

const isTranslating = ref(false);
const translations = ref<TranslatedText[]>([]);

const toggleTranslation = async () => {
  isTranslating.value = !isTranslating.value;
  if (isTranslating.value) {
    translateFrame();
  } else {
    translations.value = [];
  }
};

const translateFrame = async () => {
  if (!isTranslating.value || !videoRef.value || !canvasRef.value) return;

  const ctx = canvasRef.value.getContext('2d');
  if (!ctx) return;

  ctx.drawImage(videoRef.value, 0, 0, 320, 240);
  const imageData = canvasRef.value.toDataURL('image/jpeg');
  // Strip prefix
  const base64 = imageData.split(',')[1];

  try {
    translations.value = await invoke<TranslatedText[]>('translate_image', { imageData: base64 });
  } catch (e) {
    console.error(e);
  }

  if (isTranslating.value) {
    setTimeout(translateFrame, 2000); // Refresh every 2s
  }
};

// Simple perceptual hash (average hashing)
const getFrameHash = (ctx: CanvasRenderingContext2D): string => {
  const imageData = ctx.getImageData(0, 0, 32, 32);
  const data = imageData.data;
  let total = 0;
  for (let i = 0; i < data.length; i += 4) {
    total += data[i]; // Just use Red channel for speed, usually enough for contrast
  }
  const avg = total / (32 * 32);
  let hash = '';
  for (let i = 0; i < data.length; i += 4) {
    hash += data[i] > avg ? '1' : '0';
  }
  return hash;
};

const calculateSimilarity = (hash1: string, hash2: string): number => {
  let match = 0;
  for (let i = 0; i < hash1.length; i++) {
    if (hash1[i] === hash2[i]) match++;
  }
  return match / hash1.length;
};

const startCamera = async () => {
  try {
    stream.value = await navigator.mediaDevices.getUserMedia({ video: { width: 320, height: 240 } });
    if (videoRef.value) {
      videoRef.value.srcObject = stream.value;
    }
    isAnalyzing.value = true;
    loopDetected.value = false;
    rescueTriggered.value = false;
    confidence.value = 0;
    frameHistory.value = [];
    analyzeLoop();
  } catch (e) {
    console.error("Camera error", e);
    notificationStore.addNotification({
      title: 'Camera Error',
      message: 'Could not access webcam. Please ensure you have granted permissions.',
      type: 'error'
    });
    emit('close');
  }
};

const stopCamera = () => {
  if (stream.value) {
    stream.value.getTracks().forEach(track => track.stop());
    stream.value = null;
  }
  isAnalyzing.value = false;
};

const analyzeLoop = () => {
  if (!isAnalyzing.value || !videoRef.value || !canvasRef.value) return;

  const ctx = canvasRef.value.getContext('2d');
  if (!ctx) return;

  // Draw current frame to small canvas for processing
  ctx.drawImage(videoRef.value, 0, 0, 32, 32);
  const currentHash = getFrameHash(ctx);
  const now = Date.now();

  // Add to history (keep last 20 seconds approx, @ 500ms interval = 40 frames)
  frameHistory.value.push({ hash: currentHash, time: now });
  if (frameHistory.value.length > 40) frameHistory.value.shift();

  // Analysis Logic:
  // A bootloop typically looks like: [Logo] -> [Black] -> [Logo] -> [Black]
  // We look for a frame that matches a frame from > 3 seconds ago, 
  // AND there was a significantly different frame in between.
  
  if (frameHistory.value.length > 10) {
    const current = frameHistory.value[frameHistory.value.length - 1];
    
    // Look for a match in the past (between 3s and 15s ago)
    let matchFound = false;
    let differentInBetween = false;

    for (let i = frameHistory.value.length - 5; i >= 0; i--) {
      const past = frameHistory.value[i];
      const timeDiff = current.time - past.time;
      
      if (timeDiff > 3000 && timeDiff < 15000) {
        const sim = calculateSimilarity(current.hash, past.hash);
        if (sim > 0.90) { // 90% similar
          matchFound = true;
          
          // Check if there was a different frame in between
          for (let j = i + 1; j < frameHistory.value.length - 1; j++) {
            const mid = frameHistory.value[j];
            const midSim = calculateSimilarity(current.hash, mid.hash);
            if (midSim < 0.60) { // Significantly different (e.g. black screen vs logo)
              differentInBetween = true;
              break;
            }
          }
        }
      }
    }

    if (matchFound && differentInBetween) {
      confidence.value = Math.min(confidence.value + 20, 100);
    } else {
      confidence.value = Math.max(confidence.value - 5, 0);
    }

    if (confidence.value >= 80 && !loopDetected.value) {
      loopDetected.value = true;
      triggerRescue();
    }
  }

  if (isAnalyzing.value) {
    setTimeout(analyzeLoop, 500);
  }
};

const triggerRescue = async () => {
  if (rescueTriggered.value) return;
  rescueTriggered.value = true;
  
  notificationStore.addNotification({
    title: 'Bootloop Detected!',
    message: 'Visual analysis confirmed a bootloop. Attempting rescue...',
    type: 'warning'
  });

  try {
    const result = await invoke<string>('trigger_rescue_mode', { strategy: 'reboot_recovery' });
    notificationStore.addNotification({
      title: 'Rescue Signal Sent',
      message: result,
      type: 'success'
    });
  } catch (e) {
    notificationStore.addNotification({
      title: 'Rescue Failed',
      message: String(e),
      type: 'error'
    });
  }
};

const close = () => {
  stopCamera();
  emit('close');
};

onUnmounted(() => {
  stopCamera();
});
</script>

<template>
  <div v-if="isOpen" class="fixed inset-0 z-[70] flex items-center justify-center bg-black/90 backdrop-blur-xl">
    <div class="bg-surface/90 border border-white/10 rounded-2xl p-6 max-w-2xl w-full shadow-2xl backdrop-blur-md flex flex-col gap-6">
      
      <div class="flex justify-between items-start">
        <div>
          <h2 class="text-2xl font-bold text-white flex items-center gap-2">
            <span class="i-lucide-eye text-primary"></span>
            Visual Bootloop Doctor
          </h2>
          <p class="text-white/60 text-sm mt-1">Point your webcam at the device screen to detect bootloops.</p>
        </div>
        <button @click="close" class="p-2 hover:bg-white/10 rounded-full text-white/60 hover:text-white transition-colors">
          <span class="i-lucide-x"></span>
        </button>
      </div>

      <div class="relative aspect-video bg-black rounded-xl overflow-hidden border border-white/10 shadow-inner group">
        <video ref="videoRef" autoplay playsinline muted class="w-full h-full object-cover opacity-80"></video>
        <canvas ref="canvasRef" width="32" height="32" class="hidden"></canvas>
        
        <!-- Translation Overlays -->
        <div v-if="isTranslating" class="absolute inset-0 pointer-events-none z-10">
          <div 
            v-for="(t, i) in translations" 
            :key="i"
            class="absolute bg-black/80 text-success text-xs px-2 py-1 rounded border border-success/30 backdrop-blur-sm whitespace-nowrap"
            :style="{ 
              left: `${(t.x / 320) * 100}%`, 
              top: `${(t.y / 240) * 100}%`,
            }"
          >
            {{ t.translated }}
          </div>
        </div>

        <!-- Overlay UI -->
        <div v-if="!stream" class="absolute inset-0 flex items-center justify-center">
          <button @click="startCamera" class="px-6 py-3 bg-primary hover:bg-primary-hover text-white rounded-xl font-bold shadow-lg shadow-primary/20 transition-all flex items-center gap-2">
            <span class="i-lucide-camera"></span>
            Start Camera Analysis
          </button>
        </div>

        <div v-else class="absolute top-4 right-4 flex flex-col gap-2 items-end">
          <button 
            @click="toggleTranslation"
            class="px-3 py-1 rounded-lg border text-xs font-medium transition-colors flex items-center gap-2 pointer-events-auto"
            :class="isTranslating ? 'bg-primary text-white border-primary' : 'bg-black/60 text-white/80 border-white/10 hover:bg-white/10'"
          >
            <span class="material-symbols-rounded text-sm">translate</span>
            {{ isTranslating ? 'Translating...' : 'Translate Text' }}
          </button>

          <div class="bg-black/60 backdrop-blur px-3 py-1 rounded-lg border border-white/10 text-xs font-mono text-white/80">
            <span class="w-2 h-2 rounded-full bg-red-500 inline-block mr-2 animate-pulse"></span>
            ANALYZING
          </div>
          <div class="bg-black/60 backdrop-blur px-3 py-1 rounded-lg border border-white/10 text-xs font-mono text-white/80">
            CONFIDENCE: {{ confidence }}%
          </div>
        </div>

        <!-- Detection Overlay -->
        <div v-if="loopDetected" class="absolute inset-0 flex items-center justify-center bg-red-500/20 backdrop-blur-sm animate-pulse">
          <div class="bg-black/80 p-6 rounded-2xl border border-red-500 text-center">
            <span class="i-lucide-alert-octagon text-4xl text-red-500 mb-2 block mx-auto"></span>
            <h3 class="text-xl font-bold text-white">BOOTLOOP DETECTED</h3>
            <p class="text-red-400 text-sm mt-1">Rescue sequence initiated...</p>
          </div>
        </div>
      </div>

      <div class="grid grid-cols-3 gap-4">
        <div class="bg-white/5 p-4 rounded-xl border border-white/5">
          <div class="text-xs text-white/40 uppercase font-bold mb-1">Status</div>
          <div class="text-sm font-medium" :class="isAnalyzing ? 'text-success' : 'text-white/60'">
            {{ isAnalyzing ? 'Monitoring' : 'Standby' }}
          </div>
        </div>
        <div class="bg-white/5 p-4 rounded-xl border border-white/5">
          <div class="text-xs text-white/40 uppercase font-bold mb-1">Pattern Match</div>
          <div class="w-full bg-white/10 h-2 rounded-full mt-2 overflow-hidden">
            <div class="h-full bg-primary transition-all duration-500" :style="{ width: confidence + '%' }"></div>
          </div>
        </div>
        <div class="bg-white/5 p-4 rounded-xl border border-white/5">
          <div class="text-xs text-white/40 uppercase font-bold mb-1">Action</div>
          <div class="text-sm font-medium text-white/80">
            {{ rescueTriggered ? 'Rebooting Recovery...' : 'Waiting for trigger...' }}
          </div>
        </div>
      </div>

    </div>
  </div>
</template>
