mod dir;
mod tree;

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

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut tree = Dir::build_dir(&args.path)?;
    Dir::render_tree(&tree)?;
    Ok(())
}
