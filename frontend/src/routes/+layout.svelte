<script lang="ts">
	import favicon from '$lib/assets/favicon.ico';
	import '$lib/homepage.css';
	import { site } from '$lib/site';
	import { SvelteKitTopLoader } from 'sveltekit-top-loader';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<meta name="description" content={site.description} />
	<meta name="robots" content="index, follow, max-image-preview:large, max-snippet:-1, max-video-preview:-1" />
	<meta property="og:site_name" content={site.name} />
	<meta property="og:locale" content={site.locale} />
	<meta name="twitter:card" content="summary" />
	<meta name="theme-color" content="#080808" />
</svelte:head>

<SvelteKitTopLoader color="#58a6ff" showSpinner={false} />

{@render children()}
