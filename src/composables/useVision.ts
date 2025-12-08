import { ref, onMounted, onUnmounted, type Ref } from 'vue';

// 1. 3D Tilt Effect
export function useTilt(target: Ref<HTMLElement | null>, intensity = 15) {
  const transform = ref('');

  const handleMove = (e: MouseEvent) => {
    if (!target.value) return;
    const rect = target.value.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    
    const centerX = rect.width / 2;
    const centerY = rect.height / 2;
    
    const rotateX = ((y - centerY) / centerY) * -intensity;
    const rotateY = ((x - centerX) / centerX) * intensity;

    transform.value = `perspective(1000px) rotateX(${rotateX}deg) rotateY(${rotateY}deg)`;
  };

  const handleLeave = () => {
    transform.value = 'perspective(1000px) rotateX(0deg) rotateY(0deg)';
  };

  return { transform, handleMove, handleLeave };
}

// 2. Draggable Logic
export function useDraggable(target: Ref<HTMLElement | null>, handle: Ref<HTMLElement | null>) {
  const position = ref({ x: 100, y: 100 });
  const isDragging = ref(false);
  let startX = 0;
  let startY = 0;
  let initialX = 0;
  let initialY = 0;

  const onMouseDown = (e: MouseEvent) => {
    if (!target.value) return;
    isDragging.value = true;
    startX = e.clientX;
    startY = e.clientY;
    initialX = position.value.x;
    initialY = position.value.y;
    
    document.addEventListener('mousemove', onMouseMove);
    document.addEventListener('mouseup', onMouseUp);
  };

  const onMouseMove = (e: MouseEvent) => {
    if (!isDragging.value) return;
    const dx = e.clientX - startX;
    const dy = e.clientY - startY;
    position.value = { x: initialX + dx, y: initialY + dy };
  };

  const onMouseUp = () => {
    isDragging.value = false;
    document.removeEventListener('mousemove', onMouseMove);
    document.removeEventListener('mouseup', onMouseUp);
  };

  onMounted(() => {
    if (handle.value) {
      handle.value.addEventListener('mousedown', onMouseDown);
      handle.value.style.cursor = 'grab';
    }
  });

  onUnmounted(() => {
    if (handle.value) {
      handle.value.removeEventListener('mousedown', onMouseDown);
    }
  });

  return { position, isDragging };
}

// 3. Mouse Follow Glow
export function useGlow(target: Ref<HTMLElement | null>) {
  const glowStyle = ref({});

  const handleMove = (e: MouseEvent) => {
    if (!target.value) return;
    const rect = target.value.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    
    glowStyle.value = {
      background: `radial-gradient(circle at ${x}px ${y}px, rgba(255,255,255,0.15), transparent 150px)`
    };
  };

  return { glowStyle, handleMove };
}

// 4. Floating Animation (CSS Class helper)
export function useFloat(duration = '3s') {
  return {
    animation: `float ${duration} ease-in-out infinite`
  };
}

// 5. Magnetic Button Effect
export function useMagnetic(target: Ref<HTMLElement | null>, strength = 20) {
  const transform = ref('');

  const handleMove = (e: MouseEvent) => {
    if (!target.value) return;
    const rect = target.value.getBoundingClientRect();
    const centerX = rect.left + rect.width / 2;
    const centerY = rect.top + rect.height / 2;
    
    const dx = e.clientX - centerX;
    const dy = e.clientY - centerY;

    // Only apply if close
    if (Math.abs(dx) < 100 && Math.abs(dy) < 100) {
        transform.value = `translate(${dx / strength}px, ${dy / strength}px)`;
    } else {
        transform.value = '';
    }
  };

  const handleLeave = () => {
    transform.value = '';
  };

  return { transform, handleMove, handleLeave };
}

// 6. Sound Feedback
export function useSound() {
  const playClick = () => {
    // Placeholder for actual sound logic
    // const audio = new Audio('/sounds/click.mp3');
    // audio.play();
    console.log('Click sound');
  };
  
  const playHover = () => {
    console.log('Hover sound');
  };

  return { playClick, playHover };
}

// 7. Focus/Z-Index Management
const globalZIndex = ref(100);
export function useFocus() {
  const zIndex = ref(10);
  
  const bringToFront = () => {
    globalZIndex.value++;
    zIndex.value = globalZIndex.value;
  };

  return { zIndex, bringToFront };
}

// 8. Typewriter Effect
export function useTypewriter(text: string, speed = 50) {
  const displayedText = ref('');
  let index = 0;

  const start = () => {
    displayedText.value = '';
    index = 0;
    const interval = setInterval(() => {
      if (index < text.length) {
        displayedText.value += text.charAt(index);
        index++;
      } else {
        clearInterval(interval);
      }
    }, speed);
  };

  return { displayedText, start };
}

// 9. Resizable Logic
export function useResizable(target: Ref<HTMLElement | null>) {
  const size = ref({ width: 400, height: 300 });
  
  // Simplified resize logic would go here
  // For now, just returning the reactive state
  return { size };
}

// 10. Glass Morphism Generator
export function useGlass(opacity = 0.3, blur = 20) {
  return {
    background: `rgba(255, 255, 255, ${opacity})`,
    backdropFilter: `blur(${blur}px)`,
    border: '1px solid rgba(255, 255, 255, 0.1)',
    boxShadow: '0 8px 32px 0 rgba(0, 0, 0, 0.37)'
  };
}
