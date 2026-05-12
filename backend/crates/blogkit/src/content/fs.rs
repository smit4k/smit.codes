use std::fs;
use std::path::{Path, PathBuf};

pub fn read_text_file(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("Failed to read {}: {}", path.display(), e))
}

pub fn read_markdown_file(path: &Path) -> Result<String, String> {
    read_text_file(path)
}

pub fn collect_files_with_extensions(
    root: &Path,
    extensions: &[&str],
) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();

    if !root.exists() {
        return Err(format!("Directory {} does not exist", root.display()));
    }
    for entry in fs::read_dir(root).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.is_dir() {
            files.extend(collect_files_with_extensions(&path, extensions)?);
        } else if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| {
                extensions
                    .iter()
                    .any(|candidate| ext.eq_ignore_ascii_case(candidate))
            })
        {
            files.push(path);
        }
    }

    Ok(files)
}

pub fn collect_markdown_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    collect_files_with_extensions(root, &["md"])
}
