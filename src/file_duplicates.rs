use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use crate::folder_explorer::visit_dirs;

pub(crate) struct FileDuplicates {
    pub(crate) dupes: HashMap<String, HashMap<PathBuf, Vec<u32>>>,
}

fn hash_string(line: &String) -> Result<String, Box<dyn std::error::Error>> {
    let mut hasher = Sha256::new();
    // write input message
    hasher.update(line.as_bytes());
    // read hash digest and consume hasher
    Ok(format!("{:X}", hasher.finalize()))
}

impl FileDuplicates {
    fn new() -> FileDuplicates {
        FileDuplicates {
            dupes: HashMap::new(),
        }
    }

    fn add(&mut self, line: &String, number: u32, file: &Path) -> () {
        let hash = hash_string(line).expect("Error hashing line");
        // BUG: The first element of the vec is always duplicated
        // UPDATE: Possibly fixed
        self.dupes
            .entry(hash)
            // .or_insert(HashMap::from([(file.to_path_buf(), vec![number])]))
            .or_insert(HashMap::from([(file.to_path_buf(), vec![])]))
            .entry(file.to_path_buf())
            .or_insert(Vec::new())
            .push(number);
    }

    pub(crate) fn from_file(&mut self, filepath: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(filepath)?;
        let reader = BufReader::new(file);

        Ok(for (mut i, line) in reader.lines().enumerate() {
            let l = line?;
            i += 1;
            if l.trim().len() > 1 {
                self.add(&l, i.try_into().unwrap(), filepath);
            }
        })
    }

    pub(crate) fn prune(&mut self) {
        // TODO: Fix the logic. Discarde all the items where the summed up length of all entries is < 2
        self.dupes.retain(|_, v| v.len() > 1)
    }

    pub(crate) fn recurse_fs(&mut self, filepath: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let paths = visit_dirs(filepath).expect("IO Error");
        for path in paths {
            // let p = path;
            self.from_file(&path);
        }

        Ok(())
    }
}

impl Default for FileDuplicates {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_string() {
        let s = "Test".to_string();
        assert_eq!(
            hash_string(&s).expect("Error"),
            "532EAABD9574880DBF76B9B8CC00832C20A6EC113D682299550D7A6E0F345E25".to_string()
        )
    }
}
