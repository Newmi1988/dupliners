use std::path::Path;

use crate::{file_duplicates::FileDuplicates, folder_explorer::visit_dirs};

mod file_duplicates;
mod folder_explorer;

fn main() {
    let mut t = FileDuplicates::default();

    // let line: String = "Test".to_string();
    let root = Path::new("./src");

    t.recurse_fs(root).expect("IO Error");
    t.prune();
    print!("{:?}", t.dupes);
}
