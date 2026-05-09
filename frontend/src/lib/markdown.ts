import MarkdownIt from 'markdown-it';
import { codeToHtml } from 'shiki';

type ParseMarkdownOptions = {
	headingPermalinks?: boolean;
};

type RenderRule = NonNullable<MarkdownIt['renderer']['rules']['heading_open']>;
type MarkdownToken = Parameters<RenderRule>[0][number];

function calloutPlugin(md: MarkdownIt) {
	md.block.ruler.before('fence', 'callout', (state, startLine, endLine, silent) => {
		const start = state.getLines(startLine, startLine + 1, 0, false).trim();

		if (!start.startsWith(':::')) return false;
		if (silent) return true;

		// Parse ":::type optional title"
		const header = start.slice(3).trim();
		const [type, ...titleParts] = header.split(' ');
		const title = titleParts.join(' ');

		let nextLine = startLine + 1;

		while (nextLine < endLine) {
			const line = state.getLines(nextLine, nextLine + 1, 0, false).trim();
			if (line === ':::') break;
			nextLine++;
		}

		if (nextLine >= endLine) return false;

		const content = state.getLines(startLine + 1, nextLine, state.blkIndent, false);

		state.line = nextLine + 1;

		const token = state.push('html_block', '', 0);
		token.content = `
<div class="callout callout-${type}">
	<div class="callout-header">
		<span class="callout-icon"></span>
		${title ? `<span class="callout-title">${md.utils.escapeHtml(title)}</span>` : ''}
	</div>
	<div class="callout-body">
		${md.render(content)}
	</div>
</div>
`;

		return true;
	});
}

function collectTokenText(token: MarkdownToken): string {
	if (token.children) {
		return token.children.map(collectTokenText).join('');
	}

	if (token.type === 'html_block' || token.type === 'html_inline') {
		return '';
	}

	return token.content;
}

function slugifyHeading(heading: string): string {
	return (
		heading
			.normalize('NFKD')
			.replace(/[\u0300-\u036f]/g, '')
			.toLowerCase()
			.trim()
			.replace(/[^a-z0-9]+/g, '-')
			.replace(/^-+|-+$/g, '') || 'section'
	);
}

function headingPermalinkPlugin(md: MarkdownIt) {
	const headingLevels = new Set(['h2', 'h3']);
	const usedSlugs = new Map<string, number>();
	const defaultHeadingOpen: RenderRule =
		md.renderer.rules.heading_open ??
		((tokens, idx, options, _env, self) => self.renderToken(tokens, idx, options));
	const defaultHeadingClose: RenderRule =
		md.renderer.rules.heading_close ??
		((tokens, idx, options, _env, self) => self.renderToken(tokens, idx, options));

	const uniqueSlug = (baseSlug: string) => {
		const count = usedSlugs.get(baseSlug) ?? 0;
		usedSlugs.set(baseSlug, count + 1);
		return count === 0 ? baseSlug : `${baseSlug}-${count + 1}`;
	};

	md.renderer.rules.heading_open = (tokens, idx, options, env, self) => {
		const token = tokens[idx];

		if (!headingLevels.has(token.tag)) {
			return defaultHeadingOpen(tokens, idx, options, env, self);
		}

		const existingId = token.attrGet('id');
		if (!existingId) {
			const inlineToken = tokens[idx + 1];
			const headingText = inlineToken ? collectTokenText(inlineToken) : '';
			token.attrSet('id', uniqueSlug(slugifyHeading(headingText)));
		}

		return defaultHeadingOpen(tokens, idx, options, env, self);
	};

	md.renderer.rules.heading_close = (tokens, idx, options, env, self) => {
		const openToken = tokens[idx - 2];
		const inlineToken = tokens[idx - 1];

		if (!openToken || !inlineToken || !headingLevels.has(openToken.tag)) {
			return defaultHeadingClose(tokens, idx, options, env, self);
		}

		const id = openToken.attrGet('id');
		if (!id) {
			return defaultHeadingClose(tokens, idx, options, env, self);
		}

		const headingText = collectTokenText(inlineToken);
		const label = md.utils.escapeHtml(`Link to section: ${headingText}`);
		const escapedId = md.utils.escapeHtml(id);

		return `<a class="heading-anchor" href="#${escapedId}" aria-label="${label}" title="${label}">¶</a>${defaultHeadingClose(
			tokens,
			idx,
			options,
			env,
			self
		)}`;
	};
}

export async function parseMarkdownWithShiki(
	markdown: string,
	options: ParseMarkdownOptions = {}
): Promise<string> {
	// Extract code blocks
	const codeBlocks: { original: string; lang: string; code: string }[] = [];
	const codeBlockRegex = /```(\w+)?\n([\s\S]*?)```/g;
	let match;

	while ((match = codeBlockRegex.exec(markdown)) !== null) {
		codeBlocks.push({
			original: match[0],
			lang: match[1] || 'text',
			code: match[2]
		});
	}

	// Highlight code blocks with Shiki
	const highlightedBlocks = await Promise.all(
		codeBlocks.map(async (block) => {
			try {
				return await codeToHtml(block.code, {
					lang: block.lang,
					theme: 'dark-plus'
				});
			} catch {
				return `<pre><code>${block.code}</code></pre>`;
			}
		})
	);

	// Replace code blocks with placeholders
	let processedMarkdown = markdown;
	codeBlocks.forEach((block, index) => {
		processedMarkdown = processedMarkdown.replace(block.original, `<!--CODE_BLOCK_${index}-->`);
	});

	// Markdown-it with custom callouts
	const md = new MarkdownIt({
		html: true,
		linkify: true,
		typographer: true
	}).use(calloutPlugin);

	if (options.headingPermalinks) {
		md.use(headingPermalinkPlugin);
	}

	let html = md.render(processedMarkdown);

	// Restore highlighted code blocks
	highlightedBlocks.forEach((highlighted, index) => {
		html = html.replace(`<!--CODE_BLOCK_${index}-->`, highlighted);
	});

	return html;
}
