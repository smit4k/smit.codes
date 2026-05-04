import type { ContentItem } from '$lib/types';

export async function load({ fetch }) {
    const res = await fetch('/api/writing');

    if (!res.ok) {
        throw new Error('Failed to fetch posts');
    }

    const posts: ContentItem[] = await res.json();
    return { posts };
}
