import type { PageLoad } from './$types';
import type { ContentItem } from '$lib/types';
import { PUBLIC_API_BASE_URL } from '$env/static/public';

export const load: PageLoad = async ({ params, fetch }) => {
    const res = await fetch(`${PUBLIC_API_BASE_URL}/api/projects/${params.slug}`);
    if (!res.ok) {
        throw new Error('Post not found');
    }
    const post: ContentItem = await res.json();
    return { post };
};
