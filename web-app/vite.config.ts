import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    react(),
    wasm(),
    topLevelAwait()
  ],
  optimizeDeps: {
    exclude: ['drawconnect-core-wasm']
  },
  build: {
    target: 'esnext'
  },
  server: {
    port: 3000,
    cors: true,
    proxy: {
      '/downloads': {
        target: 'http://localhost:3001',
        changeOrigin: true
      },
      '/auth': {
        target: 'http://localhost:3001',
        changeOrigin: true
      },
      '/cloud': {
        target: 'http://localhost:3001',
        changeOrigin: true
      },
      '/plugins': {
        target: 'http://localhost:3001',
        changeOrigin: true
      },
      '/uploads': {
        target: 'http://localhost:3001',
        changeOrigin: true
      }
    }
  }
})
