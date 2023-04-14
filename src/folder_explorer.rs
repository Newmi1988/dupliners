use std::fs::{self};
use std::io;
use std::path::{Path, PathBuf};

pub fn visit_dirs(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut dirs: Vec<PathBuf> = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path)?;
            } else {
                dirs.push(entry.path());
            }
        }
    }
    Ok(dirs)
}

// test
// test
// test

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_visit_dirs() {
        let tmp_dir = tempdir().expect("Error creating tmp dir");
        let file_path = tmp_dir.path().join("my-temporary-note.txt");
        let mut res: Vec<PathBuf> = Vec::new();
        res.push(file_path.clone().to_path_buf());
        let _file = File::create(file_path).expect("Error creating file");

        assert_eq!(
            visit_dirs(tmp_dir.path()).expect("Error reading tmp file folder"),
            res
        )
    }
}
