import type { PhotoPost } from '$lib/types';
import { PUBLIC_API_BASE_URL } from '$env/static/public';
import { normalizePhotoPost } from '$lib/photos';

export async function load({ fetch }) {
	const res = await fetch(`${PUBLIC_API_BASE_URL}/api/photos`);

	if (!res.ok) {
		throw new Error('Failed to fetch photo posts');
	}

	const posts: PhotoPost[] = (await res.json()).map((post: PhotoPost) =>
		normalizePhotoPost(post, PUBLIC_API_BASE_URL)
	);
	return { posts };
}
