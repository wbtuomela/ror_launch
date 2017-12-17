use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use md5::{Digest, Md5};

const BUFFER_SIZE: usize = 1024;
const EXPECTED_HASH: &str = "8fc62753982d50cf6a6b73025adf98fb";

fn md5_hash<R: Read>(reader: &mut R) -> String {
    let mut sh = Md5::default();
    let mut buffer = [0u8; BUFFER_SIZE];
    loop {
        let n = match reader.read(&mut buffer) {
            Ok(n) => n,
            Err(_) => return String::default(),
        };
        sh.input(&buffer[..n]);
        if n == 0 || n < BUFFER_SIZE {
            break;
        }
    }

    let hashed = sh.result();
    let mut result = String::new();
    for byte in hashed.iter() {
        result.push_str(&format!("{:02x}", byte));
    }

    result
}

pub struct Patcher {
    file: File,
}

impl Patcher {
    pub fn new() -> Self {
        let mut full_path = PathBuf::new();
        full_path.push(env::current_dir().unwrap());
        full_path.push("WAR.exe");

        assert!(full_path.exists());
        Self {
            file: File::open(full_path).unwrap(),
        }
    }

    pub fn needs_patch(&mut self) -> bool {
        let hash = md5_hash(&mut self.file);
        hash == EXPECTED_HASH
    }

    pub fn patch(&self) {
        unimplemented!();
    }
}
