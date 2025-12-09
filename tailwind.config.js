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
        // Vision UI Colors (Apple System Colors)
        background: '#000000', // Pure black for OLED/Depth
        surface: '#1C1C1E', // Apple Dark Gray
        'surface-light': '#2C2C2E',
        
        primary: {
          DEFAULT: 'rgb(var(--color-primary-rgb) / <alpha-value>)', // Dynamic Primary with Opacity Support
          hover: '#0077ED', // We might want to make this dynamic too, but for now static is safer
          dim: 'rgba(10, 132, 255, 0.1)',
        },
        secondary: {
          DEFAULT: '#5E5CE6', // Apple Indigo
          hover: '#4F4DCD',
          dim: 'rgba(94, 92, 230, 0.1)',
        },
        accent: {
          DEFAULT: '#BF5AF2', // Apple Purple
          hover: '#AF4BE2',
        },
        success: '#30D158', // Apple Green
        warning: '#FFD60A', // Apple Yellow
        error: '#FF453A', // Apple Red
        
        // Text Colors
        text: {
          primary: '#FFFFFF',
          secondary: '#EBEBF599', // Apple Label Secondary (60%)
          muted: '#EBEBF54D', // Apple Label Tertiary (30%)
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
        },
      },
      fontFamily: {
        sans: ['Inter', 'sans-serif'],
        mono: ['JetBrains Mono', 'monospace'],
      },
      boxShadow: {
        'neon-blue': '0 0 10px rgba(0, 240, 255, 0.5), 0 0 20px rgba(0, 240, 255, 0.3)',
        'neon-purple': '0 0 10px rgba(112, 0, 255, 0.5), 0 0 20px rgba(112, 0, 255, 0.3)',
      },
      keyframes: {
        shimmer: {
          '100%': { transform: 'translateX(100%)' },
        },
      },
      animation: {
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        'shimmer': 'shimmer 1.5s infinite',
      }
    },
  },
  plugins: [],
}
