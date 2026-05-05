export type SystemInfo = {
	os_name: string | null;
	os_version: string | null;
	kernel_version: string | null;
	hostname: string | null;
	cpu_brand: string;
	cpu_cores: number;
	total_memory: number;
	disk_total: number;
	disk_used: number;
};

export type SystemMetrics = {
	cpu_usage: number;
	used_memory: number;
	memory_percent: number;
	uptime_seconds: number;
};

async function fetchJson<T>(fetch: typeof globalThis.fetch, path: string): Promise<T> {
	const response = await fetch(path);

	if (!response.ok) {
		throw new Error(`Failed to fetch ${path}: ${response.status}`);
	}

	return response.json();
}

export async function load({ fetch }) {
	const [info, metrics] = await Promise.all([
		fetchJson<SystemInfo>(fetch, '/api/system/info'),
		fetchJson<SystemMetrics>(fetch, '/api/system/metrics')
	]);

	return {
		info,
		metrics,
		loadedAt: new Date().toISOString()
	};
}
