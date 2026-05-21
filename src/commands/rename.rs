use anyhow::{bail, Context, Result};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

pub fn rename(files: &[impl AsRef<Path>], pattern: &str) -> Result<()> {
    if files.is_empty() {
        bail!("No files specified");
    }

    let mut plan: Vec<(PathBuf, PathBuf)> = Vec::new();
    let mut targets: HashSet<PathBuf> = HashSet::new();

    for (i, file) in files.iter().enumerate() {
        let path = file.as_ref();
        if !path.exists() {
            eprintln!("skip (not found): {}", path.display());
            continue;
        }

        let name = path.file_stem().unwrap_or_default().to_string_lossy();
        let ext = path.extension().map(|e| e.to_string_lossy().into_owned()).unwrap_or_default();

        #[allow(clippy::literal_string_with_formatting_args)]
        let new_name = pattern
            .replace("{n}", &format!("{:03}", i + 1))
            .replace("{name}", &name)
            .replace("{ext}", &ext);

        let new_path = path.with_file_name(if new_name.contains('.') {
            new_name
        } else if !ext.is_empty() {
            format!("{new_name}.{ext}")
        } else {
            new_name
        });

        if new_path.exists() || targets.contains(&new_path) {
            bail!("conflict: target already exists: {}", new_path.display());
        }
        targets.insert(new_path.clone());
        plan.push((path.to_path_buf(), new_path));
    }

    // Execute with rollback on failure
    let mut done: Vec<usize> = Vec::new();
    for (idx, (src, dst)) in plan.iter().enumerate() {
        if let Err(e) = fs::rename(src, dst) {
            eprintln!("error renaming {} -> {}: {e}", src.display(), dst.display());
            // Rollback completed renames
            for &i in done.iter().rev() {
                let (orig_src, orig_dst) = &plan[i];
                if let Err(re) = fs::rename(orig_dst, orig_src) {
                    eprintln!("  rollback failed: {} -> {}: {re}", orig_dst.display(), orig_src.display());
                }
            }
            return Err(e).context("rename failed, changes rolled back");
        }
        println!("{} -> {}", src.display(), dst.display());
        done.push(idx);
    }

    Ok(())
}
