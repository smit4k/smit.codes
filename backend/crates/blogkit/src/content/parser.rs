use crate::content::models::Frontmatter;

pub fn parse_markdown(md: &str) -> Result<(Frontmatter, String), String> {
    let mut lines = md.lines();

    match lines.next() {
        Some(line) if line.trim() == "---" => {}
        _ => return Err("Missing frontmatter start delimiter (---) on line 1".into()),
    }

    let mut fm_lines = Vec::new();

    for line in &mut lines {
        if line.trim() == "---" {
            let yaml = fm_lines.join("\n");

            let frontmatter: Frontmatter =
                serde_yaml::from_str(&yaml).map_err(|e| e.to_string())?;

            let markdown = lines.collect::<Vec<_>>().join("\n");

            return Ok((frontmatter, markdown));
        }

        fm_lines.push(line);
    }

    Err("Missing frontmatter end delimiter (---)".into())
}
