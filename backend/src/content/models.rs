use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Frontmatter {
    pub title: String,
    pub date: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentKind {
    Project,
    Writing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentItem {
    pub slug: String,
    pub kind: ContentKind,
    pub frontmatter: Frontmatter,
    pub markdown: String,
    pub read_time: u32,
}
