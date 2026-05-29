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
	import { breadcrumbJsonLd, serializeJsonLd } from '$lib/seo';
	import { absoluteUrl, buildPageTitle, site } from '$lib/site';

	export let data: {
		post: ContentItem;
		viewCount: ViewCountResponse;
		htmlContent: string;
		headings: MarkdownHeading[];
	};

	const canonicalUrl = absoluteUrl(`/writing/${data.post.slug}`);
	const title = buildPageTitle(data.post.frontmatter.title);
	const description = data.post.frontmatter.description;
	const structuredData = serializeJsonLd([
		{
			'@context': 'https://schema.org',
			'@type': 'BlogPosting',
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
			{ name: 'Writing', path: '/writing' },
			{ name: data.post.frontmatter.title, path: `/writing/${data.post.slug}` }
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
				<a href="/writing">writing</a>
				<span class="separator">&gt;&gt;</span>
				<span class="current">{data.post.slug}</span>
			</nav>
			<h1>{data.post.frontmatter.title}</h1>
			<p class="desc">{data.post.frontmatter.description}</p>
			<p class="meta">
				Published {formatDate(data.post.frontmatter.date)} • {data.post.read_time} min read • {data
					.viewCount.total_views} views
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

	@media (max-width: 1150px) {
		.toc-slot {
			display: none;
		}
	}
</style>
