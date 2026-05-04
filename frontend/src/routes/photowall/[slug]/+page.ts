import type { PageLoad } from './$types';
import type { PhotoPost, ViewCountResponse } from '$lib/types';
import { error } from '@sveltejs/kit';
import { normalizePhotoPost } from '$lib/photos';

export const load: PageLoad = async ({ params, fetch }) => {
	const url = `/api/photos/${params.slug}`;
	const viewCountUrl = `/api/photos/${params.slug}/views`;

	const res = await fetch(url);
	const viewRes = await fetch(viewCountUrl);

	if (res.status === 404) {
		throw error(404, 'Photo post not found');
	}

	if (!res.ok) {
		throw error(res.status, 'Failed to load photo post');
	}

	const post: PhotoPost = normalizePhotoPost(await res.json(), '');
	const viewCount: ViewCountResponse = viewRes.ok
		? await viewRes.json()
		: { total_views: 0, unique_views: 0 };

	fetch(`/api/photos/${params.slug}/view`, {
		method: 'POST'
	}).catch((err) => {
		console.error('Failed to record photo post view:', err);
	});

	return {
		post,
		viewCount
	};
};
