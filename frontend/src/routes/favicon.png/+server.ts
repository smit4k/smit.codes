import { site } from '$lib/site';
import type { RequestHandler } from './$types';

export const prerender = true;

const fallbackFavicon =
	'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO+/p9sAAAAASUVORK5CYII=';

function getFallbackFavicon() {
	const body = Uint8Array.from(atob(fallbackFavicon), (character) => character.charCodeAt(0));

	return new Response(body, {
		headers: {
			'cache-control': 'public, max-age=3600',
			'content-type': 'image/png'
		}
	});
}

export const GET: RequestHandler = async ({ fetch }) => {
	try {
		const profileResponse = await fetch(`https://api.github.com/users/${site.githubUsername}`, {
			headers: {
				accept: 'application/vnd.github+json',
				'user-agent': site.name
			}
		});

		if (!profileResponse.ok) {
			return getFallbackFavicon();
		}

		const profile = (await profileResponse.json()) as { avatar_url?: string };

		if (!profile.avatar_url) {
			return getFallbackFavicon();
		}

		const avatarUrl = new URL(profile.avatar_url);
		avatarUrl.searchParams.set('s', '64');

		const avatarResponse = await fetch(avatarUrl);

		if (!avatarResponse.ok) {
			return getFallbackFavicon();
		}

		return new Response(await avatarResponse.arrayBuffer(), {
			headers: {
				'cache-control': 'public, max-age=86400',
				'content-type': avatarResponse.headers.get('content-type') ?? 'image/png'
			}
		});
	} catch {
		return getFallbackFavicon();
	}
};
