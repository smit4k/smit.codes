<script lang="ts">
	import { page } from '$app/stores';
	import Container from '$lib/components/Container.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';
</script>

<Container>
	<Navbar />
	<div class="error-page">
		<span class="code">{$page.status}</span>
		<p class="message">{$page.error?.message ?? 'Something went wrong'}</p>
		<a href="/">← back home</a>
	</div>
	<Footer />
</Container>

<style>
	.error-page {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		margin-top: 4rem;
	}

	.code {
		font-size: 4rem;
		font-weight: 700;
		color: white;
		line-height: 1;
	}

	.message {
		margin: 0;
		color: #bbb;
		font-size: 1.1rem;
	}
</style>
