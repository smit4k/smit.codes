import type { ContentItem } from '$lib/types';

export const load = async ({ fetch }) => {
    const [projectsRes, writingRes] = await Promise.all([
        fetch('/api/projects'),
        fetch('/api/writing')
    ]);

    const projects: ContentItem[] = await projectsRes.json();
    const writing: ContentItem[] = await writingRes.json();

    const sortByDateDesc = (a: ContentItem, b: ContentItem) =>
        b.frontmatter.date.localeCompare(a.frontmatter.date);

    return {
        recentProjects: [...projects].sort(sortByDateDesc).slice(0, 3),
        recentWriting: [...writing].sort(sortByDateDesc).slice(0, 3),
    };
};

