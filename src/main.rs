mod tree;

use walkdir::{DirEntry, WalkDir};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path
    #[arg(short, long)]
    path: String,
}

fn is_dir(entry: &DirEntry) -> anyhow::Result<bool> {
    let metadata = entry.metadata()?;
    Ok(metadata.is_dir())
}

fn traverse_dir(path: &str, prefix: &str, depth: u64) -> anyhow::Result<()> {
    let dir = WalkDir::new(path).max_depth(1);
    for entry in dir {
        let entry = entry?;
        if is_dir(&entry)? {
            let entry_path = format!("{}", entry.path().display());
            println!("{} {}", prefix, &entry.file_name().to_str().unwrap());
            if &entry_path != &path {
                traverse_dir(&entry_path, &format!("{}--", prefix), depth + 1)?;
            }
        } else {
            println!("{} {}", prefix, entry.file_name().to_str().unwrap());
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    traverse_dir(&args.path, "", 0)?;
    Ok(())
}
