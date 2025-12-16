use crate::content::fs::{collect_markdown_files, read_markdown_file};
use crate::content::models::{ContentItem, Frontmatter};
use crate::content::parser::parse_markdown;
use crate::utils::read_time::estimate_read_time;
use std::path::Path;

pub fn create_content_item(md: &str, slug: &str) -> Result<ContentItem, String> {
    let (frontmatter, body) = parse_markdown(md)?;
    let read_time = estimate_read_time(&body);

    Ok(ContentItem {
        slug: slug.to_string(),
        frontmatter,
        markdown: body,
        read_time,
    })
}

pub fn load_content_from_dir(root: &Path) -> Result<Vec<ContentItem>, String> {
    let mut items = Vec::new();

    let files = collect_markdown_files(root)?;

    for path in files {
        let md = read_markdown_file(&path)?;
        let slug = path
            .file_stem()
            .ok_or("Failed to get file stem")?
            .to_string_lossy()
            .to_string();

        let item = create_content_item(&md, &slug)?;
        items.push(item);
    }

    Ok(items)
}
