import { absoluteUrl } from '$lib/site';

type JsonLdValue = Record<string, unknown> | Array<Record<string, unknown>>;

export function serializeJsonLd(value: JsonLdValue) {
	return JSON.stringify(value).replace(/</g, '\\u003c');
}

export function breadcrumbJsonLd(items: Array<{ name: string; path: string }>) {
	return {
		'@context': 'https://schema.org',
		'@type': 'BreadcrumbList',
		itemListElement: items.map((item, index) => ({
			'@type': 'ListItem',
			position: index + 1,
			name: item.name,
			item: absoluteUrl(item.path)
		}))
	};
}

export function collectionJsonLd({
	title,
	description,
	path,
	items
}: {
	title: string;
	description: string;
	path: string;
	items: Array<{ name: string; path: string }>;
}) {
	return {
		'@context': 'https://schema.org',
		'@type': 'CollectionPage',
		name: title,
		description,
		url: absoluteUrl(path),
		mainEntity: {
			'@type': 'ItemList',
			itemListElement: items.map((item, index) => ({
				'@type': 'ListItem',
				position: index + 1,
				name: item.name,
				url: absoluteUrl(item.path)
			}))
		}
	};
}
