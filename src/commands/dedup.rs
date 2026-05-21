use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn dedup(path: &Path, delete: bool) -> Result<()> {
    let mut size_map: HashMap<u64, Vec<PathBuf>> = HashMap::new();

    for entry in WalkDir::new(path).follow_links(false) {
        let entry = entry.context("failed to read directory entry")?;
        if !entry.file_type().is_file() {
            continue;
        }
        let meta = entry.metadata().context("failed to read metadata")?;
        if meta.len() == 0 {
            continue;
        }
        size_map.entry(meta.len()).or_default().push(entry.into_path());
    }

    let mut dupes: Vec<(PathBuf, PathBuf)> = Vec::new();
    for files in size_map.values_mut() {
        if files.len() < 2 {
            continue;
        }
        files.sort();
        let mut seen: HashMap<String, PathBuf> = HashMap::new();
        for file_path in files.iter() {
            let hash = hash_file(file_path)
                .with_context(|| format!("failed to hash {}", file_path.display()))?;
            if let Some(original) = seen.get(&hash) {
                dupes.push((file_path.clone(), original.clone()));
            } else {
                seen.insert(hash, file_path.clone());
            }
        }
    }

    dupes.sort();

    if dupes.is_empty() {
        println!("No duplicates found.");
        return Ok(());
    }

    for (dupe, original) in &dupes {
        println!("dup: {} == {}", dupe.display(), original.display());
    }

    if delete {
        println!("\n{} file(s) will be deleted. Continue? [y/N] ", dupes.len());
        let mut answer = String::new();
        io::stdin().lock().read_line(&mut answer)?;
        if !answer.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            return Ok(());
        }
        for (dupe, _) in &dupes {
            fs::remove_file(dupe)?;
            println!("  deleted: {}", dupe.display());
        }
    } else {
        println!("\n{} duplicates found. Use --delete to remove them.", dupes.len());
    }

    Ok(())
}

fn hash_file(path: &Path) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}
