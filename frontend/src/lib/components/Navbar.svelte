<script>
	// Optional: export props for active link or customize as needed
	// export let active = "/";
</script>

<header class="topbar">
	<p class="brand"><a href="/">Smit Patil</a></p>

	<nav aria-label="Primary">
		<a href="/">Home</a>
		<a href="/projects">Projects</a>
		<a href="/writing">Writing</a>
		<a href="/photowall">Photowall</a>
		<a href="https://github.com/smit4k" rel="me">Github</a>
	</nav>
</header>

<style>
	.topbar {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-top: 2rem;
		margin-bottom: 1.5rem;
	}

	.brand {
		margin: 0;
		font-weight: 600;
		font-size: 1.2rem;
	}

	.brand a {
		color: white;
		text-decoration: none;
	}

	nav {
		display: flex;
		gap: 1.2rem;
	}

	nav a {
		color: #bbb;
		text-decoration: none;
		font-size: 1rem;
		transition: color 0.4s ease;
	}

	nav a:hover {
		color: #58a6ff;
	}
</style>
