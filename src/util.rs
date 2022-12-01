#[cfg(debug_assertions)]
use std::{fs::File, io::Read};

#[cfg(debug_assertions)]
use anyhow::{anyhow, Result};

#[cfg(debug_assertions)]
pub fn read_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut input = String::new();

    match file.read_to_string(&mut input) {
        Ok(_) => Ok(input),
        Err(e) => Err(anyhow!(e)),
    }
}
