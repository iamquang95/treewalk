mod dir;
mod tree;

use walkdir::{DirEntry, WalkDir};

use crate::dir::Dir;
use crate::tree::{Arena, NodeId};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path
    #[arg(short, long, default_value_t = String::from("."))]
    path: String,
}

fn is_dir(entry: &DirEntry) -> anyhow::Result<bool> {
    let metadata = entry.metadata()?;
    Ok(metadata.is_dir())
}

fn traverse_dir(
    tree: &mut Arena<Dir>,
    node_id: NodeId,
    path: &str,
    depth: u64,
) -> anyhow::Result<()> {
    let dir = WalkDir::new(path).max_depth(1);
    for entry in dir {
        let entry = entry?;
        let is_folder = is_dir(&entry)?;
        let file_name = entry.file_name().to_str().unwrap();

        let new_dir = Dir::new(file_name.to_owned(), is_folder, true);
        let new_node_id = tree.new_node(new_dir, node_id)?;

        if is_folder {
            let entry_path = format!("{}", entry.path().display());
            if &entry_path != &path {
                traverse_dir(tree, new_node_id, &entry_path, depth + 1)?;
            }
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let root = Dir::new(args.path.clone(), true, true);
    let mut tree = Arena::<Dir>::new(root);
    let root_id = tree.root_id();
    traverse_dir(&mut tree, root_id, &args.path, 0)?;
    dbg!(tree.nodes.len());
    let root = tree.get_node(0).unwrap();
    for node in &root.children {
        dbg!(tree.get_node(*node));
    }
    Ok(())
}
