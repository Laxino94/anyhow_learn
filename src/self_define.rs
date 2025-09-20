#![allow(dead_code)]
pub mod self_define {

    use anyhow::{Context, Error, Result};
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    #[error("Invalid input: {msg}")]
    pub struct InvalidInput {
        msg: String,
    }
    impl InvalidInput {
        pub fn new(msg: &str) -> Self {
            Self {
                msg: msg.to_string(),
            }
        }
    }

    pub fn validate_age(age: u8) -> Result<u8> {
        if age < 18 {
            return Err(Error::new(InvalidInput::new(
                "Age must be greater than 18!",
            )));
        }
        Ok(age)
    }

    pub fn process_user(age: u8) -> Result<()> {
        let validated = validate_age(age).context("failed to validate age")?;
        println!("passed validate with age: {}", validated);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use self_define::*;

    #[test]
    fn it_works() {
        process_user(19).expect("User should pass validation!")
    }
}
