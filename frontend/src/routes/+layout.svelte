<script lang="ts">
	import favicon from '$lib/assets/favicon.ico';
	import '$lib/homepage.css';
	import { SvelteKitTopLoader } from 'sveltekit-top-loader';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<SvelteKitTopLoader color="#58a6ff" showSpinner={false} />

{@render children()}
