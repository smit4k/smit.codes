<script>
	import Container from '$lib/components/Container.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';
</script>

<Container>
	<Navbar />
	<h1>Home</h1>
	<hr />
	<p>
		Hey! I’m Smit, welcome to my homepage,
		<a href="https://smit.codes">smit.codes</a>!
		<br />
		I'm currently a high school student in Michigan, interested in sftware development and UI/UX design.
	</p>

	<h2>Contact</h2>
	<hr />

	<p>If you have any questions or comments, you can contact me through:</p>
	<ul>
		<li>Email: <a href="mailto:smit@smit.codes">smit@smit.codes</a></li>
		<li>Discord: sm.it</li>
	</ul>
	<hr style="margin-bottom: 1rem" />
	<Footer />
</Container>

<style>
	:global(body) {
		background: black;
		color: #ddd;
		margin: 0;
	}

	/* Headings */
	h1,
	h2 {
		margin-top: 0.5rem;
		margin-bottom: 0.3rem;
		color: white;
	}

	/* HR styling */
	hr {
		margin: 0.3rem 0;
		border-color: #444;
	}

	a {
		color: #58a6ff;
	}
</style>
