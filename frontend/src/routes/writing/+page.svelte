<script lang="ts">
	import type { ContentItem } from '$lib/types';
	import Container from '$lib/components/Container.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { formatDate } from '$lib/date';
	import { collectionJsonLd, serializeJsonLd } from '$lib/seo';
	import { absoluteUrl, buildPageTitle } from '$lib/site';

	export let data: { posts: ContentItem[] };

	const title = buildPageTitle('Writing');
	const description =
		'Technical writing by Smit Patil covering software projects, configuration languages, tools, and programming experiments.';
	const canonicalUrl = absoluteUrl('/writing');
	const rssUrl = absoluteUrl('/writing/rss.xml');

	// Sort posts by date, newest first
	let sortedPosts = data.posts.sort((a, b) => {
		return b.frontmatter.date.localeCompare(a.frontmatter.date);
	});

	// Group posts by year
	let postsByYear: { year: string; posts: ContentItem[] }[] = [];
	for (const post of sortedPosts) {
		const year = post.frontmatter.date.slice(0, 4);
		const group = postsByYear.find((g) => g.year === year);
		if (group) {
			group.posts.push(post);
		} else {
			postsByYear.push({ year, posts: [post] });
		}
	}

	$: structuredData = serializeJsonLd(
		collectionJsonLd({
			title: 'Writing',
			description,
			path: '/writing',
			items: sortedPosts.map((post) => ({
				name: post.frontmatter.title,
				path: `/writing/${post.slug}`
			}))
		})
	);
</script>

<svelte:head>
	<title>{title}</title>
	<meta name="description" content={description} />
	<link rel="canonical" href={canonicalUrl} />
	<link rel="alternate" type="application/rss+xml" title="smit.codes Writing RSS" href={rssUrl} />
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
	<div class="heading-row">
		<h1>Writing</h1>
		<a class="rss-link" href="/writing/rss.xml" data-sveltekit-reload>RSS</a>
	</div>
	<p>
		Here are some topics I've written about! These posts cover a variety of tech-related topics.
		Take a look and see if you find anything interesting!
	</p>
	<hr />

	<div class="posts-list">
		{#each postsByYear as { year, posts }}
			<div class="year-divider">
				<span class="year-line"></span>
				<span class="year-label"><strong>{year}</strong></span>
				<span class="year-line"></span>
			</div>
			<div class="cards-grid">
				{#each posts as post}
					<a href={`/writing/${post.slug}`} class="card">
						<h2>{post.frontmatter.title}</h2>
						<p class="description">{post.frontmatter.description}</p>
						<span class="meta"
							>{formatDate(post.frontmatter.date)} • {post.read_time} min read • {post.frontmatter.tags.join(
								', '
							)}</span
						>
					</a>
				{/each}
			</div>
		{/each}
	</div>

	<hr />
	<Footer />
</Container>

<style>
	.heading-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-top: 0.5rem;
		margin-bottom: 0.5rem;
	}

	h1 {
		margin: 0;
		line-height: 1.1;
	}

	.rss-link {
		color: #888;
		font-size: 0.9rem;
		line-height: 1;
		text-decoration: none;
		transition: color 0.18s ease;
	}

	.rss-link:hover {
		color: #ccc;
	}

	.posts-list {
		margin-top: 1rem;
	}

	.cards-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 0.55rem;
		margin-bottom: 0.5rem;
	}

	.card {
		display: grid;
		gap: 0.22rem;
		padding: 0.62rem 0.78rem;
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

	h2 {
		margin: 0;
		font-size: 1rem;
		color: white;
		word-wrap: break-word;
		overflow-wrap: break-word;
		line-height: 1.35;
		min-width: 0;
	}

	.description {
		margin: 0;
		font-size: 0.85rem;
		color: #aaa;
		line-height: 1.35;
	}

	.meta {
		font-size: 0.78rem;
		color: #555;
	}

	hr {
		margin: 1rem 0;
	}

	.year-divider {
		display: flex;
		align-items: center;
		gap: 0.6rem;
		margin: 1.5rem 0 0.75rem;
		color: #aaa;
		font-size: 0.9rem;
	}
	.year-line {
		flex: 1;
		height: 1px;
		background-color: #444;
	}
	.year-label strong {
		color: #ccc;
	}
</style>
