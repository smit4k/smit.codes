export type Frontmatter = {
    title: string;
    date: string;
    tags: string[];
    links: string[];
    description: string;
};

export type ContentItem = {
    slug: string;
    frontmatter: Frontmatter;
    markdown: string;
    read_time: number;
};


export type ViewCountResponse = {
    total_views: number;
    unique_views: number;
}
