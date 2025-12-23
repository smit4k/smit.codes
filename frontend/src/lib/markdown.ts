import MarkdownIt from 'markdown-it';
import { codeToHtml } from 'shiki';

export async function parseMarkdownWithShiki(markdown: string): Promise<string> {
    // First, extract all code blocks
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

    // Highlight all code blocks
    const highlightedBlocks = await Promise.all(
        codeBlocks.map(async (block) => {
            try {
                return await codeToHtml(block.code, {
                    lang: block.lang,
                    theme: 'dark-plus'
                });
            } catch (e) {
                return `<pre><code>${block.code}</code></pre>`;
            }
        })
    );

    // Replace code blocks with HTML comment placeholders that markdown-it won't touch
    let processedMarkdown = markdown;
    codeBlocks.forEach((block, index) => {
        processedMarkdown = processedMarkdown.replace(
            block.original,
            `<!--CODE_BLOCK_${index}-->`
        );
    });

    // Parse remaining markdown with markdown-it
    const md: MarkdownIt = new MarkdownIt({
        html: true,
        linkify: true,
        typographer: true
    });

    let html = md.render(processedMarkdown);

    // Replace HTML comment placeholders with highlighted code
    highlightedBlocks.forEach((highlighted, index) => {
        html = html.replace(`<!--CODE_BLOCK_${index}-->`, highlighted);
    });

    return html;
}
