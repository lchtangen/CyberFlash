<script setup lang="ts">
import { ref, watch, nextTick, onMounted } from 'vue';

interface Props {
  logs: string[];
  title?: string;
  height?: string;
  autoScroll?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  title: 'NEURAL_CONSOLE_V2',
  height: '100%',
  autoScroll: true
});

const logContainer = ref<HTMLElement | null>(null);

const scrollToBottom = () => {
  if (props.autoScroll && logContainer.value) {
    nextTick(() => {
      logContainer.value!.scrollTop = logContainer.value!.scrollHeight;
    });
  }
};

watch(() => props.logs.length, scrollToBottom);
onMounted(scrollToBottom);
</script>

<template>
  <div class="holographic-terminal relative overflow-hidden rounded-xl border border-primary/20 bg-[#050505]/80 backdrop-blur-md flex flex-col group" :style="{ height: height }">
    
    <!-- Holographic Header -->
    <div class="relative z-20 flex items-center justify-between px-4 py-2 border-b border-primary/20 bg-primary/5">
      <div class="flex items-center gap-2">
        <div class="w-2 h-2 rounded-full bg-primary animate-pulse shadow-[0_0_8px_rgba(10,132,255,0.8)]"></div>
        <span class="text-primary font-mono text-xs font-bold tracking-[0.2em] uppercase drop-shadow-[0_0_5px_rgba(10,132,255,0.5)]">
          {{ title }}
        </span>
      </div>
      <div class="flex gap-1">
        <div class="w-1 h-1 rounded-full bg-primary/30"></div>
        <div class="w-1 h-1 rounded-full bg-primary/30"></div>
        <div class="w-1 h-1 rounded-full bg-primary/30"></div>
      </div>
    </div>

    <!-- CRT Scanline Overlay -->
    <div class="absolute inset-0 pointer-events-none z-10 bg-[linear-gradient(rgba(18,16,16,0)_50%,rgba(0,0,0,0.25)_50%),linear-gradient(90deg,rgba(255,0,0,0.06),rgba(0,255,0,0.02),rgba(0,0,255,0.06))] bg-[length:100%_4px,3px_100%]"></div>
    <div class="absolute inset-0 pointer-events-none z-10 animate-scanline bg-gradient-to-b from-transparent via-primary/5 to-transparent h-[20%] w-full opacity-20"></div>

    <!-- Content -->
    <div ref="logContainer" class="relative z-0 flex-1 overflow-y-auto p-4 font-mono text-xs space-y-1 custom-scrollbar">
      <div v-if="logs.length === 0" class="text-primary/30 italic mt-4 text-center animate-pulse">
        // WAITING FOR INPUT STREAM...
      </div>
      
      <div v-for="(log, i) in logs" :key="i" class="flex gap-3 text-primary/90 drop-shadow-[0_0_2px_rgba(10,132,255,0.3)] animate-fade-in-up">
        <span class="text-primary/40 select-none shrink-0">[{{ i.toString().padStart(3, '0') }}]</span>
        <span class="break-all">{{ log }}</span>
      </div>
      
      <!-- Active Cursor -->
      <div class="flex items-center gap-2 mt-2 text-primary">
        <span class="animate-blink">_</span>
      </div>
    </div>

    <!-- Corner Accents -->
    <div class="absolute top-0 left-0 w-4 h-4 border-l border-t border-primary/50 rounded-tl-lg"></div>
    <div class="absolute top-0 right-0 w-4 h-4 border-r border-t border-primary/50 rounded-tr-lg"></div>
    <div class="absolute bottom-0 left-0 w-4 h-4 border-l border-b border-primary/50 rounded-bl-lg"></div>
    <div class="absolute bottom-0 right-0 w-4 h-4 border-r border-b border-primary/50 rounded-br-lg"></div>

  </div>
</template>

<style scoped>
.holographic-terminal {
  box-shadow: 0 0 20px rgba(10, 132, 255, 0.05), inset 0 0 20px rgba(10, 132, 255, 0.05);
}

.animate-scanline {
  animation: scanline 8s linear infinite;
}

@keyframes scanline {
  0% { transform: translateY(-100%); }
  100% { transform: translateY(500%); }
}

.animate-blink {
  animation: blink 1s step-end infinite;
}

@keyframes blink {
  0%, 100% { opacity: 1; }
  50% { opacity: 0; }
}

.animate-fade-in-up {
  animation: fadeInUp 0.3s ease-out forwards;
}

@keyframes fadeInUp {
  from { opacity: 0; transform: translateY(5px); }
  to { opacity: 1; transform: translateY(0); }
}

/* Custom Scrollbar for Terminal */
.custom-scrollbar::-webkit-scrollbar {
  width: 4px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: rgba(10, 132, 255, 0.05);
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(10, 132, 255, 0.3);
  border-radius: 2px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(10, 132, 255, 0.5);
}
</style>
