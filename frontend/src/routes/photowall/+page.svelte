<script lang="ts">
	import Container from '$lib/components/Container.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
	import { formatDate } from '$lib/date';
	import { collectionJsonLd, serializeJsonLd } from '$lib/seo';
	import { absoluteUrl, buildPageTitle } from '$lib/site';
	import type { PhotoPost } from '$lib/types';
	import { Camera } from '@lucide/svelte';

	export let data: { posts: PhotoPost[] };

	const title = buildPageTitle('Photowall');
	const description =
		'Photo posts by Smit Patil featuring screenshots, experiments, and curated visual collections.';
	const canonicalUrl = absoluteUrl('/photowall');

	const ALL_TAG = 'All';

	let selectedTag = ALL_TAG;

	$: sortedPosts = [...data.posts].sort((a, b) => b.date.localeCompare(a.date));
	$: uniqueTags = [...new Set(sortedPosts.flatMap((post) => post.tags))].sort((a, b) =>
		a.localeCompare(b)
	);
	$: filteredPosts = sortedPosts.filter(
		(post) => selectedTag === ALL_TAG || post.tags.includes(selectedTag)
	);

	function previewTiles(post: PhotoPost) {
		const sourceImages = post.previewImages.length > 0 ? post.previewImages : [post.coverImage];
		const tiles = sourceImages.slice(0, 4);

		while (tiles.length < 4) {
			tiles.push(sourceImages[tiles.length % sourceImages.length] ?? post.coverImage);
		}

		return tiles;
	}

	$: structuredData = serializeJsonLd(
		collectionJsonLd({
			title: 'Photowall',
			description,
			path: '/photowall',
			items: sortedPosts.map((post) => ({
				name: post.title,
				path: `/photowall/${post.slug}`
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
	<h1>Photowall</h1>
	<p>
		A running wall of image posts. Browse collections by tag and open any post to view the full set
		of images.
	</p>
	<hr />

	<div class="toolbar">
		<div class="toolbar-copy">
			<p class="toolbar-label">Filter by tag</p>
			<p class="toolbar-meta">{filteredPosts.length} post{filteredPosts.length === 1 ? '' : 's'}</p>
		</div>

		<div class="filter-group" aria-label="Photo tags">
			<button
				type="button"
				class:selected={selectedTag === ALL_TAG}
				onclick={() => (selectedTag = ALL_TAG)}
			>
				All
			</button>

			{#each uniqueTags as tag}
				<button
					type="button"
					class:selected={selectedTag === tag}
					onclick={() => (selectedTag = tag)}
				>
					{tag}
				</button>
			{/each}
		</div>
	</div>

	{#if filteredPosts.length === 0}
		<div class="empty-state">
			<Camera size={20} />
			<div>
				<h2>No photo posts yet.</h2>
				<p>
					Add a manifest under `backend/content/photos` and the wall will populate automatically.
				</p>
			</div>
		</div>
	{:else}
		<div class="cards-grid">
			{#each filteredPosts as post}
				<a class="card" href={`/photowall/${post.slug}`} aria-label={`Open ${post.title}`}>
					<div class="preview-shell">
						<div class="preview-grid">
							{#each previewTiles(post) as imageSrc, index (`${post.slug}:${imageSrc}:${index}`)}
								<img
									src={imageSrc}
									alt={`${post.title} preview ${index + 1}`}
									loading="lazy"
									decoding="async"
									sizes="(max-width: 720px) 50vw, (max-width: 1100px) 25vw, 16vw"
								/>
							{/each}
						</div>
					</div>

					<div class="card-top">
						<p class="date">{formatDate(post.date)}</p>
						<h2>{post.title}</h2>
					</div>
					<div class="card-bottom">
						<span class="meta">{post.tags.join(', ')}</span>
					</div>
				</a>
			{/each}
		</div>
	{/if}

	<hr />
	<Footer />
</Container>

<style>
	h1 {
		margin-top: 0.5rem;
		margin-bottom: 0.5rem;
	}

	.toolbar {
		display: flex;
		flex-wrap: wrap;
		align-items: flex-end;
		justify-content: space-between;
		gap: 1rem;
		margin: 1rem 0;
	}

	.toolbar-copy {
		display: flex;
		flex-direction: column;
		gap: 0.2rem;
	}

	.toolbar-label,
	.toolbar-meta,
	.date {
		margin: 0;
	}

	.toolbar-label {
		color: #ccc;
		font-size: 0.92rem;
	}

	.toolbar-meta {
		color: #666;
		font-size: 0.82rem;
	}

	.filter-group {
		display: flex;
		flex-wrap: wrap;
		gap: 0.55rem;
	}

	.filter-group button {
		border: 1px solid #2d2d2d;
		background: rgba(12, 12, 12, 0.9);
		color: #d6d6d6;
	}

	.filter-group button {
		padding: 0.45rem 0.8rem;
		border-radius: 999px;
		font-size: 0.82rem;
		transition:
			border-color 0.18s ease,
			color 0.18s ease,
			background 0.18s ease;
	}

	.filter-group button.selected,
	.filter-group button:hover {
		background: #141414;
		border-color: #555;
		color: #fff;
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
		gap: 0.95rem;
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

	.preview-shell {
		position: relative;
		overflow: hidden;
		border-radius: 6px;
		aspect-ratio: 4 / 3;
		background: #090909;
		border: 1px solid #1f1f1f;
	}

	.preview-grid {
		display: grid;
		grid-template-columns: repeat(2, minmax(0, 1fr));
		grid-template-rows: repeat(2, minmax(0, 1fr));
		width: 100%;
		height: 100%;
		gap: 0.18rem;
	}

	.preview-grid img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		display: block;
		background: #050505;
	}

	h2 {
		margin: 0 0 0.35rem;
		color: #ffffff;
		font-size: 1rem;
		word-wrap: break-word;
		overflow-wrap: break-word;
		line-height: 1.35;
	}

	.card-top {
		display: flex;
		flex-direction: column;
	}

	.date {
		display: block;
		margin-bottom: 0.3rem;
		color: #7d7d7d;
		font-size: 0.78rem;
	}

	.card-bottom {
		display: flex;
		flex-direction: column;
		gap: 0.2rem;
	}

	.meta {
		font-size: 0.78rem;
		color: #555;
	}

	.empty-state {
		display: flex;
		align-items: flex-start;
		gap: 0.8rem;
		padding: 1.1rem;
		margin-bottom: 1rem;
		border: 1px dashed #2a2a2a;
		border-radius: 16px;
		color: #a0a0a0;
		background: #0b0b0b;
	}

	.empty-state h2 {
		margin: 0 0 0.2rem;
		font-size: 1rem;
		color: #f0f0f0;
	}

	.empty-state p {
		margin: 0;
		line-height: 1.45;
	}

	hr {
		margin: 1rem 0;
	}
</style>
