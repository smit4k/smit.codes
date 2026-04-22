<script lang="ts">
	import Container from '$lib/components/Container.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
	import { formatDate } from '$lib/date';
	import type { PhotoImage, PhotoPost, ViewCountResponse } from '$lib/types';

	export let data: {
		post: PhotoPost;
		viewCount: ViewCountResponse;
	};

	function isWide(image: PhotoImage) {
		if (!image.width || !image.height) {
			return false;
		}

		return image.width / image.height > 1.25;
	}
</script>

<Container>
	<Navbar />

	<article>
		<nav class="breadcrumbs">
			<a href="/photowall">photowall</a>
			<span class="separator">&gt;&gt;</span>
			<span class="current">{data.post.slug}</span>
		</nav>

		<h1>{data.post.title}</h1>
		<p class="meta">
			Published {formatDate(data.post.date)} • {data.post.images.length} image{data.post.images.length === 1
				? ''
				: 's'} • {data.viewCount.total_views} views
		</p>
		<p class="tags">Tags: {data.post.tags.join(', ')}</p>
		{#if data.post.description}
			<p class="description">{data.post.description}</p>
		{/if}
		<hr />
		<section class="gallery">
			{#each data.post.images as image, index}
				<figure class:wide={isWide(image)}>
					<img
						src={image.src}
						alt={image.alt ?? `${data.post.title} photo ${index + 1}`}
						width={image.width}
						height={image.height}
						loading={index < 3 ? 'eager' : 'lazy'}
						decoding="async"
						sizes="(max-width: 720px) 100vw, (max-width: 1100px) 50vw, 33vw"
					/>
					{#if image.alt}
						<figcaption>{image.alt}</figcaption>
					{/if}
				</figure>
			{/each}
		</section>
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

	.description {
		margin: 0.75rem 0 0;
		color: #aaa;
		line-height: 1.6;
	}

	.gallery {
		margin-top: 1rem;
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
		gap: 0.9rem;
	}

	figure {
		margin: 0;
		padding: 0;
		background: none;
		border: none;
	}

	figure.wide {
		grid-column: span 2;
	}

	figure img {
		display: block;
		width: 100%;
		height: auto;
		border-radius: 0;
		background: none;
	}

	figcaption {
		margin-top: 0.45rem;
		color: #8d8d8d;
		font-size: 0.8rem;
		line-height: 1.4;
	}

	hr {
		margin: 1rem 0;
	}

	@media (max-width: 700px) {
		figure.wide {
			grid-column: span 1;
		}
	}
</style>
