import type { PageLoad } from './$types';
import type { ContentItem } from '$lib/types';
import { PUBLIC_API_BASE_URL } from '$env/static/public';
import { error } from '@sveltejs/kit';
import { parseMarkdownWithShiki } from '$lib/markdown';

export const load: PageLoad = async ({ params, fetch }) => {
    const url = `${PUBLIC_API_BASE_URL}/api/writing/${params.slug}`;

    try {
        const res = await fetch(url);

        if (!res.ok) {
            throw error(res.status, 'Post not found');
        }

        const post: ContentItem = await res.json();
        const htmlContent = await parseMarkdownWithShiki(post.markdown);

        return {
            post,
            htmlContent
        };
    } catch (e) {
        console.error('Fetch error:', e);
        throw error(500, `Failed to load post: ${e}`);
    }
};
