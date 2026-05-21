mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ftidy", about = "File tidying: dedup, rename, organize")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, ValueEnum)]
pub enum KeepStrategy {
    /// Keep the file with the oldest modification time
    Oldest,
    /// Keep the file with the newest modification time
    Newest,
    /// Keep the file with the smallest path (lexicographic)
    Path,
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
        /// Strategy for which duplicate to keep
        #[arg(long, value_enum, default_value = "oldest")]
        keep: KeepStrategy,
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
        Commands::Dedup { path, delete, keep } => commands::dedup(&path, delete, &keep),
        Commands::Rename { files, pattern } => commands::rename(&files, &pattern),
    }
}
