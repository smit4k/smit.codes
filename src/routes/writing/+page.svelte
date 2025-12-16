<script>
	import Container from '$lib/components/Container.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';
</script>

<Container>
	<Navbar />
	<h1>Writing</h1>
	<p>
		Here are some topics I've written about! These posts cover a variety of technology related
		topics, so feel free to take a look and see if you find anything interesting!
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
