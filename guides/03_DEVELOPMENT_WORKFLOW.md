# 🛠️ Development Workflow (Vue 3)

**Version**: 2.0.0 | **Updated**: December 7, 2025

---

## ☀️ Daily Routine

### 1. Start the Environment
```bash
# Terminal 1: Start the Development Server (Vue + Tauri)
npm run tauri dev

# Terminal 2: Run Tests in Watch Mode
npm run test:unit -- --watch
```

### 2. Code Quality Checks
Before committing, run the full suite:
```bash
npm run lint         # ESLint + Prettier
npm run typecheck    # TypeScript compiler check
npm run test:unit    # Vitest
```

---

## 🧪 Testing Strategy (Vitest)

We use **Vitest** for unit testing. It is compatible with Jest but much faster and native to Vite.

### Component Testing
```typescript
// tests/components/NeonButton.spec.ts
import { mount } from '@vue/test-utils'
import { describe, it, expect } from 'vitest'
import NeonButton from '@/components/ui/NeonButton.vue'

describe('NeonButton', () => {
  it('renders slot content', () => {
    const wrapper = mount(NeonButton, {
      slots: { default: 'Click Me' }
    })
    expect(wrapper.text()).toContain('Click Me')
  })

  it('shows loading state', () => {
    const wrapper = mount(NeonButton, {
      props: { loading: true }
    })
    expect(wrapper.text()).toContain('Processing...')
  })
})
```

### Store Testing (Pinia)
```typescript
// tests/stores/device.spec.ts
import { setActivePinia, createPinia } from 'pinia'
import { describe, it, expect, beforeEach } from 'vitest'
import { useDeviceStore } from '@/stores/device'

describe('Device Store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('sets device correctly', () => {
    const store = useDeviceStore()
    store.setDevice('OnePlus7Pro')
    expect(store.currentDevice).toBe('OnePlus7Pro')
    expect(store.isConnected).toBe(true)
  })
})
```

---

## 🌳 Git Workflow

We follow a strict branching model to ensure stability.

1.  **Main Branch (`main`)**: Production-ready code.
2.  **Develop Branch (`develop`)**: Integration branch.
3.  **Feature Branches**: `feature/feature-name` (e.g., `feature/gemini-integration`).

### Commit Convention
We use **Conventional Commits**:
- `feat: add Gemini AI chat component`
- `fix: resolve ADB connection timeout`
- `docs: update architecture guide`
- `style: refine glass blur effect`
- `refactor: migrate device store to setup syntax`

---

## 🐛 Debugging

### Vue DevTools
Install the **Vue.js devtools** extension in your browser (or standalone for Tauri). It allows you to:
- Inspect component hierarchy.
- View Pinia state changes in real-time.
- Track custom events.

### Rust Debugging
Use `println!` macros in Rust for quick logs, which appear in your terminal running `npm run tauri dev`.

```rust
println!("DEBUG: Device connected: {:?}", device_id);
```

For advanced debugging, use **LLDB** in VS Code with the "CodeLLDB" extension.
