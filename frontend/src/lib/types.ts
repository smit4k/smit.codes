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

export type PhotoImage = {
    src: string;
    width?: number;
    height?: number;
    alt?: string;
};

export type PhotoPost = {
    slug: string;
    title: string;
    date: string;
    tags: string[];
    coverImage: string;
    previewImages: string[];
    images: PhotoImage[];
    description?: string;
};

export type ViewCountResponse = {
    total_views: number;
    unique_views: number;
}
