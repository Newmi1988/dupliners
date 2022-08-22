use std::path::Path;

use crate::{file_duplicates::FileDuplicates, folder_explorer::visit_dirs};

mod file_duplicates;
mod folder_explorer;

fn main() {
    let mut t = FileDuplicates::default();

    // let line: String = "Test".to_string();

    // t.add(&line, 1);
    let f = Path::new("./README.md");
    t.from_file(&f).expect("Error reading file");
    t.prune();
    print!("{:?}", t.dupes);

    let root = Path::new("./src");
    let paths = visit_dirs(root).expect("IO Error");
    println!("{:?}", &paths)
}
