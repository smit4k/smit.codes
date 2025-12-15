<script>
	import Navbar from '$lib/components/Navbar.svelte';
	import Container from '$lib/components/Container.svelte';
</script>

<Container>
	<Navbar />
	<h1>Projects</h1>
	<p>
		You can find all of my projects on my Github, <a href="https://github.com/smit4k">@smit4k</a>.
		Some of my most interesting or complete projects will be put here, with articles explaining them
		in detail.
	</p>
	<hr />
</Container>

<style>
	:global(body) {
		background-color: black;
	}

	h1 {
		margin-top: 0.5rem;
		margin-bottom: 0.3rem;
		color: white;
	}

	p {
		font-family: Arial;
		color: #ddd;
	}

	hr {
		margin: 0.3rem 0;
		border-color: #444;
	}

	a {
		color: #58a6ff;
	}
</style>
