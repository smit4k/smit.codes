import type { PageLoad } from './$types';
import type { ContentItem } from '$lib/types';

export const load: PageLoad = async () => {
    const res = await fetch('http://localhost:3001/api/posts');
    if (!res.ok) {
        console.error('Failed to fetch posts', res.status);
        return { posts: [] as ContentItem[] };
    }

    const posts: ContentItem[] = await res.json();
    return { posts };
};

