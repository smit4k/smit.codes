import { absoluteUrl } from '$lib/site';
import { getIndexedContent } from '$lib/server/content-index';

export const prerender = true;

export async function GET() {
	const { writing, projects, photos } = await getIndexedContent();
	const pages: Array<{ path: string; lastModified?: string }> = [
		{ path: '/', lastModified: undefined },
		{ path: '/projects', lastModified: undefined },
		{ path: '/writing', lastModified: undefined },
		{ path: '/photowall', lastModified: undefined },
		...projects.map((page: { path: string; lastModified?: string }) => ({
			path: page.path,
			lastModified: page.lastModified
		})),
		...writing.map((page: { path: string; lastModified?: string }) => ({
			path: page.path,
			lastModified: page.lastModified
		})),
		...photos.map((page: { path: string; lastModified?: string }) => ({
			path: page.path,
			lastModified: page.lastModified
		}))
	];

	const xml = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${pages
	.map(
		(page) => `  <url>
    <loc>${absoluteUrl(page.path)}</loc>${page.lastModified ? `
    <lastmod>${page.lastModified}</lastmod>` : ''}
  </url>`
	)
	.join('\n')}
</urlset>`;

	return new Response(xml, {
		headers: {
			'content-type': 'application/xml; charset=utf-8'
		}
	});
}
