use crate::content::models::Frontmatter;

pub fn parse_markdown(md: &str) -> Result<(Frontmatter, String), String> {
    let input = md.trim_start();

    if !input.starts_with("---") {
        return Err("Missing frontmatter".into());
    }

    let mut parts = input.splitn(4, "---");

    parts.next();

    let yaml = parts.next().ok_or("Missing frontmatter")?;

    let markdown = parts
        .next()
        .ok_or("Missing markdown content")?
        .trim_start()
        .to_string();

    let frontmatter: Frontmatter = serde_yaml::from_str(yaml).map_err(|e| e.to_string())?;

    Ok((frontmatter, markdown))
}
