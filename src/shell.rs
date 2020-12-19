use std::{env, env::VarError};

pub fn get_shell() -> Result<String, VarError> {
    match env::var("SHELL") {
        Ok(s) => Ok(s),
        Err(e) => Err(e),
    }
}
