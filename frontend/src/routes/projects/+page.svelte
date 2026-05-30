<script lang="ts">
	import type { ContentItem } from '$lib/types';

	import Container from '$lib/components/Container.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { formatDate } from '$lib/date';
	import { collectionJsonLd, serializeJsonLd } from '$lib/seo';
	import { absoluteUrl, buildPageTitle } from '$lib/site';
	import { Github } from '@lucide/svelte';
	import { Link } from '@lucide/svelte';

	export let data: { posts: ContentItem[]; languages: Record<string, string | null> };

	const title = buildPageTitle('Projects');
	const description =
		'Software projects by Smit Patil, including selected tools, plugins, mods, and source code links.';
	const canonicalUrl = absoluteUrl('/projects');

	let sortedProjects = [...data.posts].sort((a, b) => {
		return b.frontmatter.date.localeCompare(a.frontmatter.date);
	});

	const getGitHubLink = (links: string[]) => links.find((link) => link.includes('github.com'));
	const externalLinks = (links: string[]) => links.filter((link) => !link.includes('github.com'));

	function stripMarkdown(value: string) {
		return value
			.replace(/!\[[^\]]*\]\([^)]+\)/g, '')
			.replace(/\[([^\]]+)\]\([^)]+\)/g, '$1')
			.replace(/`([^`]+)`/g, '$1')
			.replace(/\*\*([^*]+)\*\*/g, '$1')
			.replace(/\*([^*]+)\*/g, '$1')
			.replace(/_{1,2}([^_]+)_{1,2}/g, '$1')
			.replace(/:::/g, '')
			.trim();
	}

	function getProjectDescription(project: ContentItem) {
		const paragraphs = project.markdown
			.split(/\n\s*\n/)
			.map((paragraph) => paragraph.trim())
			.filter((paragraph) => {
				return (
					paragraph &&
					!paragraph.startsWith('![') &&
					!paragraph.startsWith('#') &&
					!paragraph.startsWith('```') &&
					!paragraph.startsWith(':::') &&
					!paragraph.includes('img.shields.io')
				);
			})
			.map(stripMarkdown)
			.filter(Boolean);

		return paragraphs.slice(0, 2).length
			? paragraphs.slice(0, 2)
			: [project.frontmatter.description];
	}

	function getProjectMeta(project: ContentItem) {
		const language = data.languages[project.slug];
		return [formatDate(project.frontmatter.date), language].filter(Boolean).join(' · ');
	}

	function getLinkLabel(link: string) {
		if (link.includes('modrinth.com')) return 'Modrinth';
		try {
			return new URL(link).hostname.replace(/^www\./, '');
		} catch {
			return 'Link';
		}
	}

	$: structuredData = serializeJsonLd(
		collectionJsonLd({
			title: 'Projects',
			description,
			path: '/projects',
			items: sortedProjects.map((project) => ({
				name: project.frontmatter.title,
				path: `/projects#${project.slug}`
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
	<script type="application/ld+json">
{@html structuredData}
	</script>
</svelte:head>

<Container>
	<Navbar />
	<h1>Projects</h1>
	<p>
		A list of open-source projects I've authored that I feel like showcasing. All of my projects can
		be found on my Github, <a href="https://github.com/smit4k">@smit4k</a>
	</p>
	<hr />

	<div class="project-list">
		{#each sortedProjects as project}
			<section class="project" id={project.slug} aria-labelledby={`${project.slug}-title`}>
				<h2 id={`${project.slug}-title`}>
					{#if getGitHubLink(project.frontmatter.links)}
						<a href={getGitHubLink(project.frontmatter.links)} target="_blank" rel="noreferrer">
							{project.frontmatter.title}
						</a>
					{:else}
						{project.frontmatter.title}
					{/if}
				</h2>
				<p class="meta">{getProjectMeta(project)}</p>

				<div class="description">
					{#each getProjectDescription(project) as paragraph}
						<p>{paragraph}</p>
					{/each}
				</div>

				<div class="links" aria-label={`${project.frontmatter.title} links`}>
					{#if getGitHubLink(project.frontmatter.links)}
						<a href={getGitHubLink(project.frontmatter.links)} target="_blank" rel="noreferrer">
							<Github size="1em" aria-hidden="true" />
							GitHub
						</a>
					{/if}

					{#each externalLinks(project.frontmatter.links) as link (link)}
						<a href={link} target="_blank" rel="noreferrer">
							<Link size="1em" aria-hidden="true" />
							{getLinkLabel(link)}
						</a>
					{/each}
				</div>
			</section>
		{/each}
	</div>

	<hr />
	<Footer />
</Container>

<style>
	h1 {
		margin-top: 0.5rem;
		margin-bottom: 0.5rem;
	}

	.project-list {
		display: grid;
		gap: 2.1rem;
		margin: 1.45rem 0 0.5rem;
	}

	h2 {
		margin: 0;
		font-size: 1.2rem;
		color: white;
		word-wrap: break-word;
		overflow-wrap: break-word;
		line-height: 1.25;
	}

	h2 a {
		color: inherit;
		text-decoration: none;
		transition: color 0.2s;
	}

	h2 a:hover {
		color: #8abfff;
	}

	.meta {
		margin: 0.22rem 0 0;
		color: #777;
		font-size: 0.82rem;
	}

	.description {
		margin-top: 0.85rem;
		color: #b9b9b9;
		line-height: 1.55;
	}

	.description p {
		margin: 0;
	}

	.description p + p {
		margin-top: 0.7rem;
	}

	.links {
		display: flex;
		flex-wrap: wrap;
		gap: 0.8rem;
		margin-top: 0.85rem;
	}

	.links a {
		display: inline-flex;
		align-items: center;
		gap: 0.32rem;
		color: #8abfff;
		font-size: 0.9rem;
		text-decoration: none;
		transition: color 0.2s;
	}

	.links a:hover {
		color: #ccc;
	}

	.project + .project {
		padding-top: 2.1rem;
		border-top: 1px solid #2a2a2a;
	}

	hr {
		margin: 1rem 0;
	}
</style>
