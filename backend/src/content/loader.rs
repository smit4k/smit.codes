use crate::content::fs::{
    collect_files_with_extensions, collect_markdown_files, read_markdown_file, read_text_file,
};
use crate::content::models::{ContentItem, ContentKind, PhotoImage, PhotoPost, PhotoPostManifest};
use crate::content::parser::parse_markdown;
use crate::utils::read_time::estimate_read_time;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::time::SystemTime;

pub fn load_content_from_dir(root: &Path, kind: ContentKind) -> Result<Vec<ContentItem>, String> {
    let mut items = Vec::new();

    let files = collect_markdown_files(root)?;

    for path in files {
        let md = read_markdown_file(&path)?;
        let slug = path
            .file_stem()
            .ok_or("Failed to get file stem")?
            .to_string_lossy()
            .to_string();

        let item = create_content_item_with_asset_base(&md, &slug, kind.clone(), root, root)?;
        items.push(item);
    }

    Ok(items)
}

fn create_content_item_with_asset_base(
    md: &str,
    slug: &str,
    kind: ContentKind,
    content_root: &Path,
    base_dir: &Path,
) -> Result<ContentItem, String> {
    let (frontmatter, body) = parse_markdown(md)?;
    let body = resolve_markdown_image_paths(&body, content_root, base_dir)?;
    let read_time = estimate_read_time(&body);

    Ok(ContentItem {
        slug: slug.to_string(),
        kind,
        frontmatter,
        markdown: body,
        read_time,
    })
}

pub fn load_photo_posts_from_dir(root: &Path) -> Result<Vec<PhotoPost>, String> {
    let mut manifest_files = collect_files_with_extensions(root, &["json", "yaml", "yml"])?;
    manifest_files.sort();

    let mut posts = Vec::new();
    let mut folders_with_manifests = Vec::new();

    for path in manifest_files {
        let file_stem = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .ok_or_else(|| format!("Failed to read file stem for {}", path.display()))?;

        if !is_photo_manifest_file(root, &path, file_stem) {
            continue;
        }

        let raw_manifest = read_text_file(&path)?;
        let manifest = parse_photo_manifest(&path, &raw_manifest)?;
        posts.push(build_photo_post(root, &path, manifest)?);

        if let Some(parent) = path.parent() {
            folders_with_manifests.push(parent.to_path_buf());
        }
    }

    for entry in fs::read_dir(root).map_err(|err| err.to_string())? {
        let entry = entry.map_err(|err| err.to_string())?;
        let path = entry.path();

        if !path.is_dir() || folders_with_manifests.iter().any(|folder| folder == &path) {
            continue;
        }

        if let Some(generated_post) = build_generated_photo_post(root, &path)? {
            posts.push(generated_post);
        }
    }

    posts.sort_by(|a, b| b.date.cmp(&a.date).then_with(|| a.slug.cmp(&b.slug)));
    Ok(posts)
}

fn is_photo_manifest_file(root: &Path, path: &Path, stem: &str) -> bool {
    if stem == "post" || stem == "index" {
        return true;
    }

    path.parent() == Some(root)
}

fn parse_photo_manifest(path: &Path, raw_manifest: &str) -> Result<PhotoPostManifest, String> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("json") => serde_json::from_str(raw_manifest)
            .map_err(|err| format!("Failed to parse {}: {}", path.display(), err)),
        Some("yaml") | Some("yml") => serde_yaml::from_str(raw_manifest)
            .map_err(|err| format!("Failed to parse {}: {}", path.display(), err)),
        Some(extension) => Err(format!(
            "Unsupported photo manifest extension {} for {}",
            extension,
            path.display()
        )),
        None => Err(format!(
            "Photo manifest {} is missing an extension",
            path.display()
        )),
    }
}

fn build_photo_post(
    content_root: &Path,
    manifest_path: &Path,
    manifest: PhotoPostManifest,
) -> Result<PhotoPost, String> {
    let PhotoPostManifest {
        title,
        date,
        tags,
        cover_image,
        preview_images,
        images,
        description,
    } = manifest;

    let manifest_dir = manifest_path.parent().unwrap_or(content_root);
    let slug = photo_slug_from_manifest_path(manifest_path)?;

    let cover_image = resolve_asset_path(content_root, manifest_dir, &cover_image)?;

    let preview_sources = if preview_images.is_empty() {
        vec![cover_image.clone()]
    } else {
        preview_images
    };

    let preview_images = preview_sources
        .iter()
        .map(|src| resolve_asset_path(content_root, manifest_dir, src))
        .collect::<Result<Vec<_>, _>>()?;

    let images = if images.is_empty() {
        vec![PhotoImage {
            src: cover_image.clone(),
            width: None,
            height: None,
            alt: Some(title.clone()),
        }]
    } else {
        images
            .into_iter()
            .map(|image| {
                Ok(PhotoImage {
                    src: resolve_asset_path(content_root, manifest_dir, &image.src)?,
                    width: image.width,
                    height: image.height,
                    alt: image.alt,
                })
            })
            .collect::<Result<Vec<_>, String>>()?
    };

    Ok(PhotoPost {
        slug,
        title,
        date,
        tags,
        cover_image,
        preview_images,
        images,
        description,
    })
}

fn build_generated_photo_post(
    content_root: &Path,
    folder: &Path,
) -> Result<Option<PhotoPost>, String> {
    let mut image_files =
        collect_files_with_extensions(folder, &["png", "jpg", "jpeg", "webp", "gif", "avif"])?;
    image_files.sort();

    if image_files.is_empty() {
        return Ok(None);
    }

    let slug = folder
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| format!("Failed to derive photo slug from {}", folder.display()))?
        .to_string();

    let resolved_images = image_files
        .iter()
        .map(|path| {
            let relative = path.strip_prefix(folder).map_err(|_| {
                format!(
                    "Image {} is outside of {}",
                    path.display(),
                    folder.display()
                )
            })?;

            let src = resolve_asset_path(content_root, folder, &relative.to_string_lossy())?;

            Ok(PhotoImage {
                src,
                width: None,
                height: None,
                alt: None,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;

    let cover_image = resolved_images
        .first()
        .map(|image| image.src.clone())
        .ok_or_else(|| format!("Failed to generate cover image for {}", folder.display()))?;

    let preview_images = resolved_images
        .iter()
        .take(4)
        .map(|image| image.src.clone())
        .collect::<Vec<_>>();

    let date = latest_modified_date(&image_files)?;

    Ok(Some(PhotoPost {
        slug: slug.clone(),
        title: humanize_slug(&slug),
        date,
        tags: Vec::new(),
        cover_image,
        preview_images,
        images: resolved_images,
        description: None,
    }))
}

fn photo_slug_from_manifest_path(path: &Path) -> Result<String, String> {
    let stem = path
        .file_stem()
        .and_then(|value| value.to_str())
        .ok_or_else(|| format!("Failed to derive photo slug from {}", path.display()))?;

    if stem == "post" || stem == "index" {
        let parent = path
            .parent()
            .and_then(|value| value.file_name())
            .and_then(|value| value.to_str())
            .ok_or_else(|| format!("Failed to derive photo slug from {}", path.display()))?;

        Ok(parent.to_string())
    } else {
        Ok(stem.to_string())
    }
}

fn resolve_asset_path(
    content_root: &Path,
    base_dir: &Path,
    raw_path: &str,
) -> Result<String, String> {
    if raw_path.starts_with("/assets/") {
        return Ok(raw_path.to_string());
    }

    if raw_path.starts_with("http://") || raw_path.starts_with("https://") {
        return Ok(raw_path.to_string());
    }

    let sanitized_relative_path = sanitize_relative_path(raw_path)?;
    let absolute_path = base_dir.join(&sanitized_relative_path);

    if !absolute_path.exists() {
        return Err(format!("Missing asset {}", absolute_path.display()));
    }

    let assets_root = content_root.parent().unwrap_or(content_root);
    let relative_to_content = absolute_path.strip_prefix(assets_root).map_err(|_| {
        format!(
            "Asset {} is outside of {}",
            absolute_path.display(),
            assets_root.display()
        )
    })?;

    let web_path = relative_to_content
        .to_string_lossy()
        .replace(std::path::MAIN_SEPARATOR, "/");

    Ok(format!("/assets/{}", web_path))
}

fn resolve_markdown_image_paths(
    markdown: &str,
    content_root: &Path,
    base_dir: &Path,
) -> Result<String, String> {
    let mut normalized = Vec::new();
    let mut in_fence = false;

    for line in markdown.lines() {
        let trimmed = line.trim_start();

        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
            normalized.push(line.to_string());
            continue;
        }

        if in_fence {
            normalized.push(line.to_string());
        } else {
            normalized.push(resolve_markdown_image_paths_in_line(
                line,
                content_root,
                base_dir,
            )?);
        }
    }

    Ok(normalized.join("\n"))
}

fn resolve_markdown_image_paths_in_line(
    line: &str,
    content_root: &Path,
    base_dir: &Path,
) -> Result<String, String> {
    let mut output = String::with_capacity(line.len());
    let mut cursor = 0;

    while let Some(image_start) = line[cursor..].find("![") {
        let image_start = cursor + image_start;
        let Some(target_start) = line[image_start + 2..].find("](") else {
            break;
        };
        let target_start = image_start + 2 + target_start + 2;

        let Some(target_end) = line[target_start..].find(')') else {
            break;
        };
        let target_end = target_start + target_end;

        output.push_str(&line[cursor..target_start]);
        output.push_str(&resolve_markdown_image_target(
            &line[target_start..target_end],
            content_root,
            base_dir,
        )?);
        output.push(')');

        cursor = target_end + 1;
    }

    output.push_str(&line[cursor..]);
    Ok(output)
}

fn resolve_markdown_image_target(
    target: &str,
    content_root: &Path,
    base_dir: &Path,
) -> Result<String, String> {
    let leading_len = target.len() - target.trim_start().len();
    let leading = &target[..leading_len];
    let rest = &target[leading_len..];

    if let Some(stripped) = rest.strip_prefix('<') {
        if let Some(destination_end) = stripped.find('>') {
            let destination = &stripped[..destination_end];
            let suffix = &stripped[destination_end + 1..];

            return Ok(format!(
                "{}<{}>{}",
                leading,
                resolve_markdown_image_destination(destination, content_root, base_dir)?,
                suffix
            ));
        }
    }

    let destination_end = rest
        .char_indices()
        .find(|(_, value)| value.is_whitespace())
        .map(|(index, _)| index)
        .unwrap_or(rest.len());

    let destination = &rest[..destination_end];
    let suffix = &rest[destination_end..];

    Ok(format!(
        "{}{}{}",
        leading,
        resolve_markdown_image_destination(destination, content_root, base_dir)?,
        suffix
    ))
}

fn resolve_markdown_image_destination(
    destination: &str,
    content_root: &Path,
    base_dir: &Path,
) -> Result<String, String> {
    if should_leave_markdown_image_destination(destination) {
        return Ok(destination.to_string());
    }

    let path_end = destination.find(['?', '#']).unwrap_or(destination.len());
    let path = &destination[..path_end];
    let suffix = &destination[path_end..];

    Ok(format!(
        "{}{}",
        resolve_asset_path(content_root, base_dir, path)?,
        suffix
    ))
}

fn should_leave_markdown_image_destination(destination: &str) -> bool {
    destination.is_empty()
        || destination.starts_with('/')
        || destination.starts_with('#')
        || destination.starts_with("http://")
        || destination.starts_with("https://")
        || destination.starts_with("mailto:")
        || destination.starts_with("tel:")
        || destination.starts_with("data:")
}

fn sanitize_relative_path(raw_path: &str) -> Result<PathBuf, String> {
    let path = Path::new(raw_path.trim());

    if raw_path.trim().is_empty() {
        return Err("Image path cannot be empty".to_string());
    }

    let mut clean = PathBuf::new();

    for component in path.components() {
        match component {
            Component::Normal(value) => clean.push(value),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                return Err(format!("Invalid image path {}", raw_path));
            }
        }
    }

    if clean.as_os_str().is_empty() {
        return Err(format!("Invalid image path {}", raw_path));
    }

    Ok(clean)
}

fn latest_modified_date(paths: &[PathBuf]) -> Result<String, String> {
    let latest = paths
        .iter()
        .filter_map(|path| {
            fs::metadata(path)
                .ok()
                .and_then(|metadata| metadata.modified().ok())
        })
        .max()
        .unwrap_or(SystemTime::now());

    let datetime: chrono::DateTime<chrono::Utc> = latest.into();
    Ok(datetime.format("%Y-%m-%d").to_string())
}

fn humanize_slug(slug: &str) -> String {
    slug.split(['-', '_'])
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                Some(first) => {
                    let mut output = first.to_uppercase().collect::<String>();
                    output.push_str(chars.as_str());
                    output
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_content_dir(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock should be after unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "smit-codes-loader-test-{}-{}-{}",
            std::process::id(),
            name,
            unique
        ))
    }

    fn frontmatter() -> &'static str {
        "---\ntitle: Test Post\ndate: 2026-05-08\ntags: []\nlinks: []\ndescription: Test\n---\n"
    }

    #[test]
    fn load_content_from_dir_normalizes_relative_markdown_images() {
        let content_root = temp_content_dir("relative-images");
        let writing_root = content_root.join("writing");
        let image_dir = writing_root.join("imgs");
        fs::create_dir_all(&image_dir).expect("create image dir");
        fs::write(image_dir.join("codexexamplescreenshot.png"), b"png").expect("write image");
        fs::write(
            writing_root.join("codex-is-awesome.md"),
            format!(
                "{}![codex cli screenshot](imgs/codexexamplescreenshot.png)\n",
                frontmatter()
            ),
        )
        .expect("write markdown");

        let posts = load_content_from_dir(&writing_root, ContentKind::Writing).expect("load posts");

        assert_eq!(posts.len(), 1);
        assert_eq!(
            posts[0].markdown,
            "![codex cli screenshot](/assets/writing/imgs/codexexamplescreenshot.png)"
        );

        fs::remove_dir_all(content_root).expect("remove temp content");
    }

    #[test]
    fn markdown_image_normalization_preserves_external_urls_titles_and_code_fences() {
        let content_root = temp_content_dir("mixed-images");
        let writing_root = content_root.join("writing");
        let image_dir = writing_root.join("imgs");
        fs::create_dir_all(&image_dir).expect("create image dir");
        fs::write(image_dir.join("local.png"), b"png").expect("write image");

        let markdown = "\
![external](https://example.com/image.png)
![local](imgs/local.png \"Local image\")

```markdown
![example](imgs/missing.png)
```
";

        let normalized = resolve_markdown_image_paths(markdown, &writing_root, &writing_root)
            .expect("normalize");

        assert_eq!(
            normalized,
            "\
![external](https://example.com/image.png)
![local](/assets/writing/imgs/local.png \"Local image\")

```markdown
![example](imgs/missing.png)
```"
        );

        fs::remove_dir_all(content_root).expect("remove temp content");
    }

    #[test]
    fn markdown_image_normalization_rejects_missing_relative_assets() {
        let content_root = temp_content_dir("missing-image");
        let writing_root = content_root.join("writing");
        fs::create_dir_all(&writing_root).expect("create writing dir");

        let error = resolve_markdown_image_paths(
            "![missing](imgs/missing.png)",
            &writing_root,
            &writing_root,
        )
        .expect_err("missing image should fail");

        assert!(error.contains("Missing asset"));

        fs::remove_dir_all(content_root).expect("remove temp content");
    }
}
