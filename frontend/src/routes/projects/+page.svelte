<script lang="ts">
	import type { ContentItem } from '$lib/types';

	import Container from '$lib/components/Container.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { formatDate } from '$lib/date';

	export let data: { posts: ContentItem[] };

	// Sort projects by date, newest first
	let sortedProjects = data.posts.sort((a, b) => {
		return b.frontmatter.date.localeCompare(a.frontmatter.date);
	});
</script>

<Container>
	<Navbar />
	<h1>Projects</h1>
	<p>
		You can find all of my projects on my Github,
		<a href="https://github.com/smit4k">@smit4k</a>. Some of my most interesting or complete
		projects will be put here, with articles explaining them in detail.
	</p>
	<hr />

	<div class="posts-list">
		{#each sortedProjects as project}
			<div class="post-item">
				<a href={`/ProjectPost/${project.slug}`} class="post-link">
					<h2>{project.frontmatter.title}</h2>
				</a>

				<p class="meta">
					Created {formatDate(project.frontmatter.date)} • {project.read_time} min read
				</p>
				<p class="tags">Tags: {project.frontmatter.tags.join(', ')}</p>
				<p class="description">{project.frontmatter.description}</p>
			</div>
			<hr />
		{/each}
	</div>

	<Footer />
</Container>

<style>
	h1 {
		margin-top: 0.5rem;
		margin-bottom: 0.5rem;
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
	a {
		text-decoration: none;
	}
</style>
