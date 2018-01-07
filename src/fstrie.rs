use std::fs::File;
use std::path::PathBuf;
use std::io::{BufRead, BufReader};
use err::{ErrorKind, Result};

pub struct Database {
    root: String,
}

impl Database {
    pub fn new(root: &str) -> Result<Self> {
        Ok(Self {
            root: root.to_owned(),
        })
    }

    pub fn lookup(&self, key: &str) -> Result<Vec<Vec<u8>>> {
        let mut path = PathBuf::from(&self.root);
        if !path.is_dir() {
            return Err(ErrorKind::RootNotDir.into());
        }
        for c in key.chars().flat_map(|c| c.to_lowercase()) {
            if c.is_digit(10) || c.is_lowercase() {
                path.push(c.to_string());
            } else {
                path.push("symbols");
            }
            if !path.is_dir() {
                break;
            }
        }
        if path.is_dir() {
            return Ok(Vec::new());
        }
        let file = File::open(path)?;
        let needle = format!("{}:", key).as_bytes().to_owned();
        let mut result = Vec::new();
        let mut buf = Vec::new();
        let mut rd = BufReader::new(file);
        while rd.read_until(b'\n', &mut buf)? > 0 {
            if buf.starts_with(&needle) {
                result.push(buf[needle.len()..buf.len()-1].to_owned());
            }
            buf.clear();
        }
        Ok(result)
    }
}
