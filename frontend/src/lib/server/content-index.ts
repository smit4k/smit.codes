// @ts-nocheck
import { readdir, readFile, stat } from 'node:fs/promises';
import { basename, extname, join } from 'node:path';

const contentRoot = join(process.cwd(), '..', 'backend', 'content');

type IndexedPage = {
	title: string;
	path: string;
	description?: string;
	lastModified?: string;
	date?: string;
	tags?: string[];
};

function extractField(block: string, field: string) {
	const quotedMatch = block.match(new RegExp(`^${field}:\\s*"([^"]+)"$`, 'm'));
	if (quotedMatch) {
		return quotedMatch[1].trim();
	}

	const plainMatch = block.match(new RegExp(`^${field}:\\s*(.+)$`, 'm'));
	return plainMatch?.[1]?.trim();
}

function extractListField(block: string, field: string) {
	const match = block.match(new RegExp(`^${field}:\\s*\\[([^\\]]*)\\]$`, 'm'));
	if (!match) {
		return [];
	}

	return match[1]
		.split(',')
		.map((value) => value.trim().replace(/^["']|["']$/g, ''))
		.filter(Boolean);
}

function extractMarkdownFrontmatter(source: string) {
	const match = source.match(/^---\n([\s\S]*?)\n---/);
	if (!match) {
		return {};
	}

	const block = match[1];
	return {
		title: extractField(block, 'title'),
		description: extractField(block, 'description'),
		date: extractField(block, 'date'),
		tags: extractListField(block, 'tags')
	};
}

function extractPhotoFrontmatter(source: string) {
	return {
		title: extractField(source, 'title'),
		description: extractField(source, 'description'),
		date: extractField(source, 'date')
	};
}

async function readMarkdownIndex(section: 'writing' | 'projects') {
	const directory = join(contentRoot, section);
	const entries = await readdir(directory, { withFileTypes: true });

	const pages = await Promise.all(
		entries
			.filter((entry) => entry.isFile() && extname(entry.name) === '.md')
			.map(async (entry) => {
				const filePath = join(directory, entry.name);
				const source = await readFile(filePath, 'utf8');
				const fileStat = await stat(filePath);
				const metadata = extractMarkdownFrontmatter(source);
				const slug = basename(entry.name, '.md');
				const basePath = section === 'writing' ? '/writing' : '/projects';

				return {
					title: metadata.title ?? slug,
					path: `${basePath}/${slug}`,
					description: metadata.description,
					lastModified: fileStat.mtime.toISOString(),
					date: metadata.date,
					tags: metadata.tags
				} satisfies IndexedPage;
			})
	);

	return pages.sort((a, b) => (b.date ?? '').localeCompare(a.date ?? ''));
}

async function readPhotoIndex() {
	const directory = join(contentRoot, 'photos');
	const entries = await readdir(directory, { withFileTypes: true });

	const pages = await Promise.all(
		entries
			.filter((entry) => entry.isDirectory())
			.map(async (entry) => {
				try {
					const manifestPath = join(directory, entry.name, 'post.yaml');
					const source = await readFile(manifestPath, 'utf8');
					const fileStat = await stat(manifestPath);
					const metadata = extractPhotoFrontmatter(source);

					return {
						title: metadata.title ?? entry.name,
						path: `/photowall/${entry.name}`,
						description: metadata.description,
						lastModified: fileStat.mtime.toISOString(),
						date: metadata.date
					} satisfies IndexedPage;
				} catch {
					return null;
				}
			})
	);

	return pages
		.filter(Boolean)
		.sort((a, b) => (b.date ?? '').localeCompare(a.date ?? ''));
}

export async function getIndexedContent() {
	const [writing, projects, photos] = await Promise.all([
		readMarkdownIndex('writing'),
		readMarkdownIndex('projects'),
		readPhotoIndex()
	]);

	return { writing, projects, photos };
}
