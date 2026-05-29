import type { ContentItem } from '$lib/types';

export const load = async ({ fetch }) => {
	const [writingRes] = await Promise.all([fetch('/api/writing')]);

	const writing: ContentItem[] = await writingRes.json();

	const sortByDateDesc = (a: ContentItem, b: ContentItem) =>
		b.frontmatter.date.localeCompare(a.frontmatter.date);

	return {
		recentWriting: [...writing].sort(sortByDateDesc).slice(0, 5)
	};
};
