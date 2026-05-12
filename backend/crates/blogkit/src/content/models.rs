use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Frontmatter {
    pub title: String,
    pub date: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub links: Vec<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhotoImage {
    pub src: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub alt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhotoPost {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub cover_image: String,
    pub preview_images: Vec<String>,
    pub images: Vec<PhotoImage>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhotoPostManifest {
    pub title: String,
    pub date: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub cover_image: String,
    #[serde(default)]
    pub preview_images: Vec<String>,
    #[serde(default)]
    pub images: Vec<PhotoImage>,
    pub description: Option<String>,
}
