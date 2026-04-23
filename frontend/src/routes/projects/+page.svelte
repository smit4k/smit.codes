<script lang="ts">
	import type { ContentItem } from '$lib/types';
	import { goto } from '$app/navigation';

	import Container from '$lib/components/Container.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { formatDate } from '$lib/date';
	import { collectionJsonLd, serializeJsonLd } from '$lib/seo';
	import { absoluteUrl, buildPageTitle } from '$lib/site';

	// Icons
	import { Github } from '@lucide/svelte';
	import { Link } from '@lucide/svelte';

	export let data: { posts: ContentItem[]; languages: Record<string, string | null> };

	const title = buildPageTitle('Projects');
	const description =
		'Software projects by Smit Patil, including build notes, implementation details, and external links.';
	const canonicalUrl = absoluteUrl('/projects');

	// GitHub linguist language colors (subset)
	const LANG_COLORS: Record<string, string> = {
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
		Vue: '#41b883',
	};

	// Sort projects by date, newest first
	let sortedProjects = data.posts.sort((a, b) => {
		return b.frontmatter.date.localeCompare(a.frontmatter.date);
	});

	// Helper function to find GitHub link
	const getGitHubLink = (links: string[]) => links.find((link) => link.includes('github.com'));

	function openProject(slug: string) {
		goto(`/projects/${slug}`);
	}

	function onCardKeydown(event: KeyboardEvent, slug: string) {
		if (event.key === 'Enter' || event.key === ' ') {
			event.preventDefault();
			openProject(slug);
		}
	}

	$: structuredData = serializeJsonLd(
		collectionJsonLd({
			title: 'Projects',
			description,
			path: '/projects',
			items: sortedProjects.map((project) => ({
				name: project.frontmatter.title,
				path: `/projects/${project.slug}`
			}))
		})
	);
</script>

<svelte:head>
	<title>{title}</title>
	<meta name="description" content={description} />
	<link rel="canonical" href={canonicalUrl} />
	<meta property="og:type" content="website" />
	<meta property="og:title" content={title} />
	<meta property="og:description" content={description} />
	<meta property="og:url" content={canonicalUrl} />
	<meta name="twitter:title" content={title} />
	<meta name="twitter:description" content={description} />
	<script type="application/ld+json">{@html structuredData}</script>
</svelte:head>

<Container>
	<Navbar />
	<h1>Projects</h1>
	<p>
		You can find all of my projects on my Github,
		<a href="https://github.com/smit4k">@smit4k</a>. Some of my most interesting or complete
		projects will be put here, with articles explaining them in detail.
	</p>
	<hr />

	<div class="posts-list">
		<div class="cards-grid">
			{#each sortedProjects as project}
					<div
						class="card"
						role="link"
						tabindex="0"
						aria-label={`Open ${project.frontmatter.title}`}
						onclick={() => openProject(project.slug)}
						onkeydown={(event) => onCardKeydown(event, project.slug)}
					>
						<div class="card-top">
							<div class="card-title-row">
								<span class="date">{formatDate(project.frontmatter.date)}</span>
								<div class="icons">
									{#if getGitHubLink(project.frontmatter.links)}
										<a
											href={getGitHubLink(project.frontmatter.links)}
											target="_blank"
											aria-label="GitHub"
											onclick={(e) => e.stopPropagation()}
										>
											<Github size="1em" />
										</a>
									{/if}
									{#each project.frontmatter.links as link (link)}
										{#if !link.includes('github.com')}
											<a
												href={link}
												target="_blank"
												aria-label="External Link"
												onclick={(e) => e.stopPropagation()}
											>
												<Link size="1em" />
											</a>
										{/if}
									{/each}
								</div>
							</div>
							<h2>{project.frontmatter.title}</h2>
							<p class="description">{project.frontmatter.description}</p>
						</div>
						<div class="card-bottom">
							<span class="meta"
								>{project.read_time} min read • {project.frontmatter.tags.join(', ')}</span
							>
							{#if data.languages[project.slug] && LANG_COLORS[data.languages[project.slug]!]}
								<span
									class="lang-dot"
									style="background-color: {LANG_COLORS[data.languages[project.slug]!]}"
									title={data.languages[project.slug]}
								></span>
							{/if}
						</div>
					</div>
			{/each}
		</div>
	</div>

	<hr />
	<Footer />
</Container>

<style>
	h1 {
		margin-top: 0.5rem;
		margin-bottom: 0.5rem;
	}

	.posts-list {
		margin-top: 1rem;
	}

	.cards-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
		gap: 0.85rem;
		margin-bottom: 0.5rem;
	}

	.card {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		gap: 0.75rem;
		padding: 1rem 1.1rem;
		background: #0e0e0e;
		border: 1px solid #2a2a2a;
		border-radius: 8px;
		text-decoration: none;
		color: inherit;
		transition:
			border-color 0.18s ease,
			background 0.18s ease;
	}

	.card:hover {
		border-color: #555;
		background: #141414;
	}

	.card-title-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: 0.3rem;
	}

	h2 {
		margin: 0 0 0.35rem;
		font-size: 1rem;
		color: white;
		word-wrap: break-word;
		overflow-wrap: break-word;
		line-height: 1.35;
	}

	.description {
		margin: 0;
		font-size: 0.85rem;
		color: #aaa;
		line-height: 1.45;
	}

	.card-bottom {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: space-between;
		gap: 0.5rem;
	}

	.lang-dot {
		width: 10px;
		height: 10px;
		border-radius: 50%;
		flex-shrink: 0;
	}

	.date {
		display: block;
		font-size: 0.78rem;
		color: #666;
	}

	.meta {
		font-size: 0.78rem;
		color: #555;
	}

	hr {
		margin: 1rem 0;
	}

	.icons {
		display: flex;
		gap: 0.4rem;
		flex-shrink: 0;
	}

	.icons a {
		color: #666;
		transition: color 0.2s;
		line-height: 0;
	}

	.icons a:hover {
		color: #ccc;
	}
</style>
