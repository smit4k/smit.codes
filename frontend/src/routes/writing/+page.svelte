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
	let sortedPosts = [...data.posts].sort((a, b) => {
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
	<script type="application/ld+json">
{@html structuredData}
	</script>
</svelte:head>

<Container>
	<Navbar />
	<div class="heading-row">
		<h1>Writing</h1>
		<a class="rss-link" href="/writing/rss.xml" data-sveltekit-reload>RSS</a>
	</div>
	<p class="lede">Notes on software projects, developer tools, configuration, and experiments.</p>
	<hr />

	<div class="posts-list">
		{#each postsByYear as { year, posts }}
			<section class="year-section" aria-labelledby={`writing-${year}`}>
				<h2 id={`writing-${year}`} class="year-heading">{year}</h2>
				{#each posts as post}
					<article class="post-row" aria-labelledby={`${post.slug}-title`}>
						<time class="date" datetime={post.frontmatter.date}>{formatDate(post.frontmatter.date)}</time>
						<a href={`/writing/${post.slug}`} class="post-link">
							<span id={`${post.slug}-title`} class="post-title">{post.frontmatter.title}</span>
							<span class="description">{post.frontmatter.description}</span>
						</a>
						<div class="meta" aria-label={`Metadata for ${post.frontmatter.title}`}>
							<span>{post.read_time} min</span>
							<span>{post.frontmatter.tags.join(' / ')}</span>
						</div>
					</article>
				{/each}
			</section>
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

	.lede {
		max-width: 58ch;
		margin: 0.65rem 0 0.9rem;
		color: #b8b8b8;
		line-height: 1.45;
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
		display: grid;
		gap: 1.2rem;
		margin-top: 1.05rem;
	}

	.year-section {
		display: grid;
		grid-template-columns: 4.1rem minmax(0, 1fr);
		column-gap: 1rem;
		border-top: 1px solid #292929;
		padding-top: 0.8rem;
	}

	.year-heading {
		margin: 0.12rem 0 0;
		color: #9c9c9c;
		font-family: JetBrainsMono, ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
		font-size: 0.78rem;
		font-weight: 400;
		line-height: 1.35;
	}

	.post-row {
		grid-column: 2;
		display: grid;
		grid-template-columns: minmax(0, 1fr) auto;
		grid-template-areas:
			'date date'
			'content meta';
		column-gap: 1rem;
		row-gap: 0.16rem;
		padding: 0.62rem 0;
		border-bottom: 1px solid #1c1c1c;
	}

	.date {
		grid-area: date;
		color: #707070;
		font-family: JetBrainsMono, ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
		font-size: 0.74rem;
		line-height: 1.25;
	}

	.post-link {
		grid-area: content;
		display: grid;
		gap: 0.16rem;
		min-width: 0;
		color: inherit;
		text-decoration: none;
	}

	.post-link:hover .post-title {
		color: #fff;
		text-decoration: underline;
		text-decoration-thickness: 1px;
		text-underline-offset: 0.16em;
	}

	.post-title {
		margin: 0;
		font-size: 1rem;
		font-weight: 700;
		color: #eeeeee;
		word-wrap: break-word;
		overflow-wrap: break-word;
		line-height: 1.3;
		min-width: 0;
		transition: color 0.16s ease;
	}

	.description {
		margin: 0;
		font-size: 0.85rem;
		color: #a8a8a8;
		line-height: 1.38;
	}

	.meta {
		display: grid;
		gap: 0.16rem;
		grid-area: meta;
		justify-items: end;
		align-self: end;
		max-width: 14rem;
		padding-top: 0.1rem;
		font-size: 0.8rem;
		line-height: 1.35;
		color: #666;
		text-align: right;
	}

	hr {
		margin: 1rem 0;
	}

	@media (max-width: 680px) {
		.year-section {
			grid-template-columns: 1fr;
			gap: 0.45rem;
		}

		.year-heading {
			font-size: 0.82rem;
		}

		.post-row {
			grid-column: 1;
			grid-template-columns: 1fr;
			grid-template-areas:
				'date'
				'content'
				'meta';
			gap: 0.18rem;
		}

		.meta {
			justify-items: start;
			max-width: none;
			text-align: left;
		}
	}
</style>
