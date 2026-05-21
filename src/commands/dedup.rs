use anyhow::Result;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn dedup(path: &Path, delete: bool) -> Result<()> {
    let mut seen: HashMap<String, PathBuf> = HashMap::new();
    let mut dupes: Vec<(PathBuf, PathBuf)> = Vec::new();

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }
        let file_path = entry.into_path();
        let hash = hash_file(&file_path)?;
        if let Some(original) = seen.get(&hash) {
            dupes.push((file_path, original.clone()));
        } else {
            seen.insert(hash, file_path);
        }
    }

    if dupes.is_empty() {
        println!("No duplicates found.");
        return Ok(());
    }

    for (dupe, original) in &dupes {
        println!("dup: {} == {}", dupe.display(), original.display());
        if delete {
            fs::remove_file(dupe)?;
            println!("  deleted: {}", dupe.display());
        }
    }

    if !delete {
        println!("\n{} duplicates found. Use --delete to remove them.", dupes.len());
    }

    Ok(())
}

fn hash_file(path: &Path) -> Result<String> {
    let data = fs::read(path)?;
    let hash = Sha256::digest(&data);
    Ok(format!("{:x}", hash))
}
