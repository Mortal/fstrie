use std::fs::File;
use std::path::PathBuf;
use std::io::{Read, BufRead, BufReader, Seek, SeekFrom};
use err::{ErrorKind, Result};

pub struct Database {
    root: String,
}

struct BlockSearcher {
    file: File,
    nbytes: u64,
    block_size: usize,
    lo: u64,
    hi: u64,
    buf: Vec<u8>,
}

impl BlockSearcher {
    fn new(mut file: File, block_size: usize) -> Result<Self> {
        let nbytes = file.seek(SeekFrom::End(0))?;
        let mut buf = Vec::new();
        buf.resize(block_size, b'\x00');
        let mut res = Self {
            file: file,
            nbytes: nbytes,
            block_size: block_size,
            lo: 0,
            hi: 0,
            buf: buf,
        };
        res.hi = res.nblocks();
        Ok(res)
    }

    fn done(&self) -> bool {
        self.lo + 1 >= self.hi
    }

    fn nblocks(&self) -> u64 {
        let bs = self.block_size as u64;
        (self.nbytes + bs - 1) / bs
    }

    fn mid(&self) -> u64 {
        self.lo + (self.hi - self.lo) / 2
    }

    fn lower(&mut self) {
        self.hi = self.mid();
    }

    fn higher(&mut self) {
        self.lo = self.mid() + 1;
    }

    fn trimmed(&self) -> &[u8] {
        let mut i = 0;
        if self.mid() > 0 {
            i += 1;
            while i < self.buf.len() && self.buf[i-1] != b'\n' {
                i += 1;
            }
        }
        let mut j = self.buf.len();
        if self.mid() + 1 != self.nblocks() {
            j -= 1;
            while j > i && self.buf[j] != b'\n' {
                j -= 1
            }
        }
        if i == j {
            panic!("Lines too long for buffer size");
        }
        &self.buf[i..j]
    }

    fn read(&mut self) -> Result<&[u8]> {
        let mid = self.mid();
        self.file.seek(SeekFrom::Start(mid * (self.block_size as u64))).unwrap();
        self.buf.resize(self.block_size, b'\x00');
        let n = self.file.read(&mut self.buf).unwrap();
        self.buf.resize(n, b'\x00');
        Ok(self.trimmed())
    }

    fn into_inner(mut self) -> Result<File> {
        // If self.lo == self.hi,
        // then there might be a single result cut off at a block boundary,
        // in which case we want to start reading from block self.hi-1.
        if self.lo == self.hi {
            assert!(self.hi > 0);
            self.lo = self.hi - 1;
        }
        self.read()?;
        let i = self.buf.iter().position(|c| *c == b'\n').unwrap() as u64;
        let mid = self.mid();
        let pos = mid * (self.block_size as u64) + i + 1;
        self.file.seek(SeekFrom::Start(pos)).unwrap();
        Ok(self.file)
    }
}

impl Database {
    pub fn new(root: &str) -> Result<Self> {
        Ok(Self {
            root: root.to_owned(),
        })
    }

    fn lookup_path(&self, key: &str) -> Result<PathBuf> {
        let mut path = PathBuf::from(&self.root);
        if !path.exists() {
            return Err(ErrorKind::RootDoesNotExist.into());
        }
        for c in key.chars().flat_map(|c| c.to_lowercase()) {
            if !path.is_dir() {
                break;
            }
            if c.is_digit(10) || c.is_lowercase() {
                path.push(c.to_string());
            } else {
                path.push("symbols");
            }
        }
        Ok(path)
    }

    pub fn lookup(&self, key: &str) -> Result<Vec<Vec<u8>>> {
        let path = self.lookup_path(key)?;
        if path.is_dir() {
            return Ok(Vec::new());
        }
        let file = File::open(path)?;
        let mut searcher = BlockSearcher::new(file, (2 as usize).pow(16))?;
        let needle = format!("{}:", key).as_bytes().to_owned();
        while !searcher.done() {
            let (i, l_len) = {
                let l = searcher.read()?.split(|c| *c == b'\n').collect::<Vec<_>>();
                (match l.binary_search(&&needle[..]) {
                    Ok(i) => i,
                    Err(i) => i,
                }, l.len())
            };
            if i == 0 {
                searcher.lower();
            } else if i == l_len {
                searcher.higher();
            } else {
                break;
            }
        }
        let mut file = BufReader::new(searcher.into_inner()?);
        let mut buf = Vec::new();
        let mut result = Vec::new();
        while file.read_until(b'\n', &mut buf)? > 0 {
            if buf.starts_with(&needle) {
                result.push(buf[needle.len()..buf.len()-1].to_owned());
            } else if buf.gt(&needle) {
                break;
            }
            buf.clear();
        }
        Ok(result)
    }
}
