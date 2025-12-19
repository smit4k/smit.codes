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
		I'm currently a high school student in Michigan, interested in software development, Linux, and UI/UX
		design.
	</p>

	<h2>Contact</h2>
	<hr />

	<p>If you have any questions or comments, you can contact me through the following ways:</p>
	<ul>
		<li>Email: <a href="mailto:smit@smit.codes">smit@smit.codes</a></li>
		<li>Discord: <code>sm.it</code></li>
	</ul>
	<hr />
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
	code {
		font-size: 1.1em;
		border: 1px solid;
		border-radius: 5px;
		padding: 0 3px 0 3px;

		background: #050505;
		color: rgb(102, 204, 187);
	}
</style>
