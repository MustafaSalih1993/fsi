use std::{env, env::VarError};

pub fn get_shell() -> Result<String, VarError> {
    match env::var("SHELL") {
        Ok(s) => {
            let s = s.split('/').last().unwrap().to_string();
            Ok(s)
        }
        Err(e) => Err(e),
    }
}
