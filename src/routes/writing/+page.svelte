<script lang="ts">
	import type { ContentItem } from '$lib/types';
	import Container from '$lib/components/Container.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';
	export let data: { posts: ContentItem[] };
</script>

<Container>
	<Navbar />
	<h1>Writing</h1>
	<p>
		Here are some topics I've written about! These posts cover a variety of tech-related topics.
		Take a look and see if you find anything interesting!
	</p>
	<hr />

	<div class="posts-list">
		{#each data.posts as post}
			<div class="post-item">
				<h2>{post.frontmatter.title}</h2>
				<p class="meta">
					Published {post.frontmatter.date} • {post.read_time} min read
				</p>
				<p class="tags">Tags: {post.frontmatter.tags.join(', ')}</p>
				<p class="description">{post.frontmatter.description}</p>
			</div>
			<hr />
		{/each}
	</div>

	<Footer />
</Container>

<style>
	:global(body) {
		background-color: black;
		color: #ddd;
		font-family: Arial, sans-serif;
	}

	h1 {
		margin-top: 0.5rem;
		margin-bottom: 0.5rem;
		color: white;
	}

	.posts-list {
		margin-top: 1rem;
	}

	.post-item {
		margin-bottom: 1.5rem;
	}

	h2 {
		margin: 0;
		font-size: 1.2rem;
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
		margin-top: 0.35rem;
		font-size: 0.9rem;
	}
	hr {
		margin: 1rem 0;
	}
</style>
