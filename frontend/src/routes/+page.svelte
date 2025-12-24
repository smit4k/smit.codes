<script lang="ts">
	import Container from '$lib/components/Container.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import type { ContentItem } from '$lib/types';
	export let data: {
		recentProjects: ContentItem[];
		recentWriting: ContentItem[];
	};
</script>

<Container>
	<Navbar />
	<h1>Home</h1>
	<hr />
	<p>
		Hey! I’m Smit, welcome to my homepage,
		<a href="https://smit.codes">smit.codes</a>! I'm a high school student in Michigan, interested
		in software development, Linux, and machining. I enjoy coding in Rust and Lua.
	</p>

	<h2>Contact</h2>
	<hr />

	<p>If you have any questions or comments, you can contact me through the following ways:</p>
	<ul>
		<li>Email: <a href="mailto:smit@smit.codes">smit@smit.codes</a></li>
		<li>Discord: <code>sm.it</code></li>
	</ul>
	<h2>Recent</h2>
	<hr />
	<p>Here are some links to recent projects I have worked on and articles I have written.</p>
	<h3>Projects</h3>
	<ul>
		{#each data.recentProjects as project}
			<li>
				<a href={`/ProjectPost/${project.slug}`}>
					{project.frontmatter.title}
				</a>
				— {project.frontmatter.description}
			</li>
		{/each}
	</ul>
	<h3>Writing</h3>
	<ul>
		{#each data.recentWriting as post}
			<li>
				<a href={`/WritingPost/${post.slug}`}>
					{post.frontmatter.title}
				</a>
				— {post.frontmatter.description}
			</li>
		{/each}
	</ul>
	<hr />
	<Footer />
</Container>

<style>
	h1,
	h2 {
		margin-top: 0.5rem;
		margin-bottom: 0.3rem;
		color: white;
	}
</style>
