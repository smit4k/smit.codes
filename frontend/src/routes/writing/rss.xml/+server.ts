import { getIndexedContent } from '$lib/server/content-index';
import { absoluteUrl, site } from '$lib/site';

export const prerender = true;

const feedTitle = `${site.name} - Writing`;
const feedDescription =
	'Technical writing by Smit Patil covering software projects, configuration languages, tools, and programming experiments.';
const feedPath = '/writing/rss.xml';

function escapeXml(value: string) {
	return value
		.replace(/&/g, '&amp;')
		.replace(/</g, '&lt;')
		.replace(/>/g, '&gt;')
		.replace(/"/g, '&quot;')
		.replace(/'/g, '&apos;');
}

function rssDate(value?: string) {
	if (!value) {
		return undefined;
	}

	const date = value.includes('T') ? new Date(value) : new Date(`${value}T00:00:00.000Z`);
	if (Number.isNaN(date.getTime())) {
		return undefined;
	}

	return date.toUTCString();
}

export async function GET() {
	const { writing } = await getIndexedContent();
	const sortedWriting = [...writing].sort((a, b) => (b.date ?? '').localeCompare(a.date ?? ''));
	const latestDate = rssDate(sortedWriting[0]?.date ?? sortedWriting[0]?.lastModified);

	const items = sortedWriting
		.map((post) => {
			const url = absoluteUrl(post.path);
			const pubDate = rssDate(post.date ?? post.lastModified);
			const categories = (post.tags ?? [])
				.map((tag: string) => `      <category>${escapeXml(tag)}</category>`)
				.join('\n');

			return `    <item>
      <title>${escapeXml(post.title)}</title>
      <link>${escapeXml(url)}</link>
      <guid isPermaLink="true">${escapeXml(url)}</guid>${pubDate ? `
      <pubDate>${escapeXml(pubDate)}</pubDate>` : ''}${post.description ? `
      <description>${escapeXml(post.description)}</description>` : ''}${categories ? `
${categories}` : ''}
    </item>`;
		})
		.join('\n');

	const xml = `<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
  <channel>
    <title>${escapeXml(feedTitle)}</title>
    <link>${escapeXml(absoluteUrl('/writing'))}</link>
    <description>${escapeXml(feedDescription)}</description>
    <language>en-US</language>
    <atom:link href="${escapeXml(absoluteUrl(feedPath))}" rel="self" type="application/rss+xml" />${latestDate ? `
    <lastBuildDate>${escapeXml(latestDate)}</lastBuildDate>` : ''}
${items}
  </channel>
</rss>`;

	return new Response(xml, {
		headers: {
			'content-type': 'application/rss+xml; charset=utf-8'
		}
	});
}
