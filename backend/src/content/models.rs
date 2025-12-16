use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Frontmatter {
    pub title: String,
    pub date: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ContentItem {
    pub slug: String,
    pub frontmatter: Frontmatter,
    pub markdown: String,
    pub read_time: u32,
}
