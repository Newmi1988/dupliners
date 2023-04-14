use std::path::Path;

use crate::file_duplicates::FileDuplicates;

mod file_duplicates;
mod folder_explorer;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short,long, default_value_t = String::from("./src"))]
    path: String,
}

fn main() {
    let args = Args::parse();
    let mut t = FileDuplicates::default();

    let root = Path::new(&args.path);

    t.recurse_fs(root).expect("IO Error");
    // print!("{:?}", &t.dupes);
    t.prune();
    print!("{:?}", t.dupes);
}
