use hex_literal::hex;
use sha2::{digest::generic_array::GenericArray, Digest, Sha256, Sha512};
use std::{collections::HashMap, fs::File, hash, str::Bytes, path::Path, io::{BufReader, BufRead}};
use typenum::U256;

struct FileDuplicates {
    dupes: HashMap<String, Vec<u32>>,
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

    fn add(&mut self, line: &String, number: u32) -> () {
        let hash = hash_string(line).expect("Error hashing line");
        self.dupes.entry(hash).or_insert(Vec::new()).push(number);
    }

    fn from_file(&mut self, filepath: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(filepath)?;
        let reader = BufReader::new(file);
    
        Ok(for line in reader.lines() {
            self.add(&line?, 12);
        })
    }
}

impl Default for FileDuplicates {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let mut t = FileDuplicates::default();

    // let line: String = "Test".to_string();

    // t.add(&line, 1);
    let f = Path::new("./README.md");
    t.from_file(&f);

    print!("{:?}", t.dupes);
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
