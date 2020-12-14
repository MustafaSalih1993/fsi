use std::env;

pub fn get_shell() -> Result<String, std::io::Error> {
    let sh = match env::var("SHELL") {
        Ok(s) => s,
        Err(_) => String::from("Not found"),
    };

    Ok(sh)
}
