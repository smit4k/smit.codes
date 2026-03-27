<script lang="ts">
	import type { ContentItem, ViewCountResponse } from '$lib/types';
	import Container from '$lib/components/Container.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { formatDate } from '$lib/date';

	export let data: {
		post: ContentItem;
		viewCount: ViewCountResponse;
		htmlContent: string;
	};
</script>

<Container>
	<Navbar />
	<article>
		<nav class="breadcrumbs">
			<a href="/writing">writing</a>
			<span class="separator">&gt;&gt;</span>
			<span class="current">{data.post.slug}</span>
		</nav>
		<h1>{data.post.frontmatter.title}</h1>
		<p class="meta">
			Published {formatDate(data.post.frontmatter.date)} • {data.post.read_time} min read • {data
				.viewCount.total_views} views
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
	.breadcrumbs {
		font-size: 0.9rem;
		color: #888;
		margin-bottom: 0.75rem;
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
	.content {
		line-height: 1.6;
	}
	hr {
		margin: 1rem 0;
	}
</style>
