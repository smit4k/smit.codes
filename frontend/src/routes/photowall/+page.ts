import type { PhotoPost } from '$lib/types';
import { normalizePhotoPost } from '$lib/photos';

export async function load({ fetch }) {
	const res = await fetch('/api/photos');

	if (!res.ok) {
		throw new Error('Failed to fetch photo posts');
	}

	const posts: PhotoPost[] = (await res.json()).map((post: PhotoPost) =>
		normalizePhotoPost(post, '')
	);
	return { posts };
}
