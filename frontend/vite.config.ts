import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [sveltekit()],
  build: {
    reportCompressedSize: false // disables gzip/brotli size calculation
  }
});

