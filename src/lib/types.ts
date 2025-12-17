export type Frontmatter = {
    title: string;
    date: string;
    tags: string[];
};

export type ContentItem = {
    slug: string;
    frontmatter: Frontmatter;
    markdown: string;
    read_time: number;
};

