#![allow(dead_code)]
pub mod function_error {

    use anyhow::{Context, Result};
    use serde_json::{Value, from_str};
    use std::fs::read_to_string;

    pub fn parse_config(path: &str) -> Result<Value> {
        let content = read_to_string(path)
            .with_context(|| format!("cannot read config {}", path))?;
        let json =
            from_str(&content).with_context(|| format!("parse error"))?;
        Ok(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use function_error::*;

    #[test]
    fn it_works() {
        let config = parse_config("config.json");
        println!("Config: {:#?}", config);
    }
}
