<script lang="ts">
	import type { ContentItem } from '$lib/types';

	import Container from '$lib/components/Container.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
	import { LANG_COLORS } from '$lib/project-language';
	import { collectionJsonLd, serializeJsonLd } from '$lib/seo';
	import { absoluteUrl, buildPageTitle } from '$lib/site';

	export let data: { posts: ContentItem[]; languages: Record<string, string | null> };

	type ProjectRow = {
		project: ContentItem;
		year: string;
		language: string | null;
		languageLabel: string | null;
		color: string;
		githubLink: string | undefined;
		summary: string;
	};

	const title = buildPageTitle('Projects');
	const description =
		'Software projects by Smit Patil, including selected tools, plugins, mods, and source code links.';
	const canonicalUrl = absoluteUrl('/projects');

	const getGitHubLink = (links: string[]) => links.find((link) => link.includes('github.com'));
	const fallbackColor = '#777777';

	const normalizeSummary = (value: string) => value.replace(/\s+/g, ' ').trim();

	let hoveredSlug: string | null = null;

	$: sortedProjects = [...data.posts].sort((a, b) =>
		b.frontmatter.date.localeCompare(a.frontmatter.date)
	);

	$: rows = sortedProjects.map<ProjectRow>((project) => {
		const language = data.languages[project.slug] ?? null;

		return {
			project,
			year: project.frontmatter.date.slice(0, 4),
			language,
			languageLabel: language,
			color: language ? (LANG_COLORS[language] ?? fallbackColor) : fallbackColor,
			githubLink: getGitHubLink(project.frontmatter.links),
			summary: normalizeSummary(project.frontmatter.description)
		};
	});

	$: structuredData = serializeJsonLd(
		collectionJsonLd({
			title: 'Projects',
			description,
			path: '/projects',
			items: sortedProjects.map((project) => ({
				name: project.frontmatter.title,
				path: `/projects#${project.slug}`
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
	<script type="application/ld+json">
{@html structuredData}
	</script>
</svelte:head>

<div class="projects-page">
	<Container>
		<Navbar />

		<section class="project-section" aria-labelledby="projects-title">
			<h1 id="projects-title">Projects</h1>
			<p>Code projects and contributions</p>

			<div class="section-rule"></div>

			<div class="project-list">
				{#each rows as row, index (row.project.slug)}
					{@const isHovered = hoveredSlug === row.project.slug}
					{@const shouldDim = hoveredSlug !== null && !isHovered}
					<article
						class:dimmed={shouldDim}
						class:hovered={isHovered}
						class="project-row"
						id={row.project.slug}
						style={`--language-color: ${row.color}; --row-index: ${index};`}
						on:mouseenter={() => (hoveredSlug = row.project.slug)}
						on:mouseleave={() => (hoveredSlug = null)}
						aria-labelledby={`${row.project.slug}-title`}
					>
						<div class="meta-column" aria-label={row.language ?? row.year}>
							<span class:fade-out={isHovered && row.languageLabel} class="year">{row.year}</span>
							{#if row.languageLabel}
								<span class:fade-in={isHovered} class="language">{row.languageLabel}</span>
							{/if}
						</div>

						<div class="title-column">
							<span class="language-rail" title={row.language ?? 'Unknown language'}></span>
							{#if row.githubLink}
								<a
									id={`${row.project.slug}-title`}
									href={row.githubLink}
									target="_blank"
									rel="noreferrer"
									class="project-title"
								>
									{row.project.frontmatter.title}
								</a>
							{:else}
								<h2 id={`${row.project.slug}-title`} class="project-title">
									{row.project.frontmatter.title}
								</h2>
							{/if}
						</div>

						<p class="summary">{row.summary}</p>
					</article>
				{/each}
			</div>
		</section>

		<div class="footer-wrap">
			<div class="section-rule"></div>
			<Footer />
		</div>
	</Container>
</div>

<style>
	.projects-page {
		min-height: 100vh;
		width: 100%;
		background: #000;
		color: #f2f2f2;
		font-family: 'IBM Plex Sans', Arial, system-ui, sans-serif;
	}

	.section-rule {
		height: 1px;
		width: 100%;
		background: #444;
	}

	.project-list {
		display: grid;
		gap: 0;
		padding: 1.15rem 0 4.8rem;
	}

	.project-row {
		--rail-width: 5px;
		display: grid;
		grid-template-columns: 4.4rem minmax(10rem, 15rem) minmax(0, 1fr);
		column-gap: 1rem;
		align-items: start;
		min-height: 2.5rem;
		color: #ededed;
		opacity: 0;
		transform: translateY(0.45rem);
		animation: row-in 0.42s ease forwards;
		animation-delay: calc(var(--row-index) * 55ms);
		transition:
			opacity 0.15s ease,
			color 0.15s ease;
	}

	.project-row.dimmed {
		opacity: 0.36 !important;
	}

	.meta-column {
		position: relative;
		min-height: 1.6rem;
		color: #b9b9b9;
		font-family: JetBrainsMono, ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
		font-size: 0.8rem;
		font-weight: 400;
		line-height: 1.25;
	}

	.year,
	.language {
		position: absolute;
		inset: 0 auto auto 0;
		transition:
			opacity 0.15s ease,
			color 0.15s ease;
	}

	.language {
		color: #e8e8e8;
		opacity: 0;
	}

	.fade-out {
		opacity: 0;
	}

	.fade-in {
		opacity: 1;
	}

	.title-column {
		display: grid;
		grid-template-columns: var(--rail-width) minmax(0, 1fr);
		column-gap: 0.78rem;
		min-height: 2.5rem;
	}

	.language-rail {
		display: block;
		width: var(--rail-width);
		min-height: 2.5rem;
		background: var(--language-color);
		opacity: 0.9;
		transition:
			opacity 0.15s ease,
			transform 0.15s ease;
	}

	.project-row.hovered .language-rail {
		opacity: 1;
		transform: scaleX(1.28);
	}

	.project-title {
		display: block;
		margin: 0;
		min-width: 0;
		color: #f5f5f5;

		font-weight: 700;
		line-height: 1.25;
		letter-spacing: 0.01em;
		text-decoration: none;
		overflow-wrap: anywhere;
		transition:
			color 0.15s ease,
			text-decoration-color 0.15s ease;
	}

	a.project-title:hover {
		color: #ffffff;
		text-decoration: underline;
		text-decoration-thickness: 1px;
		text-underline-offset: 0.16em;
	}

	.summary {
		display: -webkit-box;
		margin: 0;
		min-width: 0;
		overflow: hidden;
		color: #aaa;
		font-size: 0.8rem;
		font-weight: 400;
		line-height: 1.34;
		letter-spacing: 0.015em;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		-webkit-line-clamp: 2;
	}

	.project-row.hovered .summary {
		color: #c7c7c7;
	}

	@keyframes row-in {
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.project-row {
			opacity: 1;
			transform: none;
			animation: none;
		}

		.project-row,
		.language-rail,
		.year,
		.language,
		.project-title {
			transition: none;
		}
	}

	@media (max-width: 860px) {
		.project-list {
			gap: 1rem;
			padding: 1rem 0 3rem;
		}

		.project-row {
			grid-template-columns: 3.6rem minmax(0, 1fr);
			column-gap: 0.85rem;
			row-gap: 0.35rem;
			min-height: 0;
		}

		.meta-column {
			grid-row: 1 / span 2;
		}

		.title-column {
			grid-template-columns: 0.65rem minmax(0, 1fr);
			column-gap: 0.65rem;
			min-height: 0;
		}

		.language-rail {
			width: 0.55rem;
			height: 0.55rem;
			min-height: 0;
			margin-top: 0.28rem;
			border-radius: 999px;
		}

		.summary {
			grid-column: 2;
			line-clamp: 3;
			-webkit-line-clamp: 3;
		}
	}

	@media (max-width: 520px) {
		.project-row {
			grid-template-columns: 3rem minmax(0, 1fr);
		}
	}
</style>
