import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  server: {
    port: 80,
    host: '0.0.0.0',
  },
  define: {
    'process.env.VITE_SERVER_IP': JSON.stringify(process.env.VITE_SERVER_IP || 'backend-service'),
    'process.env.VITE_SERVER_PORT': JSON.stringify(process.env.VITE_SERVER_PORT || '8090')
  },
  plugins: [react()],
});
