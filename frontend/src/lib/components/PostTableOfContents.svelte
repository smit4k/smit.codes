<script lang="ts">
	import { onMount, tick } from 'svelte';
	import type { MarkdownHeading } from '$lib/markdown';

	export let headings: MarkdownHeading[] = [];
	export let contentSelector = '.content';

	let activeId = headings[0]?.id ?? '';
	let progress = 0;

	const clamp = (value: number) => Math.min(1, Math.max(0, value));

	onMount(() => {
		let frame = 0;
		let headingElements: HTMLElement[] = [];
		let contentElement: HTMLElement | null = null;

		const collectElements = () => {
			contentElement = document.querySelector<HTMLElement>(contentSelector);
			headingElements = headings
				.map((heading) => document.getElementById(heading.id))
				.filter((element): element is HTMLElement => Boolean(element));
		};

		const update = () => {
			frame = 0;

			if (!contentElement) {
				collectElements();
			}

			if (!contentElement) return;

			const scrollY = window.scrollY;
			const contentTop = contentElement.getBoundingClientRect().top + scrollY;
			const readableDistance = contentElement.offsetHeight - window.innerHeight;
			progress = readableDistance <= 0 ? 1 : clamp((scrollY - contentTop) / readableDistance);

			if (contentElement.getBoundingClientRect().bottom <= window.innerHeight + 8) {
				activeId = headings.at(-1)?.id ?? '';
				return;
			}

			const activationLine = Math.min(window.innerHeight * 0.35, 180);
			const activeHeading = [...headingElements]
				.reverse()
				.find((heading) => heading.getBoundingClientRect().top <= activationLine);
			activeId = activeHeading?.id ?? headings[0]?.id ?? '';
		};

		const requestUpdate = () => {
			if (frame) return;
			frame = requestAnimationFrame(update);
		};

		tick().then(() => {
			collectElements();
			update();
		});

		window.addEventListener('scroll', requestUpdate, { passive: true });
		window.addEventListener('resize', requestUpdate);

		return () => {
			if (frame) cancelAnimationFrame(frame);
			window.removeEventListener('scroll', requestUpdate);
			window.removeEventListener('resize', requestUpdate);
		};
	});
</script>

{#if headings.length >= 2}
	<aside class="toc" aria-label="Table of contents">
		<div class="progress-track" aria-hidden="true">
			<div class="progress-fill" style={`height: ${progress * 100}%`}></div>
		</div>
		<nav class="toc-list">
			{#each headings as heading (heading.id)}
				<a
					class:active={heading.id === activeId}
					class:h3={heading.level === 3}
					href={`#${heading.id}`}
					aria-current={heading.id === activeId ? 'location' : undefined}
				>
					{heading.text}
				</a>
			{/each}
		</nav>
	</aside>
{/if}

<style>
	.toc {
		position: sticky;
		top: 1rem;
		display: grid;
		grid-template-columns: 3px minmax(0, 1fr);
		gap: 0.75rem;
		max-height: calc(100vh - 2rem);
		overflow: hidden auto;
		padding: 0.25rem 0 0.25rem 0.1rem;
		align-self: start;
	}

	.progress-track {
		position: relative;
		width: 3px;
		min-height: 100%;
		border-radius: 999px;
		background: #242424;
		overflow: hidden;
	}

	.progress-fill {
		position: absolute;
		inset: 0 0 auto;
		border-radius: inherit;
		background: #ffffff;
		transition: height 0.18s ease-out;
	}

	.toc-list {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
		min-width: 0;
	}

	a {
		display: block;
		border-radius: 4px;
		padding: 0.16rem 0.35rem;
		color: #898989;
		font-size: 0.78rem;
		line-height: 1.25;
		text-decoration: none;
		transition:
			color 0.16s ease,
			background 0.16s ease,
			transform 0.16s ease;
	}

	a:hover,
	a:focus-visible {
		color: #d6d6d6;
		background: rgba(255, 255, 255, 0.04);
	}

	a.active {
		color: #ffffff;
		background: rgba(255, 255, 255, 0.1);
		transform: translateX(2px);
	}

	a.h3 {
		margin-left: 0.65rem;
		font-size: 0.73rem;
		color: #747474;
	}

	a.h3.active {
		color: #e8f2ff;
	}

	@media (max-width: 900px) {
		.toc {
			display: none;
		}
	}

	@media (prefers-reduced-motion: reduce) {
		.progress-fill,
		a {
			transition: none;
		}

		a.active {
			transform: none;
		}
	}
</style>
