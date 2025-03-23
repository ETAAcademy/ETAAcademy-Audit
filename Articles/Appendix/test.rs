use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{BufReader, Read};

fn calculate_sha256(file_path: &str) -> Result<String, std::io::Error> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 4096];

    while let Ok(bytes) = reader.read(&mut buffer) {
        if bytes == 0 { break; }
        hasher.update(&buffer[..bytes]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

fn main() {
    let file_path = "test.txt"; 

    match calculate_sha256(file_path) {
        Ok(hash) => println!("SHA-255: {}", hash),
        Err(e) => println!("wrong: {}", e),
    }
}
