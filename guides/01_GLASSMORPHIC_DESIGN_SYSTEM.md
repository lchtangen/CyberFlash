# ✨ Glassmorphic Design System (Vue 3 Edition)

**Version**: 2.0.0 | **Framework**: Vue 3 + Tailwind CSS | **Status**: Production Ready

---

## 🎯 Design Philosophy

**CyberFlash V2** adopts a refined "Glassmorphism" aesthetic. It is **not** chaotic cyberpunk; it is sophisticated, transparent, and functional.

### Core Principles
1.  **Glass Surfaces**: High transparency (`opacity-70`), heavy blur (`backdrop-blur-xl`), and subtle white borders.
2.  **Neon Accents**: Used sparingly for focus states and active indicators (Cyan & Magenta).
3.  **Floating Depth**: Elements float on different z-indexes with soft, colored shadows.
4.  **Vue 3 Composition**: All components use `<script setup lang="ts">`.

---

## 🎨 Color Palette (Tailwind Config)

We use a semantic color system defined in `tailwind.config.js`.

| Token | Hex | Tailwind Class | Usage |
|-------|-----|----------------|-------|
| **Primary** | `#00F0FF` | `text-neon-cyan` | Main actions, active states, glow |
| **Secondary** | `#7000FF` | `text-neon-purple` | Gradients, secondary highlights |
| **Surface** | `#121212` | `bg-obsidian` | Deep background (behind glass) |
| **Glass** | `rgba(255,255,255,0.05)` | `bg-glass` | Card backgrounds |
| **Success** | `#00FF94` | `text-emerald-400` | Successful flash/connection |
| **Error** | `#FF0055` | `text-rose-500` | Errors, critical warnings |

---

## 💎 Glass Component Patterns (Vue 3)

### 1. The Base Glass Card

The fundamental building block. Note the `backdrop-blur` and `border`.

```vue
<!-- src/components/ui/GlassCard.vue -->
<script setup lang="ts">
defineProps<{
  hover?: boolean
}>()
</script>

<template>
  <div 
    class="relative overflow-hidden rounded-2xl border border-white/10 bg-white/5 backdrop-blur-xl transition-all duration-300"
    :class="{ 'hover:-translate-y-1 hover:shadow-[0_0_30px_rgba(0,240,255,0.15)] hover:border-white/20': hover }"
  >
    <!-- Noise Texture Overlay -->
    <div class="absolute inset-0 bg-noise opacity-[0.03] pointer-events-none"></div>
    
    <!-- Content -->
    <div class="relative z-10 p-6">
      <slot />
    </div>
  </div>
</template>

<style scoped>
.bg-noise {
  background-image: url('/assets/noise.png');
}
</style>
```

### 2. Neon Button

A button that glows when hovered.

```vue
<!-- src/components/ui/NeonButton.vue -->
<script setup lang="ts">
defineProps<{
  variant?: 'primary' | 'danger'
  loading?: boolean
}>()
</script>

<template>
  <button
    class="group relative px-6 py-3 rounded-lg font-jetbrains font-bold uppercase tracking-wider transition-all duration-300"
    :class="[
      variant === 'danger' ? 'text-rose-500 border-rose-500/50' : 'text-cyan-400 border-cyan-400/50',
      'border hover:bg-cyan-400/10 hover:shadow-[0_0_20px_rgba(0,240,255,0.4)]'
    ]"
    :disabled="loading"
  >
    <span v-if="loading" class="animate-pulse">Processing...</span>
    <slot v-else />
    
    <!-- Glitch Effect Element -->
    <div class="absolute inset-0 opacity-0 group-hover:opacity-100 transition-opacity duration-100 pointer-events-none mix-blend-overlay bg-gradient-to-r from-transparent via-white/10 to-transparent skew-x-12 translate-x-[-100%] group-hover:animate-shine"></div>
  </button>
</template>
```

### 3. Terminal Output (Glass)

```vue
<!-- src/components/features/Terminal.vue -->
<script setup lang="ts">
import { ref, onMounted, nextTick } from 'vue'

const logs = ref<string[]>([])
const terminalRef = ref<HTMLElement | null>(null)

const addLog = (msg: string) => {
  logs.value.push(msg)
  nextTick(() => {
    if (terminalRef.value) terminalRef.value.scrollTop = terminalRef.value.scrollHeight
  })
}
</script>

<template>
  <div class="font-mono text-sm rounded-xl bg-black/80 backdrop-blur-md border border-white/5 p-4 h-64 overflow-y-auto shadow-inner" ref="terminalRef">
    <div v-for="(log, i) in logs" :key="i" class="mb-1">
      <span class="text-gray-500">[{{ new Date().toLocaleTimeString() }}]</span>
      <span class="text-cyan-300 ml-2">></span>
      <span class="text-gray-300 ml-2">{{ log }}</span>
    </div>
  </div>
</template>
```

---

## 🎬 Animations (Vue Transition)

We use Vue's `<Transition>` component with CSS classes.

### Fade & Slide Up
Used for page transitions and modal appearances.

```css
/* styles/transitions.css */
.slide-up-enter-active,
.slide-up-leave-active {
  transition: all 0.4s cubic-bezier(0.16, 1, 0.3, 1);
}

.slide-up-enter-from,
.slide-up-leave-to {
  opacity: 0;
  transform: translateY(20px);
  filter: blur(10px);
}
```

### Usage
```vue
<Transition name="slide-up" mode="out-in">
  <component :is="activeView" />
</Transition>
```

---

## 🔤 Typography

**Font Family**: `JetBrains Mono` (Code & UI)
**Weights**: 
- Regular (400): Body text
- Bold (700): Headers & Buttons

```css
/* tailwind.config.js extension */
fontFamily: {
  sans: ['Inter', 'sans-serif'],
  mono: ['JetBrains Mono', 'monospace'],
}
```
