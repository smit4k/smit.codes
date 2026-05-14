export const LANG_COLORS: Record<string, string> = {
	TypeScript: '#3178c6',
	JavaScript: '#f1e05a',
	Svelte: '#ff3e00',
	Rust: '#dea584',
	Python: '#3572A5',
	Go: '#00ADD8',
	CSS: '#563d7c',
	HTML: '#e34c26',
	Shell: '#89e051',
	C: '#555555',
	'C++': '#f34b7d',
	'C#': '#178600',
	Java: '#b07219',
	Ruby: '#701516',
	PHP: '#4F5D95',
	Swift: '#F05138',
	Kotlin: '#A97BFF',
	Dart: '#00B4AB',
	Lua: '#000080',
	Haskell: '#5e5086',
	Elixir: '#6e4a7e',
	Nix: '#7e7eff',
	Vue: '#41b883'
};

export function parseGitHubRepo(url: string): { owner: string; repo: string } | null {
	const match = url.match(/github\.com\/([^/]+)\/([^/]+)/);
	if (!match) return null;
	return { owner: match[1], repo: match[2].replace(/\.git$/, '') };
}

export async function fetchGitHubLanguage(
	fetcher: typeof fetch,
	links: string[]
): Promise<string | null> {
	const ghLink = links.find((link) => link.includes('github.com'));
	if (!ghLink) return null;

	const parsed = parseGitHubRepo(ghLink);
	if (!parsed) return null;

	try {
		const response = await fetcher(`https://api.github.com/repos/${parsed.owner}/${parsed.repo}`, {
			headers: { Accept: 'application/vnd.github+json' }
		});

		if (!response.ok) return null;

		const data = await response.json();
		return data.language ?? null;
	} catch {
		return null;
	}
}
