use std::path::Path;

use crate::file_duplicates::FileDuplicates;

mod file_duplicates;

fn main() {
    let mut t = FileDuplicates::default();

    // let line: String = "Test".to_string();

    // t.add(&line, 1);
    let f = Path::new("./README.md");
    t.from_file(&f);
    t.prune();
    print!("{:?}", t.dupes);


}
