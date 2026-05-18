<script lang="ts">
	import { onMount, tick } from 'svelte';
	import type { MarkdownHeading } from '$lib/markdown';

	export let headings: MarkdownHeading[] = [];
	export let contentSelector = '.content';

	let activeId = headings[0]?.id ?? '';
	let progress = 0;
	let targetId = '';
	let isNavigatingToTarget = false;

	const clamp = (value: number) => Math.min(1, Math.max(0, value));

	const getHashId = () => {
		const hash = window.location.hash.slice(1);
		if (!hash) return '';

		try {
			return decodeURIComponent(hash);
		} catch {
			return hash;
		}
	};

	const handleHeadingClick = (id: string) => {
		targetId = id;
		isNavigatingToTarget = true;
		activeId = id;
	};

	onMount(() => {
		let frame = 0;
		let targetScrollY = 0;
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
			const viewportHeight = window.innerHeight;
			const activationLine = Math.min(viewportHeight * 0.35, 180);
			const maxScrollY = document.documentElement.scrollHeight - viewportHeight;
			progress = maxScrollY <= 0 ? 1 : clamp(scrollY / maxScrollY);

			if (!headingElements.length) {
				activeId = headings[0]?.id ?? '';
				return;
			}

			if (targetId && !isNavigatingToTarget && Math.abs(scrollY - targetScrollY) > 2) {
				targetId = '';
			}

			if (progress >= 0.999 && !targetId) {
				activeId = headings.at(-1)?.id ?? '';
				return;
			}

			let activeIndex = 0;
			for (const [index, heading] of headingElements.entries()) {
				if (heading.getBoundingClientRect().top <= activationLine) {
					activeIndex = index;
				} else {
					break;
				}
			}

			const targetIndex = headingElements.findIndex((heading) => heading.id === targetId);
			if (targetIndex !== -1) {
				const targetRect = headingElements[targetIndex].getBoundingClientRect();
				const targetIsVisible = targetRect.bottom >= 0 && targetRect.top <= viewportHeight - 16;

				if (targetIsVisible) {
					activeIndex = targetIndex;
					targetScrollY = scrollY;
					isNavigatingToTarget = false;
				} else {
					targetId = '';
					isNavigatingToTarget = false;
				}
			}

			activeId = headingElements[activeIndex]?.id ?? headings[0]?.id ?? '';
		};

		const requestUpdate = () => {
			if (frame) return;
			frame = requestAnimationFrame(update);
		};

		const handleHashChange = () => {
			targetId = getHashId();
			isNavigatingToTarget = true;
			requestUpdate();
		};

		const handleManualScroll = () => {
			if (!isNavigatingToTarget) {
				targetId = '';
			}
		};

		const finishTargetNavigation = () => {
			isNavigatingToTarget = false;
		};

		tick().then(() => {
			targetId = getHashId();
			isNavigatingToTarget = Boolean(targetId);
			collectElements();
			update();
		});

		window.addEventListener('scroll', requestUpdate, { passive: true });
		window.addEventListener('resize', requestUpdate);
		window.addEventListener('hashchange', handleHashChange);
		window.addEventListener('wheel', handleManualScroll, { passive: true });
		window.addEventListener('touchstart', handleManualScroll, { passive: true });
		window.addEventListener('keydown', handleManualScroll);
		window.addEventListener('scrollend', finishTargetNavigation);

		return () => {
			if (frame) cancelAnimationFrame(frame);
			window.removeEventListener('scroll', requestUpdate);
			window.removeEventListener('resize', requestUpdate);
			window.removeEventListener('hashchange', handleHashChange);
			window.removeEventListener('wheel', handleManualScroll);
			window.removeEventListener('touchstart', handleManualScroll);
			window.removeEventListener('keydown', handleManualScroll);
			window.removeEventListener('scrollend', finishTargetNavigation);
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
					on:click={() => handleHeadingClick(heading.id)}
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
