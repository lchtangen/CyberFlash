<script setup lang="ts">
import { ref } from 'vue';
import { useSettingsStore } from '../../../stores/settings';

const settingsStore = useSettingsStore();
const showCustomPicker = ref(false);

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

const setColor = (color: string) => {
  settingsStore.accentColor = color;
};
</script>

<template>
  <div class="bg-surface/30 border border-white/10 rounded-xl p-4 backdrop-blur-md">
    <div class="flex items-center justify-between mb-3">
      <h3 class="text-sm font-bold text-white flex items-center gap-2">
        <span class="material-symbols-rounded text-primary text-lg">palette</span>
        Accent Color
      </h3>
      <button 
        @click="showCustomPicker = !showCustomPicker"
        class="text-xs text-text-secondary hover:text-white transition-colors"
      >
        {{ showCustomPicker ? 'Presets' : 'Custom' }}
      </button>
    </div>

    <div v-if="!showCustomPicker" class="flex flex-wrap gap-3">
      <button
        v-for="color in colors"
        :key="color.name"
        @click="setColor(color.value)"
        class="w-8 h-8 rounded-full border border-white/10 hover:scale-110 transition-transform relative group flex items-center justify-center"
        :class="settingsStore.accentColor === color.value ? 'ring-2 ring-white ring-offset-2 ring-offset-black' : ''"
        :style="{ backgroundColor: color.value }"
      >
        <span 
          v-if="settingsStore.accentColor === color.value"
          class="material-symbols-rounded text-white text-sm drop-shadow-md"
        >check</span>
        
        <span class="absolute -bottom-8 left-1/2 -translate-x-1/2 text-[10px] text-white opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap bg-black/80 px-2 py-1 rounded pointer-events-none z-10">
          {{ color.name }}
        </span>
      </button>
    </div>

    <div v-else class="animate-in fade-in slide-in-from-top-2 duration-200">
      <div class="flex gap-2">
        <div class="relative flex-1">
          <input 
            v-model="settingsStore.accentColor"
            type="text" 
            class="w-full bg-black/20 border border-white/10 rounded-lg pl-10 pr-3 py-2 text-sm text-white font-mono focus:outline-none focus:border-primary/50"
            placeholder="#000000"
          />
          <div 
            class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 rounded-full border border-white/20"
            :style="{ backgroundColor: settingsStore.accentColor }"
          ></div>
        </div>
        <input 
          v-model="settingsStore.accentColor"
          type="color" 
          class="w-10 h-10 rounded-lg cursor-pointer bg-transparent border-0 p-0"
        />
      </div>
    </div>
  </div>
</template>
