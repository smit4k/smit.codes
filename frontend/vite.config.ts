import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig(({ mode }) => {
    const dev = mode === 'development';

    return {
        plugins: [sveltekit()],

        build: {
            reportCompressedSize: false
        },

        server: dev
            ? {
                proxy: {
                    '/api': {
                        target: 'http://localhost:3001',
                        changeOrigin: true,
                        secure: false
                    },
                    '/assets': {
                        target: 'http://localhost:3001',
                        changeOrigin: true,
                        secure: false
                    }
                }
            }
            : undefined
    };
});
