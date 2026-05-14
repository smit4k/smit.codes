import type { ContentItem } from '$lib/types';
import { fetchGitHubLanguage } from '$lib/project-language';

export async function load({ fetch }) {
	const res = await fetch('/api/projects');
	if (!res.ok) {
		throw new Error('Failed to fetch posts');
	}
	const posts: ContentItem[] = await res.json();

	// Fetch primary language for each project with a GitHub link
	const languages: Record<string, string | null> = {};
	await Promise.all(
		posts.map(async (project) => {
			languages[project.slug] = await fetchGitHubLanguage(fetch, project.frontmatter.links);
		})
	);

	return { posts, languages };
}
