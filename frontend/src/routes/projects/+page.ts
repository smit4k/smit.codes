import type { ContentItem } from '$lib/types';
import { PUBLIC_API_BASE_URL } from '$env/static/public';

function parseGitHubRepo(url: string): { owner: string; repo: string } | null {
	const match = url.match(/github\.com\/([^/]+)\/([^/]+)/);
	if (!match) return null;
	return { owner: match[1], repo: match[2].replace(/\.git$/, '') };
}

export async function load({ fetch }) {
    const res = await fetch(`${PUBLIC_API_BASE_URL}/api/projects`);
    if (!res.ok) {
        throw new Error('Failed to fetch posts');
    }
    const posts: ContentItem[] = await res.json();

    // Fetch primary language for each project with a GitHub link
    const languages: Record<string, string | null> = {};
    await Promise.all(
        posts.map(async (project) => {
            const ghLink = project.frontmatter.links.find((l: string) => l.includes('github.com'));
            if (!ghLink) return;
            const parsed = parseGitHubRepo(ghLink);
            if (!parsed) return;
            try {
                const r = await fetch(
                    `https://api.github.com/repos/${parsed.owner}/${parsed.repo}`,
                    { headers: { Accept: 'application/vnd.github+json' } }
                );
                if (r.ok) {
                    const data = await r.json();
                    languages[project.slug] = data.language ?? null;
                }
            } catch {
                // silently ignore — language dot just won't show
            }
        })
    );

    return { posts, languages };
}
