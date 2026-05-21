use anyhow::{bail, Result};
use std::fs;
use std::path::Path;

pub fn rename(files: &[impl AsRef<Path>], pattern: &str) -> Result<()> {
    if files.is_empty() {
        bail!("No files specified");
    }

    for (i, file) in files.iter().enumerate() {
        let path = file.as_ref();
        if !path.exists() {
            eprintln!("skip (not found): {}", path.display());
            continue;
        }

        let name = path.file_stem().unwrap_or_default().to_string_lossy();
        let ext = path.extension().map(|e| e.to_string_lossy().to_string()).unwrap_or_default();

        let new_name = pattern
            .replace("{n}", &format!("{:03}", i + 1))
            .replace("{name}", &name)
            .replace("{ext}", &ext);

        let new_path = path.with_file_name(if new_name.contains('.') {
            new_name
        } else if !ext.is_empty() {
            format!("{}.{}", new_name, ext)
        } else {
            new_name
        });

        println!("{} -> {}", path.display(), new_path.display());
        fs::rename(path, &new_path)?;
    }

    Ok(())
}
