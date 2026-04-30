use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rustle", about = "a fast file organizer")]
struct Cli {
    #[arg(short, long, value_name = "DIR", default_value = ".")]
    path: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if !cli.path.exists() {
        anyhow::bail!("Path {:?} not exists!", cli.path);
    }

    println!("Scanning: {:?}...", cli.path.canonicalize()?);

    let entries = std::fs::read_dir(&cli.path)
        .with_context(|| format!("Failed to read folder {:?}", cli.path))?;

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_type = if entry.path().is_dir() { "" } else { "" };

        println!("{} {:?}", file_type, file_name);
    }

    Ok(())
}
