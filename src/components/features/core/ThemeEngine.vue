<script setup lang="ts">
import { onMounted } from 'vue';
import { useSettingsStore } from '../../../stores/settings';
import { useNotificationStore } from '../../../stores/notifications';
import { useTheme } from '../../../composables/useTheme';

const settingsStore = useSettingsStore();
const notificationStore = useNotificationStore();
const { presets } = useTheme();

const resetTheme = () => {
  settingsStore.$reset();
  notificationStore.addNotification({
    title: 'Theme Reset',
    message: 'Theme settings restored to default.',
    type: 'success'
  });
};

onMounted(() => {
  // Theme is already initialized in App.vue, but we can re-apply here if needed
  // or just rely on the store reactivity which is set up in App.vue
});
</script>

<template>
  <div class="bg-surface/30 border border-white/10 rounded-xl p-6 backdrop-blur-md">
    <div class="flex items-center gap-3 mb-6">
      <div class="w-10 h-10 rounded-full bg-primary/20 flex items-center justify-center text-primary">
        <span class="material-symbols-rounded">palette</span>
      </div>
      <div>
        <h3 class="text-lg font-bold text-white">Theme Engine</h3>
        <p class="text-xs text-text-secondary">Control Center</p>
      </div>
    </div>

    <div class="space-y-6 max-h-[400px] overflow-y-auto custom-scrollbar pr-2">
      
      <!-- 1. Theme Preset -->
      <div>
        <div class="flex justify-between items-center mb-2">
          <label class="text-xs text-text-secondary font-medium uppercase tracking-wider">Theme Preset</label>
          <button @click="resetTheme" class="text-[10px] text-text-muted hover:text-white transition-colors">Reset Default</button>
        </div>
        <div class="grid grid-cols-2 gap-2">
          <button 
            v-for="(preset, key) in presets" 
            :key="key"
            @click="settingsStore.accentColor = preset.colors.primary"
            class="px-3 py-2 rounded-lg border transition-all text-left flex items-center gap-2"
            :class="settingsStore.accentColor === preset.colors.primary ? 'bg-white/10 border-primary text-white' : 'bg-white/5 border-white/10 text-text-secondary hover:bg-white/10'"
          >
            <div class="w-3 h-3 rounded-full" :style="{ backgroundColor: preset.colors.primary }"></div>
            <span class="text-xs font-medium">{{ preset.name }}</span>
          </button>
        </div>
      </div>

      <!-- 2. Glass Opacity -->
      <div>
        <div class="flex justify-between mb-2">
          <label class="text-xs text-text-secondary font-medium uppercase tracking-wider">Glass Opacity</label>
          <span class="text-xs text-white font-mono">{{ settingsStore.glassOpacity }}%</span>
        </div>
        <input 
          v-model.number="settingsStore.glassOpacity"
          type="range" 
          min="0" 
          max="100" 
          step="5"
          class="w-full accent-primary h-1.5 bg-white/10 rounded-lg appearance-none cursor-pointer"
        />
      </div>

      <!-- 3. Blur Strength -->
      <div>
        <label class="text-xs text-text-secondary block mb-2 font-medium uppercase tracking-wider">Blur Strength</label>
        <div class="grid grid-cols-4 gap-2">
          <button 
            v-for="opt in ['none', 'sm', 'md', 'xl']" 
            :key="opt"
            @click="settingsStore.blurStrength = opt"
            class="px-2 py-1.5 rounded text-xs border transition-colors capitalize"
            :class="settingsStore.blurStrength === opt ? 'bg-primary text-white border-primary' : 'bg-white/5 text-text-secondary border-white/10 hover:bg-white/10'"
          >
            {{ opt }}
          </button>
        </div>
      </div>

      <!-- 4. Border Radius -->
      <div>
        <label class="text-xs text-text-secondary block mb-2 font-medium uppercase tracking-wider">Corner Radius</label>
        <div class="grid grid-cols-4 gap-2">
          <button 
            v-for="opt in ['sm', 'md', 'xl', 'full']" 
            :key="opt"
            @click="settingsStore.borderRadius = opt"
            class="px-2 py-1.5 rounded text-xs border transition-colors capitalize"
            :class="settingsStore.borderRadius === opt ? 'bg-primary text-white border-primary' : 'bg-white/5 text-text-secondary border-white/10 hover:bg-white/10'"
          >
            {{ opt }}
          </button>
        </div>
      </div>

      <!-- 5. Font Scale -->
      <div>
        <div class="flex justify-between mb-2">
          <label class="text-xs text-text-secondary font-medium uppercase tracking-wider">Font Size</label>
          <span class="text-xs text-white font-mono">{{ settingsStore.fontSizeScale }}%</span>
        </div>
        <input 
          v-model.number="settingsStore.fontSizeScale"
          type="range" 
          min="80" 
          max="120" 
          step="5"
          class="w-full accent-primary h-1.5 bg-white/10 rounded-lg appearance-none cursor-pointer"
        />
      </div>

      <!-- 6. Animation Speed -->
      <div>
        <label class="text-xs text-text-secondary block mb-2 font-medium uppercase tracking-wider">Animation Speed</label>
        <div class="flex bg-white/5 rounded-lg p-1 border border-white/10">
          <button 
            v-for="opt in ['slow', 'normal', 'fast']" 
            :key="opt"
            @click="settingsStore.animationSpeed = opt"
            class="flex-1 py-1 rounded text-xs transition-colors capitalize"
            :class="settingsStore.animationSpeed === opt ? 'bg-white/10 text-white shadow-sm' : 'text-text-muted hover:text-white'"
          >
            {{ opt }}
          </button>
        </div>
      </div>

      <!-- 7. Mesh Gradient Toggle -->
      <div class="flex items-center justify-between">
        <label class="text-xs text-text-secondary font-medium uppercase tracking-wider">Mesh Background</label>
        <button 
          @click="settingsStore.showMeshGradient = !settingsStore.showMeshGradient"
          class="w-10 h-5 rounded-full relative transition-colors duration-200"
          :class="settingsStore.showMeshGradient ? 'bg-primary' : 'bg-white/20'"
        >
          <div 
            class="absolute top-1 left-1 w-3 h-3 bg-white rounded-full transition-transform duration-200"
            :class="settingsStore.showMeshGradient ? 'translate-x-5' : 'translate-x-0'"
          ></div>
        </button>
      </div>

      <!-- 8. Compact Mode -->
      <div class="flex items-center justify-between">
        <label class="text-xs text-text-secondary font-medium uppercase tracking-wider">Compact Mode</label>
        <button 
          @click="settingsStore.compactMode = !settingsStore.compactMode"
          class="w-10 h-5 rounded-full relative transition-colors duration-200"
          :class="settingsStore.compactMode ? 'bg-primary' : 'bg-white/20'"
        >
          <div 
            class="absolute top-1 left-1 w-3 h-3 bg-white rounded-full transition-transform duration-200"
            :class="settingsStore.compactMode ? 'translate-x-5' : 'translate-x-0'"
          ></div>
        </button>
      </div>

      <!-- 9. Sound Effects -->
      <div class="flex items-center justify-between">
        <label class="text-xs text-text-secondary font-medium uppercase tracking-wider">Sound Effects</label>
        <button 
          @click="settingsStore.soundEffects = !settingsStore.soundEffects"
          class="w-10 h-5 rounded-full relative transition-colors duration-200"
          :class="settingsStore.soundEffects ? 'bg-primary' : 'bg-white/20'"
        >
          <div 
            class="absolute top-1 left-1 w-3 h-3 bg-white rounded-full transition-transform duration-200"
            :class="settingsStore.soundEffects ? 'translate-x-5' : 'translate-x-0'"
          ></div>
        </button>
      </div>

      <!-- 10. Haptic Feedback -->
      <div class="flex items-center justify-between">
        <label class="text-xs text-text-secondary font-medium uppercase tracking-wider">Haptic Feedback</label>
        <button 
          @click="settingsStore.hapticFeedback = !settingsStore.hapticFeedback"
          class="w-10 h-5 rounded-full relative transition-colors duration-200"
          :class="settingsStore.hapticFeedback ? 'bg-primary' : 'bg-white/20'"
        >
          <div 
            class="absolute top-1 left-1 w-3 h-3 bg-white rounded-full transition-transform duration-200"
            :class="settingsStore.hapticFeedback ? 'translate-x-5' : 'translate-x-0'"
          ></div>
        </button>
      </div>

    </div>
  </div>
</template>

<style scoped>
/* Custom Range Slider Styling */
input[type=range]::-webkit-slider-thumb {
  -webkit-appearance: none;
  height: 16px;
  width: 16px;
  border-radius: 50%;
  background: #ffffff;
  cursor: pointer;
  margin-top: -5px; /* You need to specify a margin in Chrome, but in Firefox and IE it is automatic */
  box-shadow: 0 2px 6px rgba(0,0,0,0.3);
}
input[type=range]::-webkit-slider-runnable-track {
  width: 100%;
  height: 6px;
  cursor: pointer;
  background: rgba(255,255,255,0.1);
  border-radius: 3px;
}
</style>
