import type { PageLoad } from './$types';
import type { ContentItem, ViewCountResponse } from '$lib/types';
import { error } from '@sveltejs/kit';
import { parseMarkdownWithShikiWithHeadings } from '$lib/markdown';
import { fetchGitHubLanguage } from '$lib/project-language';

export const load: PageLoad = async ({ params, fetch }) => {
	const url = `/api/projects/${params.slug}`;
	const viewCountUrl = `/api/projects/${params.slug}/views`;

	try {
		const res = await fetch(url);
		const viewRes = await fetch(viewCountUrl);

		if (!res.ok) {
			throw error(res.status, 'Project not found');
		}

		const post: ContentItem = await res.json();
		const viewCount: ViewCountResponse = await viewRes.json();
		const language = await fetchGitHubLanguage(fetch, post.frontmatter.links);
		const { html: htmlContent, headings } = await parseMarkdownWithShikiWithHeadings(post.markdown);

		fetch(`/api/projects/${params.slug}/view`, {
			method: 'POST'
		}).catch((err) => {
			console.error('Failed to record view: ', err);
		});

		return {
			post,
			viewCount,
			language,
			htmlContent,
			headings
		};
	} catch (e) {
		console.error('Fetch error:', e);
		throw error(500, `Failed to load project: ${e}`);
	}
};
