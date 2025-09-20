#![allow(dead_code)]
pub mod read_file_error {

    use anyhow::{Context, Result};
    use std::fs::read_to_string;

    pub fn read_file(path: &str) -> Result<String> {
        read_to_string(path)
            // adding context with anyhow
            .with_context(|| format!("failed to read file at path: {}", path))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use read_file_error::*;

    #[test]
    fn it_works() {
        match read_file("fake_file.txt") {
            Ok(content) => println!("Content: {}", content),
            Err(e) => println!("Error: {e}"),
        }
    }
}
