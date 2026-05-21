mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ftidy", about = "File tidying: dedup, rename, organize")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Find and remove duplicate files by content hash
    Dedup {
        /// Directory to scan
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Actually delete duplicates (default: dry-run)
        #[arg(long)]
        delete: bool,
    },
    /// Batch rename files with pattern
    Rename {
        /// Files to rename (glob or list)
        files: Vec<PathBuf>,
        /// Pattern: use {n} for sequence, {name} for original name, {ext} for extension
        #[arg(short, long)]
        pattern: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Dedup { path, delete } => commands::dedup(&path, delete),
        Commands::Rename { files, pattern } => commands::rename(&files, &pattern),
    }
}
