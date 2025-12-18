<script>
	import Container from '$lib/components/Container.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';
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
	<Footer />
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
</style>
