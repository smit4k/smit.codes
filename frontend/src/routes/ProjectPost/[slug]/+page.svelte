<script lang="ts">
	import type { ContentItem } from '$lib/types';
	import Container from '$lib/components/Container.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { formatDate } from '$lib/date';

	export let data: {
		post: ContentItem;
		htmlContent: string;
	};
</script>

<Container>
	<Navbar />
	<article>
		<h1>{data.post.frontmatter.title}</h1>
		<p class="meta">
			Published {formatDate(data.post.frontmatter.date)} • {data.post.read_time} min read
		</p>
		<p class="tags">
			Tags: {data.post.frontmatter.tags.join(', ')}
		</p>
		<hr />
		<div class="content">
			{@html data.htmlContent}
		</div>
	</article>
	<hr />
	<Footer />
</Container>

<style>
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
	.content {
		line-height: 1.6;
		font-size: 0.95rem;
	}
	hr {
		margin: 1rem 0;
	}
</style>
