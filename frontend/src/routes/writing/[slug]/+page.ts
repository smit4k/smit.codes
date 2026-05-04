import type { PageLoad } from './$types';
import type { ContentItem, ViewCountResponse } from '$lib/types';
import { error } from '@sveltejs/kit';
import { parseMarkdownWithShiki } from '$lib/markdown';

export const load: PageLoad = async ({ params, fetch }) => {
	const url = `/api/writing/${params.slug}`;
	const viewCountUrl = `/api/writing/${params.slug}/views`;

	try {
		const res = await fetch(url);
		const viewRes = await fetch(viewCountUrl);

		if (!res.ok) {
			throw error(res.status, 'Post not found');
		}

		const post: ContentItem = await res.json();
		const viewCount: ViewCountResponse = await viewRes.json();
		const htmlContent = await parseMarkdownWithShiki(post.markdown);

		fetch(`/api/writing/${params.slug}/view`, {
			method: 'POST'
		}).catch((err) => {
			console.error('Failed to record view: ', err);
		});

		return {
			post,
			viewCount,
			htmlContent
		};
	} catch (e) {
		console.error('Fetch error:', e);
		throw error(500, `Failed to load post: ${e}`);
	}
};
