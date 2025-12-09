<script setup lang="ts">
import GlassCard from '../components/ui/GlassCard.vue';
import NeuralTerminal from '../components/features/experimental/NeuralTerminal.vue';
import DeviceDigitalTwin from '../components/features/experimental/DeviceDigitalTwin.vue';
import BioAuthOverlay from '../components/features/experimental/BioAuthOverlay.vue';
import WaveformVisualizer from '../components/features/experimental/WaveformVisualizer.vue';
import NetworkConstellation from '../components/features/experimental/NetworkConstellation.vue';
import TelemetryHUD from '../components/features/experimental/TelemetryHUD.vue';
import QuantumFileDrop from '../components/features/experimental/QuantumFileDrop.vue';
import AutomationTimeline from '../components/features/experimental/AutomationTimeline.vue';
import RealityScanner from '../components/features/experimental/RealityScanner.vue';
import NanoProgress from '../components/features/experimental/NanoProgress.vue';
import HolographicButton from '../components/features/experimental/HolographicButton.vue';
</script>

<template>
  <div class="p-8 h-full overflow-y-auto custom-scrollbar space-y-8">
    
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-3xl font-bold text-white mb-2">Experimental Lab</h1>
        <p class="text-text-secondary">Prototyping Next-Gen Interfaces</p>
      </div>
      <div class="px-3 py-1 rounded-full bg-primary/20 border border-primary/50 text-primary text-xs font-mono">
        BETA_ACCESS_GRANTED
      </div>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      
      <!-- 1. Neural Terminal -->
      <GlassCard class="h-[300px] col-span-1 lg:col-span-2">
        <NeuralTerminal />
      </GlassCard>

      <!-- 2. Device Digital Twin -->
      <GlassCard class="h-[300px] flex items-center justify-center bg-gradient-to-b from-gray-900 to-black">
        <DeviceDigitalTwin />
      </GlassCard>

      <!-- 3. Bio Auth (Mini Preview) -->
      <GlassCard class="h-[300px] relative overflow-hidden">
        <BioAuthOverlay />
      </GlassCard>

      <!-- 4. Network Constellation -->
      <GlassCard class="h-[300px] col-span-1 lg:col-span-2">
        <NetworkConstellation />
      </GlassCard>

      <!-- 5. Telemetry HUD -->
      <GlassCard class="h-[300px] flex items-center justify-center">
        <TelemetryHUD />
      </GlassCard>

      <!-- 6. Reality Scanner -->
      <GlassCard class="h-[300px] p-0 overflow-hidden">
        <RealityScanner />
      </GlassCard>

      <!-- 7. Quantum File Drop -->
      <GlassCard class="h-[300px] flex items-center justify-center">
        <div class="w-full max-w-xs">
          <QuantumFileDrop />
        </div>
      </GlassCard>

      <!-- 8. Waveform Visualizer -->
      <GlassCard class="h-[300px] flex items-center justify-center">
        <div class="w-full h-32">
          <WaveformVisualizer />
        </div>
      </GlassCard>

      <!-- 9. Automation Timeline & Nano Progress -->
      <GlassCard class="h-[300px] flex flex-col justify-center gap-8">
        <div>
          <h3 class="text-xs font-bold text-white mb-4 uppercase tracking-wider">Sequence</h3>
          <AutomationTimeline />
        </div>
        <div>
          <h3 class="text-xs font-bold text-white mb-4 uppercase tracking-wider">Progress</h3>
          <NanoProgress :progress="67" eta="1m 23s" />
        </div>
      </GlassCard>

      <!-- 10. Holographic Button -->
      <GlassCard class="h-[300px] flex items-center justify-center bg-surface/50">
        <HolographicButton label="ENGAGE SYSTEM" icon="rocket_launch" />
      </GlassCard>

    </div>
  </div>
</template>
