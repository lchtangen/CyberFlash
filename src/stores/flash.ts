import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useFlashStore = defineStore('flash', () => {
  const currentPhase = ref(0)
  const isFlashing = ref(false)
  const logs = ref<string[]>([])

  function startFlash() {
    isFlashing.value = true
    logs.value.push('Starting flash process...')
  }

  function nextPhase() {
    currentPhase.value++
  }

  return {
    currentPhase,
    isFlashing,
    logs,
    startFlash,
    nextPhase
  }
})
