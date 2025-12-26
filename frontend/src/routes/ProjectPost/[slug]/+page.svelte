<script lang="ts">
	import type { ContentItem } from '$lib/types';
	import Container from '$lib/components/Container.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { formatDate } from '$lib/date';

	// Import icons
	import { Github, Link } from '@lucide/svelte';

	export let data: {
		post: ContentItem;
		htmlContent: string;
	};

	// Helper function to find GitHub link
	const getGitHubLink = (links: string[]) => links.find((link) => link.includes('github.com'));
</script>

<Container>
	<Navbar />
	<article>
		<div class="post-header">
			<h1>{data.post.frontmatter.title}</h1>

			<div class="icons">
				{#if getGitHubLink(data.post.frontmatter.links)}
					<a href={getGitHubLink(data.post.frontmatter.links)} target="_blank" aria-label="GitHub">
						<Github size="1.4em" />
					</a>
				{/if}

				{#each data.post.frontmatter.links as link (link)}
					{#if !link.includes('github.com')}
						<a href={link} target="_blank" aria-label="External Link">
							<Link size="1.4em" />
						</a>
					{/if}
				{/each}
			</div>
		</div>

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
		display: inline-block;
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
	}

	.icons a {
		margin-left: 0.5rem;
		color: #ccc;
		transition: color 0.2s;
	}

	.icons a:hover {
		color: white;
	}
</style>
