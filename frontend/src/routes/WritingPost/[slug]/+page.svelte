<script lang="ts">
	import type { ContentItem } from '$lib/types';
	import Container from '$lib/components/Container.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';

	import { marked } from 'marked';
	import { markedHighlight } from 'marked-highlight';
	import hljs from 'highlight.js';

	export let data: {
		post: ContentItem;
	};

	marked.use(
		markedHighlight({
			highlight(code: string, lang?: string) {
				if (lang && hljs.getLanguage(lang)) {
					return hljs.highlight(code, { language: lang }).value;
				}
				return hljs.highlightAuto(code).value;
			}
		})
	);

	const htmlContent = marked.parse(data.post.markdown);
</script>

<Container>
	<Navbar />

	<article>
		<h1>{data.post.frontmatter.title}</h1>

		<p class="meta">
			Published {data.post.frontmatter.date} • {data.post.read_time} min read
		</p>

		<p class="tags">
			Tags: {data.post.frontmatter.tags.join(', ')}
		</p>

		<hr />

		<div class="content">
			{@html htmlContent}
		</div>
	</article>
	<hr />
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
		line-height: 1.5;
		font-size: 0.95rem;
	}

	.content :global(h2) {
		color: white;
	}

	.content :global(pre) {
		background: #111;
		padding: 0.75rem;
		overflow-x: auto;
		border-radius: 6px;
	}

	hr {
		margin: 1rem 0;
	}
</style>
