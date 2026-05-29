<script lang="ts">
	import Container from '$lib/components/Container.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import Callout from '$lib/components/Callout.svelte';
	import { serializeJsonLd } from '$lib/seo';
	import { absoluteUrl, buildPageTitle, site } from '$lib/site';
	import type { ContentItem } from '$lib/types';
	export let data: {
		recentProjects: ContentItem[];
		recentWriting: ContentItem[];
	};

	const title = buildPageTitle();
	const description =
		'Smit Patil is a student developer in Michigan publishing software projects, technical writing, and photo posts on smit.codes.';
	const canonicalUrl = absoluteUrl('/');
	const structuredData = serializeJsonLd([
		{
			'@context': 'https://schema.org',
			'@type': 'WebSite',
			name: site.name,
			url: canonicalUrl,
			description,
			author: {
				'@type': 'Person',
				name: site.personName
			}
		},
		{
			'@context': 'https://schema.org',
			'@type': 'Person',
			name: site.personName,
			url: canonicalUrl,
			email: site.email,
			sameAs: [site.githubUrl]
		}
	]);
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
	<script type="application/ld+json">
{@html structuredData}
	</script>
</svelte:head>

<Container>
	<Navbar />
	<h1>Home</h1>
	<hr />
	<p>
		Hey 👋, I'm Smit, an incoming university student from Michigan. I'm interested in software
		development, robotics, Minecraft modding, and Linux. I use this site to write technical posts,
		share what I'm learning, and showcase some of my projects. I mostly code in Rust and Java.
	</p>
	<h2>Contact</h2>
	<hr />

	<p>
		If you have any questions, comments, or want to chat, you can contact me through the following
		ways:
	</p>
	<ul>
		<li>Email: <a href="mailto:smit@smit.codes">smit@smit.codes</a></li>
		<li>Discord: <code>sm.it</code></li>
	</ul>
	<h2>Recent</h2>
	<hr />
	<p>Recent work from my projects and writing posts.</p>
	<h3>Projects</h3>
	<ul>
		{#each data.recentProjects as project}
			<li>
				<a href={`/projects/${project.slug}`}>
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
				<a href={`/writing/${post.slug}`}>
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
