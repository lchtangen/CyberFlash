<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useDraggable, useFocus, useGlass } from '../../composables/useVision';

const props = defineProps<{
  title: string;
  initialX?: number;
  initialY?: number;
}>();

const emit = defineEmits(['close']);

const windowRef = ref<HTMLElement | null>(null);
const headerRef = ref<HTMLElement | null>(null);

const { position, isDragging } = useDraggable(windowRef, headerRef);
const { zIndex, bringToFront } = useFocus();
const glassStyle = useGlass(0.15, 20);

onMounted(() => {
  if (props.initialX) position.value.x = props.initialX;
  if (props.initialY) position.value.y = props.initialY;
  bringToFront();
});
</script>

<template>
  <div 
    ref="windowRef"
    class="fixed rounded-xl overflow-hidden flex flex-col transition-shadow duration-200"
    :style="{
      left: position.x + 'px',
      top: position.y + 'px',
      zIndex: zIndex,
      width: '400px',
      ...glassStyle,
      boxShadow: isDragging ? '0 20px 50px rgba(0,0,0,0.5)' : '0 10px 30px rgba(0,0,0,0.3)'
    }"
    @mousedown="bringToFront"
  >
    <!-- Header / Drag Handle -->
    <div 
      ref="headerRef"
      class="h-10 bg-white/5 border-b border-white/10 flex items-center justify-between px-4 select-none active:cursor-grabbing"
    >
      <div class="flex items-center gap-2">
        <div class="w-3 h-3 rounded-full bg-red-500/80 hover:bg-red-500 cursor-pointer" @click="$emit('close')"></div>
        <div class="w-3 h-3 rounded-full bg-yellow-500/80"></div>
        <div class="w-3 h-3 rounded-full bg-green-500/80"></div>
      </div>
      <span class="text-xs font-medium text-white/70 tracking-wide">{{ title }}</span>
      <div class="w-8"></div> <!-- Spacer for centering -->
    </div>

    <!-- Content -->
    <div class="p-4 text-white relative">
      <slot />
    </div>
  </div>
</template>
