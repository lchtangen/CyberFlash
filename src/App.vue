<script setup lang="ts">
import { ref, onMounted } from 'vue';
import DashboardView from './views/DashboardView.vue';
import FlashView from './views/FlashView.vue';
import AIAssistantOverlay from './components/features/AIAssistantOverlay.vue';
import CommandPalette from './components/features/CommandPalette.vue';

const currentView = ref('dashboard');

onMounted(() => {
  document.documentElement.classList.add('dark');
});
</script>

<template>
  <div class="min-h-screen bg-background text-text-primary font-sans overflow-hidden relative flex">
    <!-- Sidebar -->
    <aside class="w-64 bg-surface border-r border-white/10 flex flex-col z-20">
      <div class="p-6 border-b border-white/10">
        <h1 class="text-xl font-bold tracking-wider text-primary neon-text">CYBERFLASH</h1>
        <p class="text-xs text-text-secondary mt-1">V2.0.1 // NEON</p>
      </div>
      
      <nav class="flex-1 p-4 space-y-2">
        <button 
          @click="currentView = 'dashboard'"
          class="w-full text-left px-4 py-3 rounded-lg transition-all duration-200 flex items-center gap-3"
          :class="currentView === 'dashboard' ? 'bg-primary/10 text-primary border border-primary/20' : 'text-text-secondary hover:bg-white/5 hover:text-white'"
        >
          <span class="material-symbols-rounded">dashboard</span>
          Dashboard
        </button>
        <button 
          @click="currentView = 'flash'"
          class="w-full text-left px-4 py-3 rounded-lg transition-all duration-200 flex items-center gap-3"
          :class="currentView === 'flash' ? 'bg-primary/10 text-primary border border-primary/20' : 'text-text-secondary hover:bg-white/5 hover:text-white'"
        >
          <span class="material-symbols-rounded">flash_on</span>
          Flash
        </button>
      </nav>
      
      <div class="p-4 border-t border-white/10">
        <div class="flex items-center gap-3">
          <div class="w-2 h-2 rounded-full bg-success animate-pulse"></div>
          <span class="text-xs text-text-secondary">System Online</span>
        </div>
      </div>
    </aside>

    <!-- Main Content -->
    <main class="flex-1 relative overflow-hidden flex flex-col bg-background">
      <!-- Top Bar -->
      <header class="h-16 border-b border-white/10 flex items-center justify-between px-6 bg-surface/50 backdrop-blur-md z-10">
        <div class="flex items-center gap-4">
          <h2 class="text-lg font-medium text-white capitalize">{{ currentView }}</h2>
        </div>
        <div class="flex items-center gap-4">
          <button class="p-2 rounded-full hover:bg-white/10 text-text-secondary hover:text-white transition-colors">
            <span class="material-symbols-rounded">notifications</span>
          </button>
          <button class="p-2 rounded-full hover:bg-white/10 text-text-secondary hover:text-white transition-colors">
            <span class="material-symbols-rounded">settings</span>
          </button>
        </div>
      </header>

      <!-- View Content -->
      <div class="flex-1 overflow-y-auto p-6">
        <DashboardView v-if="currentView === 'dashboard'" />
        <FlashView v-if="currentView === 'flash'" />
      </div>
    </main>

    <!-- Overlays -->
    <AIAssistantOverlay />
    <CommandPalette />
  </div>
</template>

<style>
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap');
@import url('https://fonts.googleapis.com/css2?family=Material+Symbols+Rounded:opsz,wght,FILL,GRAD@24,400,0,0');
</style>
