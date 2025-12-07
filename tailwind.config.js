/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        // Core Brand Colors (Cyber/Neon)
        background: '#050505',
        surface: '#121212',
        'surface-light': '#1E1E1E',
        
        primary: {
          DEFAULT: '#00F0FF', // Cyan Neon
          hover: '#00D0DD',
          dim: 'rgba(0, 240, 255, 0.1)',
        },
        secondary: {
          DEFAULT: '#7000FF', // Purple Neon
          hover: '#6000DD',
          dim: 'rgba(112, 0, 255, 0.1)',
        },
        accent: {
          DEFAULT: '#FF003C', // Red Neon
          hover: '#DD0030',
        },
        success: '#00FF9D',
        warning: '#FFD600',
        error: '#FF003C',
        
        // Text Colors
        text: {
          primary: '#FFFFFF',
          secondary: '#A0A0A0',
          muted: '#505050',
        },
        
        // Legacy Vision Support (Mapped to new colors to prevent crashes)
        vision: {
          bg: '#050505',
          surface: '#121212',
          border: '#333333',
          highlight: '#444444',
          text: {
            primary: '#FFFFFF',
            secondary: '#A0A0A0',
          },
          accent: '#00F0FF',
          success: '#00FF9D',
          danger: '#FF003C',
        }
      },
      fontFamily: {
        sans: ['Inter', 'sans-serif'],
        mono: ['JetBrains Mono', 'monospace'],
      },
      boxShadow: {
        'neon-blue': '0 0 10px rgba(0, 240, 255, 0.5), 0 0 20px rgba(0, 240, 255, 0.3)',
        'neon-purple': '0 0 10px rgba(112, 0, 255, 0.5), 0 0 20px rgba(112, 0, 255, 0.3)',
      },
      animation: {
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
      }
    },
  },
  plugins: [],
}
