use anyhow::Result;
use clap::Parser;
use colored::*;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "rustle", about = "A fast file organizer")]
struct Cli {
    #[arg(short, long, value_name = "DIR", default_value = ".")]
    path: PathBuf,
}

fn get_categories() -> HashMap<&'static str, (&'static str, &'static str)> {
    let mut m = HashMap::new();
    
    let code = ("Code", "");
    m.insert("rs", code);
    m.insert("py", ("Code", ""));
    m.insert("js", ("Code", ""));
    m.insert("ts", ("Code", ""));
    m.insert("cpp", ("Code", ""));
    m.insert("c", ("Code", ""));
    m.insert("h", ("Code", ""));
    m.insert("hpp", ("Code", ""));
    m.insert("go", ("Code", ""));
    m.insert("java", ("Code", ""));
    m.insert("rb", ("Code", ""));
    m.insert("php", ("Code", ""));
    m.insert("swift", ("Code", ""));
    
    let web = ("Web", "󰖟");
    m.insert("html", web);
    m.insert("css", ("Web", ""));
    m.insert("scss", ("Web", ""));
    
    let config = ("Config", "");
    m.insert("toml", config);
    m.insert("yaml", config);
    m.insert("yml", config);
    m.insert("json", ("Config", ""));
    m.insert("xml", ("Config", "󰗀"));
    m.insert("env", ("Config", ""));
    
    let db = ("Database", "");
    m.insert("sql", db);
    m.insert("db", db);
    m.insert("sqlite", db);

    let script = ("Script", "󱆃");
    m.insert("sh", script);
    m.insert("bash", script);
    m.insert("bat", ("Script", ""));
    m.insert("ps1", ("Script", ""));

    let img = ("Images", "󰈟");
    m.insert("jpg", img);
    m.insert("jpeg", img);
    m.insert("png", ("Images", "󰙏"));
    m.insert("svg", ("Images", "󰜡"));
    
    let doc = ("Documents", "󰈙");
    m.insert("pdf", ("Documents", ""));
    m.insert("md", ("Documents", ""));
    m.insert("txt", doc);
    m.insert("docx", doc);

    let bin = ("Binary", "");
    m.insert("exe", bin);
    m.insert("so", bin);
    m.insert("dll", bin);
    m.insert("lock", ("System", ""));
    
    m
}

fn print_tree(
    path: &Path,
    prefix: &str,
    categories: &HashMap<&str, (&str, &str)>,
    is_last: bool,
) -> Result<()> {
    let file_name = path.file_name().unwrap_or_default().to_string_lossy();
    let tree_symbol = if is_last { "└──" } else { "├──" };

    if path.is_dir() {
        println!("{}{}{}", prefix, tree_symbol, format!("  {}", file_name).bold());

        let entries: Vec<_> = fs::read_dir(path)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                let n = e.file_name();
                let s = n.to_string_lossy();
                s != ".git" && s != "target" && s != ".idea" && s != "node_modules"
            })
            .collect();

        let count = entries.len();
        for (i, entry) in entries.into_iter().enumerate() {
            let next_is_last = i == count - 1;
            let next_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            print_tree(&entry.path(), &next_prefix, categories, next_is_last)?;
        }
    } else {
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
        let (cat_name, icon) = categories.get(ext.as_str()).unwrap_or(&("Others", "󰈔"));

        let mut name_str = file_name.to_string();
        if name_str.len() > 30 {
            name_str.truncate(27);
            name_str.push_str("..");
        }

        print!("{}{}{} ", prefix, tree_symbol, icon);
        
        let used_width = name_str.chars().count();
        let padding = if used_width < 30 { 30 - used_width } else { 1 };
        
        print!("{}{}", name_str, " ".repeat(padding));
        println!("{}  {}", "", format!("[ {} ]", cat_name).italic());
    }
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let categories = get_categories();

    if !cli.path.exists() {
        anyhow::bail!("Path {:?} does not exist!", cli.path);
    }

    let absolute_path = cli.path.canonicalize()?;
    println!("\n󰉋 {}\n", absolute_path.display().to_string().bold());

    let entries: Vec<_> = fs::read_dir(&absolute_path)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let n = e.file_name();
            let s = n.to_string_lossy();
            s != ".git" && s != "target" && s != ".idea" && s != "node_modules"
        })
        .collect();

    let count = entries.len();
    for (i, entry) in entries.into_iter().enumerate() {
        print_tree(&entry.path(), "", &categories, i == count - 1)?;
    }

    println!("\n--- Scanning complete ---");
    Ok(())
}
