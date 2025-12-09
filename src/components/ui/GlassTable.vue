<script setup lang="ts">
defineProps<{
  headers: string[];
}>();
</script>

<template>
  <div class="w-full overflow-hidden rounded-xl border border-white/10 bg-surface/30 backdrop-blur-xl">
    <div class="overflow-x-auto custom-scrollbar">
      <table class="w-full text-left border-collapse">
        <thead>
          <tr class="border-b border-white/10 bg-white/5">
            <th 
              v-for="(header, index) in headers" 
              :key="index"
              class="px-6 py-4 text-xs font-bold text-text-secondary uppercase tracking-wider whitespace-nowrap"
            >
              {{ header }}
            </th>
          </tr>
        </thead>
        <tbody class="divide-y divide-white/5">
          <slot />
        </tbody>
      </table>
    </div>
  </div>
</template>

<style scoped>
.custom-scrollbar::-webkit-scrollbar {
  height: 6px;
}
.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}
.custom-scrollbar::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 3px;
}
.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.2);
}
</style>
