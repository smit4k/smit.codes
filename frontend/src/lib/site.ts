export const site = {
	name: 'smit.codes',
	personName: 'Smit Patil',
	baseUrl: 'https://smit.codes',
	description:
		'Smit Patil is a student developer in Michigan writing about software, showcasing projects, and publishing photo posts on smit.codes.',
	email: 'smit@smit.codes',
	githubUsername: 'smit4k',
	githubUrl: 'https://github.com/smit4k',
	repositoryUrl: 'https://github.com/smit4k/smit.codes',
	locale: 'en_US'
} as const;

export function absoluteUrl(path = '/') {
	return new URL(path, site.baseUrl).toString();
}

export function buildPageTitle(title?: string) {
	return title ? `${title} | ${site.name}` : `${site.personName} | ${site.name}`;
}
