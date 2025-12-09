<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';

const canvas = ref<HTMLCanvasElement | null>(null);
let ctx: CanvasRenderingContext2D | null = null;
let animationFrame: number;

interface Node {
  x: number;
  y: number;
  vx: number;
  vy: number;
  radius: number;
}

const nodes: Node[] = [];
const connectionDistance = 100;

const initNodes = (width: number, height: number) => {
  nodes.length = 0;
  for (let i = 0; i < 30; i++) {
    nodes.push({
      x: Math.random() * width,
      y: Math.random() * height,
      vx: (Math.random() - 0.5) * 0.5,
      vy: (Math.random() - 0.5) * 0.5,
      radius: Math.random() * 2 + 1
    });
  }
};

const draw = () => {
  if (!canvas.value || !ctx) return;
  const width = canvas.value.width;
  const height = canvas.value.height;

  ctx.clearRect(0, 0, width, height);

  // Update and draw nodes
  nodes.forEach(node => {
    node.x += node.vx;
    node.y += node.vy;

    if (node.x < 0 || node.x > width) node.vx *= -1;
    if (node.y < 0 || node.y > height) node.vy *= -1;

    ctx!.beginPath();
    ctx!.arc(node.x, node.y, node.radius, 0, Math.PI * 2);
    ctx!.fillStyle = 'rgba(10, 132, 255, 0.8)';
    ctx!.fill();
  });

  // Draw connections
  for (let i = 0; i < nodes.length; i++) {
    for (let j = i + 1; j < nodes.length; j++) {
      const dx = nodes[i].x - nodes[j].x;
      const dy = nodes[i].y - nodes[j].y;
      const dist = Math.sqrt(dx * dx + dy * dy);

      if (dist < connectionDistance) {
        ctx.beginPath();
        ctx.moveTo(nodes[i].x, nodes[i].y);
        ctx.lineTo(nodes[j].x, nodes[j].y);
        ctx.strokeStyle = `rgba(10, 132, 255, ${1 - dist / connectionDistance})`;
        ctx.lineWidth = 0.5;
        ctx.stroke();
      }
    }
  }

  animationFrame = requestAnimationFrame(draw);
};

onMounted(() => {
  if (canvas.value) {
    ctx = canvas.value.getContext('2d');
    const rect = canvas.value.parentElement?.getBoundingClientRect();
    if (rect) {
      canvas.value.width = rect.width;
      canvas.value.height = rect.height;
      initNodes(rect.width, rect.height);
      draw();
    }
  }
});

onUnmounted(() => cancelAnimationFrame(animationFrame));
</script>

<template>
  <div class="w-full h-full bg-black/40 rounded-xl overflow-hidden border border-white/5 relative">
    <canvas ref="canvas" class="absolute inset-0"></canvas>
    <div class="absolute bottom-4 left-4 text-xs font-mono text-primary/80">
      ACTIVE_NODES: {{ nodes.length }}<br>
      NETWORK_LATENCY: 24ms
    </div>
  </div>
</template>
