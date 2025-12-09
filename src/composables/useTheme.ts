import { watch, onMounted } from 'vue';
import { useSettingsStore } from '../stores/settings';

export function useTheme() {
  const settingsStore = useSettingsStore();

  // Theme Presets Definition
  const presets: Record<string, any> = {
    'cyber-cyan': {
      name: 'Cyber Cyan',
      colors: {
        primary: '#00F0FF',
        primaryHover: '#40F4FF',
        primaryDim: 'rgba(0, 240, 255, 0.1)',
        secondary: '#2962FF',
        secondaryHover: '#448AFF',
        secondaryDim: 'rgba(41, 98, 255, 0.1)',
        accent: '#D500F9',
        accentHover: '#E040FB',
        background: '#020205',
        surface: '#0A0A0C',
        shadowPrimary: 'rgba(0, 240, 255, 0.4)',
        shadowSecondary: 'rgba(41, 98, 255, 0.4)',
        shadowAccent: 'rgba(213, 0, 249, 0.4)',
        glowPrimary: 'rgba(0, 240, 255, 0.6)',
        glowSecondary: 'rgba(41, 98, 255, 0.6)',
      }
    },
    'vivid-green': {
      name: 'Vivid Green',
      colors: {
        primary: '#00E676',
        primaryHover: '#69F0AE',
        primaryDim: 'rgba(0, 230, 118, 0.1)',
        secondary: '#00C853',
        secondaryHover: '#00E676',
        secondaryDim: 'rgba(0, 200, 83, 0.1)',
        accent: '#B9F6CA',
        accentHover: '#69F0AE',
        background: '#050505',
        surface: '#121212',
        shadowPrimary: 'rgba(0, 230, 118, 0.4)',
        shadowSecondary: 'rgba(0, 200, 83, 0.4)',
        shadowAccent: 'rgba(185, 246, 202, 0.4)',
        glowPrimary: 'rgba(0, 230, 118, 0.6)',
        glowSecondary: 'rgba(0, 200, 83, 0.6)',
      }
    },
    'minimalist': {
      name: 'Minimalist',
      colors: {
        primary: '#FFFFFF',
        primaryHover: '#E0E0E0',
        primaryDim: 'rgba(255, 255, 255, 0.1)',
        secondary: '#9E9E9E',
        secondaryHover: '#BDBDBD',
        secondaryDim: 'rgba(158, 158, 158, 0.1)',
        accent: '#757575',
        accentHover: '#9E9E9E',
        background: '#000000',
        surface: '#121212',
        shadowPrimary: 'rgba(255, 255, 255, 0.2)',
        shadowSecondary: 'rgba(158, 158, 158, 0.2)',
        shadowAccent: 'rgba(117, 117, 117, 0.2)',
        glowPrimary: 'rgba(255, 255, 255, 0.3)',
        glowSecondary: 'rgba(158, 158, 158, 0.3)',
      }
    },
    'electric-violet': {
      name: 'Electric Violet',
      colors: {
        primary: '#D500F9',
        primaryHover: '#E040FB',
        primaryDim: 'rgba(213, 0, 249, 0.1)',
        secondary: '#651FFF',
        secondaryHover: '#7C4DFF',
        secondaryDim: 'rgba(101, 31, 255, 0.1)',
        accent: '#00E5FF',
        accentHover: '#18FFFF',
        background: '#05000A',
        surface: '#120024',
        shadowPrimary: 'rgba(213, 0, 249, 0.4)',
        shadowSecondary: 'rgba(101, 31, 255, 0.4)',
        shadowAccent: 'rgba(0, 229, 255, 0.4)',
        glowPrimary: 'rgba(213, 0, 249, 0.6)',
        glowSecondary: 'rgba(101, 31, 255, 0.6)',
      }
    },
    'sunset-orange': {
      name: 'Sunset Orange',
      colors: {
        primary: '#FF9100',
        primaryHover: '#FFAB40',
        primaryDim: 'rgba(255, 145, 0, 0.1)',
        secondary: '#FF3D00',
        secondaryHover: '#FF6E40',
        secondaryDim: 'rgba(255, 61, 0, 0.1)',
        accent: '#FFD600',
        accentHover: '#FFFF00',
        background: '#0A0200',
        surface: '#140500',
        shadowPrimary: 'rgba(255, 145, 0, 0.4)',
        shadowSecondary: 'rgba(255, 61, 0, 0.4)',
        shadowAccent: 'rgba(255, 214, 0, 0.4)',
        glowPrimary: 'rgba(255, 145, 0, 0.6)',
        glowSecondary: 'rgba(255, 61, 0, 0.6)',
      }
    }
  };

  const applyTheme = () => {
    const root = document.documentElement;
    
    // Apply Preset Colors
    const preset = presets[settingsStore.themePreset] || presets['cyber-cyan'];
    const c = preset.colors;

    root.style.setProperty('--color-primary', c.primary);
    root.style.setProperty('--color-primary-hover', c.primaryHover);
    root.style.setProperty('--color-primary-dim', c.primaryDim);
    
    root.style.setProperty('--color-secondary', c.secondary);
    root.style.setProperty('--color-secondary-hover', c.secondaryHover);
    root.style.setProperty('--color-secondary-dim', c.secondaryDim);
    
    root.style.setProperty('--color-accent', c.accent);
    root.style.setProperty('--color-accent-hover', c.accentHover);
    
    root.style.setProperty('--color-background', c.background);
    root.style.setProperty('--color-surface', c.surface);
    
    root.style.setProperty('--shadow-primary', c.shadowPrimary);
    root.style.setProperty('--shadow-secondary', c.shadowSecondary);
    root.style.setProperty('--shadow-accent', c.shadowAccent);
    
    root.style.setProperty('--glow-primary', c.glowPrimary);
    root.style.setProperty('--glow-secondary', c.glowSecondary);

    // 1. Glass Opacity
    root.style.setProperty('--glass-opacity', (settingsStore.glassOpacity / 100).toString());
    
    // 2. Blur Strength
    const blurMap: Record<string, string> = {
      'none': '0px', 'sm': '4px', 'md': '12px', 'lg': '16px', 'xl': '24px', '2xl': '40px', '3xl': '64px'
    };
    root.style.setProperty('--blur-strength', blurMap[settingsStore.blurStrength] || '20px');
    
    // 3. Border Radius
    const radiusMap: Record<string, string> = {
      'sm': '0.25rem', 'md': '0.5rem', 'lg': '0.75rem', 'xl': '1rem', '2xl': '1.5rem', 'full': '9999px'
    };
    root.style.setProperty('--radius-panel', radiusMap[settingsStore.borderRadius] || '1rem');
    
    // 4. Font Scale
    root.style.setProperty('--font-scale', (settingsStore.fontSizeScale / 100).toString());
    
    // 5. Mesh Gradient Visibility
    root.style.setProperty('--mesh-opacity', settingsStore.showMeshGradient ? '1' : '0');
    
    // 7. Animation Speed
    const speedMap: Record<string, string> = { 'slow': '0.5s', 'normal': '0.3s', 'fast': '0.15s' };
    root.style.setProperty('--transition-speed', speedMap[settingsStore.animationSpeed] || '0.3s');
  };

  const initTheme = () => {
    applyTheme();
    
    watch(
      () => [
        settingsStore.themePreset,
        settingsStore.glassOpacity,
        settingsStore.blurStrength,
        settingsStore.borderRadius,
        settingsStore.fontSizeScale,
        settingsStore.showMeshGradient,
        settingsStore.animationSpeed
      ],
      applyTheme,
      { deep: true }
    );
  };

  return {
    presets,
    applyTheme,
    initTheme
  };
}
