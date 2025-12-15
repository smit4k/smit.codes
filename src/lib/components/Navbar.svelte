<script>
	// Optional: export props for active link or customize as needed
	// export let active = "/";
</script>

<header class="topbar">
	<h4>Smit Patil</h4>

	<nav>
		<a href="/">Home</a>
		<a href="/projects">Projects</a>
		<a href="/writing">Writing</a>
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

	h4 {
		margin: 0;
		color: white;
		font-weight: 600;
		font-size: 1.1rem;
	}

	nav {
		display: flex;
		gap: 1.2rem;
	}

	nav a {
		color: #bbb;
		text-decoration: none;
		font-size: 0.95rem;
		transition: color 0.3s ease;
	}

	nav a:hover {
		color: #58a6ff;
	}
</style>
