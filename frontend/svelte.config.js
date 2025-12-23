import adapter from '@sveltejs/adapter-auto';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import { mdsvex } from 'mdsvex';
import { codeToHtml } from 'shiki';

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
                        theme: 'github-dark'
                    });
                    return `{@html \`${html}\` }`;
                }
            }
        })
    ],

    kit: {
        adapter: adapter()
    }
};

export default config;
