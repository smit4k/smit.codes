import type { PageLoad } from './$types';
import type { ContentItem } from '$lib/types';
import { PUBLIC_API_BASE_URL } from '$env/static/public';

export const load: PageLoad = async ({ fetch, params }) => {
    const res = await fetch(
        `${PUBLIC_API_BASE_URL}/api/posts/${params.slug}`
    );

    if (!res.ok) {
        throw new Error('Failed to fetch post');
    }

    const post: ContentItem = await res.json();

    return { post };
};


