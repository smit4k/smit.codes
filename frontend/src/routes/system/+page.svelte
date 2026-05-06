<script lang="ts">
	import { onMount } from 'svelte';

	import Container from '$lib/components/Container.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
	import { absoluteUrl, buildPageTitle } from '$lib/site';
	import type { SystemInfo, SystemMetrics } from './+page';

	export let data: {
		info: SystemInfo;
		metrics: SystemMetrics;
		loadedAt: string;
	};

	const title = buildPageTitle('System');
	const description = 'Live server CPU, memory, disk, uptime, and host stats for smit.codes.';
	const canonicalUrl = absoluteUrl('/system');

	let info = data.info;
	let metrics = data.metrics;
	let loadedAt = data.loadedAt;
	let refreshError = '';
	let refreshing = false;
	let pollTimer: ReturnType<typeof setInterval> | undefined;

	const byteFormatter = new Intl.NumberFormat('en-US', {
		maximumFractionDigits: 1,
		minimumFractionDigits: 0
	});

	const percentFormatter = new Intl.NumberFormat('en-US', {
		maximumFractionDigits: 1,
		minimumFractionDigits: 1
	});

	$: diskPercent = info.disk_total > 0 ? (info.disk_used / info.disk_total) * 100 : 0;
	$: freeMemory = Math.max(info.total_memory - metrics.used_memory, 0);
	$: cpuStatus = getResourceStatus(metrics.cpu_usage);
	$: memoryStatus = getResourceStatus(metrics.memory_percent);
	$: diskStatus = getResourceStatus(diskPercent);
	$: hostLine = [info.os_name, info.os_version].filter(Boolean).join(' ') || 'Unknown OS';
	$: lastUpdated = new Intl.DateTimeFormat('en-US', {
		hour: 'numeric',
		minute: '2-digit',
		second: '2-digit'
	}).format(new Date(loadedAt));

	function formatBytes(bytes: number) {
		const units = ['B', 'KB', 'MB', 'GB', 'TB'];
		let value = bytes;
		let unitIndex = 0;

		while (value >= 1024 && unitIndex < units.length - 1) {
			value /= 1024;
			unitIndex += 1;
		}

		return `${byteFormatter.format(value)} ${units[unitIndex]}`;
	}

	function getResourceStatus(percent: number) {
		if (percent >= 90) {
			return 'critical';
		}

		if (percent >= 70) {
			return 'warning';
		}

		return 'healthy';
	}

	function getResourceLabel(status: string) {
		if (status === 'critical') {
			return 'Low';
		}

		if (status === 'warning') {
			return 'Tight';
		}

		return 'Healthy';
	}

	function formatUptime(totalSeconds: number) {
		const days = Math.floor(totalSeconds / 86400);
		const hours = Math.floor((totalSeconds % 86400) / 3600);
		const minutes = Math.floor((totalSeconds % 3600) / 60);

		if (days > 0) {
			return `${days}d ${hours}h ${minutes}m`;
		}

		if (hours > 0) {
			return `${hours}h ${minutes}m`;
		}

		return `${minutes}m`;
	}

	async function refreshMetrics() {
		refreshing = true;
		refreshError = '';

		try {
			const response = await fetch('/api/system/metrics');

			if (!response.ok) {
				throw new Error(`Metrics refresh failed with ${response.status}`);
			}

			metrics = await response.json();
			loadedAt = new Date().toISOString();
		} catch (error) {
			refreshError = error instanceof Error ? error.message : 'Metrics refresh failed';
		} finally {
			refreshing = false;
		}
	}

	onMount(() => {
		pollTimer = setInterval(refreshMetrics, 15000);

		return () => {
			if (pollTimer) {
				clearInterval(pollTimer);
			}
		};
	});
</script>

<svelte:head>
	<title>{title}</title>
	<meta name="description" content={description} />
	<link rel="canonical" href={canonicalUrl} />
	<meta property="og:type" content="website" />
	<meta property="og:title" content={title} />
	<meta property="og:description" content={description} />
	<meta property="og:url" content={canonicalUrl} />
	<meta name="twitter:title" content={title} />
	<meta name="twitter:description" content={description} />
</svelte:head>

<Container>
	<Navbar />

	<div class="title-row">
		<div>
			<h1>System</h1>
			<p>Current server stats from <code>/api/system/info</code> and <code>/api/system/metrics</code>.</p>
		</div>
		<button type="button" onclick={refreshMetrics} disabled={refreshing}>
			{refreshing ? 'Refreshing...' : 'Refresh'}
		</button>
	</div>
	<hr />

	<p class="updated">Last updated: {lastUpdated}</p>

	{#if refreshError}
		<p class="error" role="status">{refreshError}</p>
	{/if}

	<div class="cards" aria-label="Server stats">
		<section class={`card status-${cpuStatus}`}>
			<div class="card-title">
				<h2>CPU</h2>
				<div class="metric-heading">
					<span>{getResourceLabel(cpuStatus)}</span>
					<strong>{percentFormatter.format(metrics.cpu_usage)}%</strong>
				</div>
			</div>
			<div
				class={`bar status-${cpuStatus}`}
				aria-label={`CPU usage ${percentFormatter.format(metrics.cpu_usage)} percent, ${getResourceLabel(cpuStatus)}`}
			>
				<span style={`width: ${Math.min(metrics.cpu_usage, 100)}%`}></span>
			</div>
			<p>{info.cpu_brand}</p>
			<p class="muted">{info.cpu_cores} physical cores</p>
		</section>

		<section class={`card status-${memoryStatus}`}>
			<div class="card-title">
				<h2>Memory</h2>
				<div class="metric-heading">
					<span>{getResourceLabel(memoryStatus)}</span>
					<strong>{percentFormatter.format(metrics.memory_percent)}%</strong>
				</div>
			</div>
			<div
				class={`bar status-${memoryStatus}`}
				aria-label={`Memory usage ${percentFormatter.format(metrics.memory_percent)} percent, ${getResourceLabel(memoryStatus)}`}
			>
				<span style={`width: ${Math.min(metrics.memory_percent, 100)}%`}></span>
			</div>
			<div class="stats-row">
				<span>{formatBytes(metrics.used_memory)} used</span>
				<span>{formatBytes(freeMemory)} free</span>
			</div>
		</section>

		<section class={`card status-${diskStatus}`}>
			<div class="card-title">
				<h2>Disk</h2>
				<div class="metric-heading">
					<span>{getResourceLabel(diskStatus)}</span>
					<strong>{percentFormatter.format(diskPercent)}%</strong>
				</div>
			</div>
			<div
				class={`bar status-${diskStatus}`}
				aria-label={`Disk usage ${percentFormatter.format(diskPercent)} percent, ${getResourceLabel(diskStatus)}`}
			>
				<span style={`width: ${Math.min(diskPercent, 100)}%`}></span>
			</div>
			<div class="stats-row">
				<span>{formatBytes(info.disk_used)} used</span>
				<span>{formatBytes(info.disk_total)} total</span>
			</div>
		</section>

		<section class="card">
			<div class="card-title">
				<h2>Uptime</h2>
				<strong>{formatUptime(metrics.uptime_seconds)}</strong>
			</div>
			<p>{metrics.uptime_seconds.toLocaleString('en-US')} seconds since boot.</p>
		</section>
	</div>

	<h2>Host</h2>
	<hr />
	<table class="host-table">
		<tbody>
			<tr>
				<th>Name</th>
				<td>{info.hostname ?? 'Unknown'}</td>
			</tr>
			<tr>
				<th>Platform</th>
				<td>{hostLine}</td>
			</tr>
			<tr>
				<th>Kernel</th>
				<td>{info.kernel_version ?? 'Unknown'}</td>
			</tr>
		</tbody>
	</table>

	<h2>Payload</h2>
	<hr />
	<pre>{JSON.stringify({ info, metrics }, null, 2)}</pre>

	<hr />
	<Footer />
</Container>

<style>
	:global(body) {
		height: auto;
		min-height: 100vh;
	}

	h1 {
		margin-top: 0.5rem;
		margin-bottom: 0.3rem;
		color: white;
	}

	h2 {
		margin-top: 1rem;
		margin-bottom: 0.3rem;
		color: white;
	}

	hr {
		display: block;
		width: 100%;
		box-sizing: border-box;
	}

	.title-row {
		display: flex;
		gap: 1rem;
		align-items: flex-start;
		justify-content: space-between;
	}

	.title-row p {
		margin-top: 0;
		color: #bbb;
	}

	.updated {
		color: #bbb;
		font-size: 0.9rem;
	}

	button {
		margin-top: 0.5rem;
		padding: 0.35rem 0.65rem;
		color: white;
		font: inherit;
		border: 1px solid #444;
		border-radius: 5px;
		background: #111;
		cursor: pointer;
	}

	button:hover {
		border-color: #58a6ff;
	}

	button:disabled {
		opacity: 0.7;
		cursor: progress;
	}

	.error {
		margin: 1rem 0;
		padding: 0.75rem 1rem;
		color: #ff4d4d;
		border: 1px solid rgba(255, 77, 77, 0.5);
		border-radius: 5px;
		background: rgba(255, 77, 77, 0.1);
	}

	.cards {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
		gap: 1rem;
		margin: 1rem 0 1.5rem;
	}

	.card {
		padding: 1rem;
		border: 1px solid #333;
		border-radius: 5px;
		background: rgba(255, 255, 255, 0.03);
	}

	.card.status-healthy {
		border-color: rgba(60, 214, 109, 0.45);
		background: rgba(60, 214, 109, 0.06);
	}

	.card.status-warning {
		border-color: rgba(245, 166, 35, 0.55);
		background: rgba(245, 166, 35, 0.07);
	}

	.card.status-critical {
		border-color: rgba(255, 77, 77, 0.65);
		background: rgba(255, 77, 77, 0.08);
	}

	.card-title {
		display: flex;
		align-items: baseline;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 0.8rem;
	}

	.card h2 {
		margin: 0;
		font-size: 1.1rem;
	}

	.metric-heading {
		display: flex;
		align-items: center;
		gap: 0.55rem;
	}

	.metric-heading span {
		padding: 0.12rem 0.38rem;
		color: #d6f7df;
		font-size: 0.72rem;
		font-weight: 700;
		line-height: 1.3;
		text-transform: uppercase;
		border: 1px solid rgba(60, 214, 109, 0.55);
		border-radius: 999px;
		background: rgba(60, 214, 109, 0.12);
	}

	.status-warning .metric-heading span {
		color: #ffe1a6;
		border-color: rgba(245, 166, 35, 0.65);
		background: rgba(245, 166, 35, 0.15);
	}

	.status-critical .metric-heading span {
		color: #ffb8b8;
		border-color: rgba(255, 77, 77, 0.65);
		background: rgba(255, 77, 77, 0.15);
	}

	.card strong {
		font-family: JetBrainsMono, monospace;
		color: white;
	}

	.card p {
		margin: 0.75rem 0 0;
		color: #e5e5e5;
	}

	.muted,
	.stats-row {
		color: #bbb !important;
		font-size: 0.9rem;
	}

	.stats-row {
		display: flex;
		justify-content: space-between;
		gap: 1rem;
		margin-top: 0.7rem;
	}

	.bar {
		height: 0.6rem;
		overflow: hidden;
		border: 1px solid #333;
		border-radius: 999px;
		background: #111;
	}

	.bar span {
		display: block;
		height: 100%;
		background: #3cd66d;
	}

	.bar.status-warning span {
		background: #f5a623;
	}

	.bar.status-critical span {
		background: #ff4d4d;
	}

	.host-table {
		display: table;
		width: 100%;
		overflow: visible;
		table-layout: fixed;
	}

	.host-table th {
		width: 8rem;
		vertical-align: top;
	}

	.host-table th,
	.host-table td {
		border-bottom: 1px solid #333;
	}

	.host-table tr:last-child th,
	.host-table tr:last-child td {
		border-bottom: none;
	}

	.host-table td {
		word-break: break-word;
	}

	@media (max-width: 600px) {
		.title-row,
		.stats-row {
			display: block;
		}
	}
</style>
