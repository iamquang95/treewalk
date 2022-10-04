use crate::{Arena, NodeId};
use walkdir::{DirEntry, WalkDir};

#[derive(Debug)]
pub struct Dir {
    pub name: String,
    pub is_folder: bool,
    pub is_open: bool,
}

impl Dir {
    pub fn new(name: String, is_folder: bool, is_open: bool) -> Self {
        Dir {
            name,
            is_folder,
            is_open,
        }
    }

    pub fn build_dir(path: &str) -> anyhow::Result<Arena<Dir>> {
        let root = Dir::new(path.to_owned(), true, true);
        let mut tree = Arena::<Dir>::new(root);
        let root_id = tree.root_id();
        Dir::traverse_build_dir(&mut tree, root_id, path, 0)?;
        Ok(tree)
    }

    pub fn render_tree(tree: &Arena<Dir>) -> anyhow::Result<()> {
        Dir::traverse_render_tree(tree, tree.root_id(), &mut vec![])
    }

    fn traverse_render_tree(
        tree: &Arena<Dir>,
        node_id: NodeId,
        borders: &mut Vec<bool>,
    ) -> anyhow::Result<()> {
        let node = tree.get_node(node_id)?;
        for (idx, child) in node.children.iter().enumerate() {
            let child_node = tree.get_node(*child)?;
            let child_name = &child_node.val.name;
            let is_last_idx = idx + 1 == node.children.len();
            for border in borders.iter() {
                if *border {
                    print!("    ");
                } else {
                    print!("   │");
                }
            }
            if is_last_idx {
                println!("   └── {}", child_name);
            } else {
                println!("   │── {}", child_name);
            }

            if child_node.val.is_folder {
                borders.push(is_last_idx);
                Dir::traverse_render_tree(tree, *child, borders)?;
                borders.pop();
            }
        }
        Ok(())
    }

    fn traverse_build_dir(
        tree: &mut Arena<Dir>,
        node_id: NodeId,
        path: &str,
        depth: u64,
    ) -> anyhow::Result<()> {
        let dir = WalkDir::new(path).max_depth(1);
        for entry in dir {
            let entry = entry?;
            let entry_path = format!("{}", entry.path().display());
            if &entry_path != &path {
                let is_folder = Dir::is_folder(&entry)?;
                let file_name = entry.file_name().to_str().unwrap();

                let new_dir = Dir::new(file_name.to_owned(), is_folder, true);
                let new_node_id = tree.new_node(new_dir, node_id)?;

                if is_folder {
                    Dir::traverse_build_dir(tree, new_node_id, &entry_path, depth + 1)?;
                }
            }
        }
        Ok(())
    }

    fn is_folder(entry: &DirEntry) -> anyhow::Result<bool> {
        let metadata = entry.metadata()?;
        Ok(metadata.is_dir())
    }
}
