<script lang="ts">
	import { onMount } from 'svelte';
	import type { ContentItem, ViewCountResponse } from '$lib/types';
	import type { MarkdownHeading } from '$lib/markdown';
	import Container from '$lib/components/Container.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import PostTableOfContents from '$lib/components/PostTableOfContents.svelte';
	import { installCodeBlockCopy } from '$lib/code-copy';
	import { formatDate } from '$lib/date';
	import { LANG_COLORS } from '$lib/project-language';
	import { breadcrumbJsonLd, serializeJsonLd } from '$lib/seo';
	import { absoluteUrl, buildPageTitle, site } from '$lib/site';

	import { Github, Link } from '@lucide/svelte';

	export let data: {
		post: ContentItem;
		viewCount: ViewCountResponse;
		language: string | null;
		htmlContent: string;
		headings: MarkdownHeading[];
	};

	const getGitHubLink = (links: string[]) => links.find((link) => link.includes('github.com'));
	const canonicalUrl = absoluteUrl(`/projects/${data.post.slug}`);
	const title = buildPageTitle(data.post.frontmatter.title);
	const description = data.post.frontmatter.description;
	const structuredData = serializeJsonLd([
		{
			'@context': 'https://schema.org',
			'@type': 'TechArticle',
			headline: data.post.frontmatter.title,
			description,
			url: canonicalUrl,
			datePublished: data.post.frontmatter.date,
			author: {
				'@type': 'Person',
				name: site.personName
			},
			keywords: data.post.frontmatter.tags.join(', '),
			mainEntityOfPage: canonicalUrl
		},
		breadcrumbJsonLd([
			{ name: 'Home', path: '/' },
			{ name: 'Projects', path: '/projects' },
			{ name: data.post.frontmatter.title, path: `/projects/${data.post.slug}` }
		])
	]);

	let contentElement: HTMLDivElement;

	onMount(() => installCodeBlockCopy(contentElement));
</script>

<svelte:head>
	<title>{title}</title>
	<meta name="description" content={description} />
	<link rel="canonical" href={canonicalUrl} />
	<meta property="og:type" content="article" />
	<meta property="og:title" content={title} />
	<meta property="og:description" content={description} />
	<meta property="og:url" content={canonicalUrl} />
	<meta property="article:published_time" content={data.post.frontmatter.date} />
	<meta name="twitter:title" content={title} />
	<meta name="twitter:description" content={description} />
	<script type="application/ld+json">
{@html structuredData}
	</script>
</svelte:head>

<div class="post-page">
	<Container>
		<Navbar />
	</Container>
	<div class={data.headings.length >= 2 ? 'post-shell has-toc' : 'post-shell'}>
		{#if data.headings.length >= 2}
			<div class="toc-slot">
				{#key data.post.slug}
					<PostTableOfContents headings={data.headings} />
				{/key}
			</div>
		{/if}
		<article>
			<nav class="breadcrumbs" aria-label="Breadcrumb">
				<a href="/projects">projects</a>
				<span class="separator">&gt;&gt;</span>
				<span class="current">{data.post.slug}</span>
			</nav>
			<div class="post-header">
				<h1>{data.post.frontmatter.title}</h1>

				<div class="icons">
					{#if getGitHubLink(data.post.frontmatter.links)}
						<a
							href={getGitHubLink(data.post.frontmatter.links)}
							target="_blank"
							rel="noreferrer"
							aria-label="GitHub"
						>
							<Github size="1.4em" />
						</a>
					{/if}

					{#each data.post.frontmatter.links as link (link)}
						{#if !link.includes('github.com')}
							<a href={link} target="_blank" rel="noreferrer" aria-label="External link">
								<Link size="1.4em" />
							</a>
						{/if}
					{/each}

					{#if data.language && LANG_COLORS[data.language]}
						<span
							class="language-pill"
							style="background-color: {LANG_COLORS[data.language]}"
							title={data.language}
							aria-label={`Primary language: ${data.language}`}
						>
							<span>{data.language}</span>
						</span>
					{/if}
				</div>
			</div>
			<p class="desc">{data.post.frontmatter.description}</p>
			<p class="meta">
				Created {formatDate(data.post.frontmatter.date)} • {data.post.read_time} min read • {data
					.viewCount.total_views}
				views
			</p>
			<p class="tags">Tags: {data.post.frontmatter.tags.join(', ')}</p>
			<hr />
			<div class="content" bind:this={contentElement}>
				{@html data.htmlContent}
			</div>
		</article>
	</div>
	<Container>
		<hr />
		<Footer />
	</Container>
</div>

<style>
	.post-page {
		width: 100%;
	}

	.post-shell {
		display: grid;
		grid-template-columns: minmax(0, 70ch);
		max-width: 70ch;
		width: 100%;
		margin: 0 auto;
		padding: 0 1rem;
		font-family: 'IBM Plex Sans', Arial, system-ui, sans-serif;
		word-wrap: break-word;
		overflow-wrap: break-word;
	}

	.post-shell.has-toc {
		position: relative;
	}

	.toc-slot {
		position: absolute;
		top: 0;
		right: calc(100% + 1.25rem);
		bottom: 0;
		width: 14rem;
	}

	article {
		grid-column: 1;
		min-width: 0;
	}

	.breadcrumbs {
		font-size: 0.9rem;
		color: #888;
		font-weight: bold;
	}

	.breadcrumbs a {
		color: #888;
		text-decoration: none;
		transition: color 0.2s;
	}

	.breadcrumbs a:hover {
		color: #ccc;
	}

	.breadcrumbs .separator {
		margin: 0 0.25rem;
		color: #666;
	}

	.breadcrumbs .current {
		color: #aaa;
	}

	h1 {
		margin-top: 0.5rem;
		margin-bottom: 0.25rem;
		color: white;
		display: inline-block;
		word-wrap: break-word;
		overflow-wrap: break-word;
	}

	.desc {
		margin: 0.2rem 0;
		color: #aaa;
		font-size: 0.9rem;
		font-style: italic;
	}

	.meta {
		margin: 0.2rem 0;
		color: #aaa;
		font-size: 0.9rem;
	}

	.tags {
		margin: 0;
		color: #888;
		font-size: 0.85rem;
	}

	.content {
		line-height: 1.6;
	}

	.content :global(img) {
		max-width: 100%;
		height: auto;
	}

	.content :global(h2[id]),
	.content :global(h3[id]) {
		scroll-margin-top: 1rem;
	}

	.content :global(.heading-anchor) {
		margin-left: 0.35rem;
		color: #777;
		font-weight: 400;
		text-decoration: none;
		opacity: 0;
		transition:
			color 0.2s,
			opacity 0.2s;
	}

	.content :global(h2[id]:hover .heading-anchor),
	.content :global(h3[id]:hover .heading-anchor) {
		opacity: 1;
	}

	.content :global(.heading-anchor:hover),
	.content :global(.heading-anchor:focus-visible) {
		color: #ccc;
		opacity: 1;
	}

	hr {
		margin: 1rem 0;
	}

	.post-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.icons {
		display: flex;
		align-items: center;
		flex-shrink: 0;
	}

	.language-pill {
		display: flex;
		align-items: center;
		justify-content: flex-end;
		margin-left: 0.5rem;
		width: 10px;
		height: 10px;
		max-width: 10px;
		border-radius: 50%;
		overflow: hidden;
		color: white;
		font-size: 0.68rem;
		font-weight: 700;
		line-height: 1;
		white-space: nowrap;
		cursor: default;
		transition:
			width 0.18s ease,
			max-width 0.18s ease,
			height 0.18s ease,
			border-radius 0.18s ease,
			padding 0.18s ease;
	}

	.language-pill span {
		opacity: 0;
		transform: translateX(0.25rem);
		transition:
			opacity 0.12s ease,
			transform 0.18s ease;
	}

	.language-pill:hover,
	.language-pill:focus-visible {
		width: auto;
		max-width: 8rem;
		height: 1.25rem;
		border-radius: 999px;
		padding: 0 0.45rem;
	}

	.language-pill:hover span,
	.language-pill:focus-visible span {
		opacity: 1;
		transform: translateX(0);
	}

	.icons a {
		margin-left: 0.5rem;
		color: #ccc;
		transition: color 0.2s;
	}

	.icons a:hover {
		color: white;
	}

	@media (max-width: 600px) {
		.post-header {
			flex-direction: column;
			align-items: flex-start;
		}

		.icons {
			margin-top: 0.5rem;
		}

		.icons a:first-child {
			margin-left: 0;
		}
	}

	@media (max-width: 1150px) {
		.toc-slot {
			display: none;
		}
	}
</style>
