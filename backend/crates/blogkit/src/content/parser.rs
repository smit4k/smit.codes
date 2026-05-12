use crate::content::models::Frontmatter;

pub fn parse_markdown(md: &str) -> Result<(Frontmatter, String), String> {
    let mut lines = md.lines();

    match lines.next() {
        Some(line) if line.trim() == "---" => {}
        _ => return Err("Missing frontmatter start delimiter (---) on line 1".into()),
    }

    let mut fm_lines = Vec::new();
    for i in 2..=6 {
        match lines.next() {
            Some(line) => fm_lines.push(line),
            None => return Err(format!("Missing frontmatter content on line {}", i)),
        }
    }

    match lines.next() {
        Some(line) if line.trim() == "---" => {}
        _ => return Err("Missing frontmatter end delimiter (---) on line 7".into()),
    }

    let yaml = fm_lines.join("\n");

    let frontmatter: Frontmatter = serde_yaml::from_str(&yaml).map_err(|e| e.to_string())?;

    let markdown = lines.collect::<Vec<_>>().join("\n");

    Ok((frontmatter, markdown))
}
