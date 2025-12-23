import adapterStatic from '@sveltejs/adapter-static';
import adapterAuto from '@sveltejs/adapter-auto';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import { mdsvex } from 'mdsvex';
import { codeToHtml } from 'shiki';

// SvelteKit sets this automatically
const dev = process.env.NODE_ENV === 'development';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    extensions: ['.svelte', '.md', '.svx'],

    preprocess: [
        vitePreprocess(),
        mdsvex({
            extensions: ['.md', '.svx'],
            highlight: {
                highlighter: async (code, lang = 'text') => {
                    const html = await codeToHtml(code, {
                        lang,
                        theme: 'dark-plus'
                    });
                    return `{@html \`${html}\` }`;
                }
            }
        })
    ],

    kit: {
        adapter: dev
            ? adapterAuto()
            : adapterStatic({
                pages: 'build',
                assets: 'build',
                fallback: 'index.html',
                strict: false
            })
    }
};

export default config;

