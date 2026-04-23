<script lang="ts">
	import type { ContentItem, ViewCountResponse } from '$lib/types';
	import Container from '$lib/components/Container.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { formatDate } from '$lib/date';
	import { breadcrumbJsonLd, serializeJsonLd } from '$lib/seo';
	import { absoluteUrl, buildPageTitle, site } from '$lib/site';

	import { Github, Link } from '@lucide/svelte';

	export let data: {
		post: ContentItem;
		viewCount: ViewCountResponse;
		htmlContent: string;
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
	<script type="application/ld+json">{@html structuredData}</script>
</svelte:head>

<Container>
	<Navbar />
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
			</div>
		</div>

		<p class="meta">
			Created {formatDate(data.post.frontmatter.date)} • {data.post.read_time} min read • {data
				.viewCount.total_views}
			views
		</p>
		<p class="tags">Tags: {data.post.frontmatter.tags.join(', ')}</p>
		<hr />
		<div class="content">
			{@html data.htmlContent}
		</div>
	</article>
	<hr />
	<Footer />
</Container>

<style>
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
		flex-shrink: 0;
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
</style>
