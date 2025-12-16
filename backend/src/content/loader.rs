use crate::content::models::{ContentItem, Frontmatter};
use crate::content::parser::parse_markdown;
use crate::utils::read_time::estimate_read_time;

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
