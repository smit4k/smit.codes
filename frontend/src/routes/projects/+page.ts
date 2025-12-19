import type { ContentItem } from '$lib/types';
import { PUBLIC_API_BASE_URL } from '$env/static/public';

export async function load({ fetch }) {
    const res = await fetch(`${PUBLIC_API_BASE_URL}/api/projects`);
    if (!res.ok) {
        throw new Error('Failed to fetch posts');
    }
    const posts: ContentItem[] = await res.json();
    return { posts };
}
