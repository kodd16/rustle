use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rustle", about = "A fast file organizer")]
struct Cli {
    #[arg(short, long, value_name = "DIR", default_value = ".")]
    path: PathBuf,
}

fn get_categories() -> HashMap<&'static str, (&'static str, &'static str)> {
    let mut m = HashMap::new();

    m.insert("rs", ("Code", ""));
    m.insert("py", ("Code", ""));
    m.insert("js", ("Code", ""));
    m.insert("cpp", ("Code", ""));

    m.insert("jpg", ("Images", "󰈟"));
    m.insert("jpeg", ("Images", "󰈟"));
    m.insert("png", ("Images", "󰙏"));

    m.insert("pdf", ("Documents", ""));
    m.insert("txt", ("Documents", "󰈙"));
    m.insert("md", ("Documents", ""));

    m.insert("toml", ("Config", ""));
    m.insert("json", ("Config", ""));
    m
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let categories = get_categories();

    if !cli.path.exists() {
        anyhow::bail!("Path {:?} not exists!", cli.path);
    }

    println!("Scanning: {:?}...", cli.path.canonicalize()?);

    let entries = std::fs::read_dir(&cli.path)
        .with_context(|| format!("Failed to read folder {:?}", cli.path))?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");

            let (category, icon) = categories.get(extension).unwrap_or(&("Others", "󰈔"));

            let file_name = path.file_name().unwrap().to_string_lossy();

            println!("{} {:<20} -> {}", icon, file_name, category.bold());
        }
    }

    Ok(())
}
