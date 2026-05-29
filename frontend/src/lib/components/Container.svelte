<div class="container">
	<slot />
</div>

<style>
	.container {
		max-width: 70ch;
		width: 100%;
		margin: 0 auto;
		padding: 0 1rem;
		font-family: 'IBM Plex Sans', Arial, system-ui, sans-serif;
		word-wrap: break-word;
		overflow-wrap: break-word;
	}

	.container :global(.content img) {
		max-width: 100%;
		height: auto;
	}
</style>
