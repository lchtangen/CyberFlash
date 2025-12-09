<script setup lang="ts">
import { onMounted, watch } from 'vue';
import { useSettingsStore } from '../../../stores/settings';
import { useNotificationStore } from '../../../stores/notifications';

const settingsStore = useSettingsStore();
const notificationStore = useNotificationStore();

// Local state for UI controls (mapped to store)
// We use the store directly in v-model for simplicity and reactivity

const applyTheme = () => {
  const root = document.documentElement;
  
  // 1. Glass Opacity
  root.style.setProperty('--glass-opacity', (settingsStore.glassOpacity / 100).toString());
  
  // 2. Blur Strength
  const blurMap: Record<string, string> = {
    'none': '0px', 'sm': '4px', 'md': '12px', 'lg': '16px', 'xl': '24px', '2xl': '40px', '3xl': '64px'
  };
  root.style.setProperty('--blur-strength', blurMap[settingsStore.blurStrength] || '20px');
  
  // 3. Border Radius
  const radiusMap: Record<string, string> = {
    'sm': '0.25rem', 'md': '0.5rem', 'lg': '0.75rem', 'xl': '1rem', '2xl': '1.5rem', 'full': '9999px'
  };
  root.style.setProperty('--radius-panel', radiusMap[settingsStore.borderRadius] || '1rem');
  
  // 4. Font Scale
  root.style.setProperty('--font-scale', (settingsStore.fontSizeScale / 100).toString());
  
  // 5. Mesh Gradient Visibility
  root.style.setProperty('--mesh-opacity', settingsStore.showMeshGradient ? '1' : '0');
  
  // 6. Accent Color (Primary)
  root.style.setProperty('--color-primary', settingsStore.accentColor);
  
  // 7. Animation Speed
  const speedMap: Record<string, string> = { 'slow': '0.5s', 'normal': '0.3s', 'fast': '0.15s' };
  root.style.setProperty('--transition-speed', speedMap[settingsStore.animationSpeed] || '0.3s');
};

const resetTheme = () => {
  settingsStore.$reset();
  notificationStore.addNotification({
    title: 'Theme Reset',
    message: 'Theme settings restored to default.',
    type: 'success'
  });
};

// Watch for changes in store and apply immediately
watch(
  () => [
    settingsStore.glassOpacity,
    settingsStore.blurStrength,
    settingsStore.borderRadius,
    settingsStore.fontSizeScale,
    settingsStore.showMeshGradient,
    settingsStore.accentColor,
    settingsStore.animationSpeed
  ],
  applyTheme,
  { deep: true }
);

onMounted(() => {
  applyTheme(); // Apply on load
});

const colors = [
  { name: 'Blue', value: '#0A84FF' },
  { name: 'Purple', value: '#BF5AF2' },
  { name: 'Pink', value: '#FF375F' },
  { name: 'Red', value: '#FF453A' },
  { name: 'Orange', value: '#FF9F0A' },
  { name: 'Yellow', value: '#FFD60A' },
  { name: 'Green', value: '#30D158' },
  { name: 'Teal', value: '#64D2FF' },
];
</script>

<template>
  <div class="bg-surface/30 border border-white/10 rounded-xl p-6 backdrop-blur-md">
    <div class="flex items-center gap-3 mb-6">
      <div class="w-10 h-10 rounded-full bg-purple-500/20 flex items-center justify-center text-purple-500">
        <span class="material-symbols-rounded">palette</span>
      </div>
      <div>
        <h3 class="text-lg font-bold text-white">Theme Engine</h3>
        <p class="text-xs text-text-secondary">Customize Appearance</p>
      </div>
    </div>

    <div class="space-y-6 max-h-[400px] overflow-y-auto custom-scrollbar pr-2">
      
      <!-- 1. Accent Color -->
      <div>
        <div class="flex justify-between items-center mb-2">
          <label class="text-xs text-text-secondary font-medium uppercase tracking-wider">Accent Color</label>
          <button @click="resetTheme" class="text-[10px] text-text-muted hover:text-white transition-colors">Reset Default</button>
        </div>
        <div class="flex flex-wrap gap-2">
          <button 
            v-for="color in colors" 
            :key="color.name"
            @click="settingsStore.accentColor = color.value"
            class="w-6 h-6 rounded-full border border-white/20 transition-transform hover:scale-110"
            :style="{ backgroundColor: color.value }"
            :class="{ 'ring-2 ring-white ring-offset-2 ring-offset-black': settingsStore.accentColor === color.value }"
          ></button>
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
