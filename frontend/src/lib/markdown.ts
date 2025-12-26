import MarkdownIt from 'markdown-it';
import { codeToHtml } from 'shiki';
function calloutPlugin(md: MarkdownIt) {
    md.block.ruler.before(
        'fence',
        'callout',
        (state, startLine, endLine, silent) => {
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

            const content = state.getLines(
                startLine + 1,
                nextLine,
                state.blkIndent,
                false
            );

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
        }
    );
}

export async function parseMarkdownWithShiki(markdown: string): Promise<string> {
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
        processedMarkdown = processedMarkdown.replace(
            block.original,
            `<!--CODE_BLOCK_${index}-->`
        );
    });

    // Markdown-it with custom callouts
    const md = new MarkdownIt({
        html: true,
        linkify: true,
        typographer: true
    }).use(calloutPlugin);

    let html = md.render(processedMarkdown);

    // Restore highlighted code blocks
    highlightedBlocks.forEach((highlighted, index) => {
        html = html.replace(`<!--CODE_BLOCK_${index}-->`, highlighted);
    });

    return html;
}

