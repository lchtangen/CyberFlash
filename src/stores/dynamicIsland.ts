import { defineStore } from 'pinia';
import { ref } from 'vue';

export interface IslandActivity {
  id: string;
  type: 'info' | 'success' | 'warning' | 'error' | 'process' | 'ai' | 'download';
  title: string;
  subtitle?: string;
  icon: string;
  progress?: number; // 0-100
  color?: string; // Tailwind text color class
  bg?: string; // Tailwind bg color class
  border?: string; // Tailwind border color class
}

export const useDynamicIslandStore = defineStore('dynamicIsland', () => {
  const activeActivity = ref<IslandActivity | null>(null);

  // Set a high-priority activity (overrides current)
  const setActivity = (activity: IslandActivity) => {
    activeActivity.value = activity;
  };

  // Clear the current activity
  const clearActivity = () => {
    activeActivity.value = null;
  };

  // Temporary notification style activity
  const showNotification = (activity: IslandActivity, duration = 3000) => {
    const prev = activeActivity.value;
    activeActivity.value = activity;
    setTimeout(() => {
      if (activeActivity.value === activity) {
        activeActivity.value = prev;
      }
    }, duration);
  };

  const updateProgress = (progress: number) => {
    if (activeActivity.value && activeActivity.value.type === 'process') {
      activeActivity.value.progress = progress;
    }
  };

  return {
    activeActivity,
    setActivity,
    clearActivity,
    showNotification,
    updateProgress
  };
});
