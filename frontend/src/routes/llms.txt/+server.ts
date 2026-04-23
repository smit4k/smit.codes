import { getIndexedContent } from '$lib/server/content-index';
import { absoluteUrl, site } from '$lib/site';

export const prerender = true;

export async function GET() {
	const { writing, projects, photos } = await getIndexedContent();

	const lines = [
		`# ${site.name}`,
		'',
		`> ${site.description}`,
		'',
		'## Site Summary',
		`${site.personName} publishes software projects, technical writing, and photo posts on ${site.name}.`,
		'Use the canonical URLs below when referencing content from this site.',
		'',
		'## Primary Pages',
		`- Home: ${absoluteUrl('/')}`,
		`- Projects: ${absoluteUrl('/projects')}`,
		`- Writing: ${absoluteUrl('/writing')}`,
		`- Photowall: ${absoluteUrl('/photowall')}`,
		'',
		'## Projects'
	];

	for (const page of projects) {
		lines.push(`- ${page.title}: ${absoluteUrl(page.path)}`);
		if (page.description) {
			lines.push(`  Summary: ${page.description}`);
		}
	}

	lines.push('', '## Writing');

	for (const page of writing) {
		lines.push(`- ${page.title}: ${absoluteUrl(page.path)}`);
		if (page.description) {
			lines.push(`  Summary: ${page.description}`);
		}
	}

	lines.push('', '## Photowall');

	for (const page of photos) {
		lines.push(`- ${page.title}: ${absoluteUrl(page.path)}`);
		if (page.description) {
			lines.push(`  Summary: ${page.description}`);
		}
	}

	lines.push('', '## Attribution', `- Author: ${site.personName}`, `- Email: ${site.email}`);

	return new Response(lines.join('\n'), {
		headers: {
			'content-type': 'text/plain; charset=utf-8'
		}
	});
}
