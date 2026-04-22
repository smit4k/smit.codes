import type { PhotoImage, PhotoPost } from '$lib/types';

export function resolvePhotoAssetUrl(src: string, apiBaseUrl: string): string {
	if (!src.startsWith('/assets/')) {
		return src;
	}

	if (!apiBaseUrl) {
		return src;
	}

	return `${apiBaseUrl}${src}`;
}

export function normalizePhotoImage(image: PhotoImage, apiBaseUrl: string): PhotoImage {
	return {
		...image,
		src: resolvePhotoAssetUrl(image.src, apiBaseUrl)
	};
}

export function normalizePhotoPost(post: PhotoPost, apiBaseUrl: string): PhotoPost {
	return {
		...post,
		coverImage: resolvePhotoAssetUrl(post.coverImage, apiBaseUrl),
		previewImages: post.previewImages.map((src) => resolvePhotoAssetUrl(src, apiBaseUrl)),
		images: post.images.map((image) => normalizePhotoImage(image, apiBaseUrl))
	};
}
