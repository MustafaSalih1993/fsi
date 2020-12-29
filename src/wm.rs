use std::env;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};

pub fn get_wm() -> Result<(String, String), Error> {
    // if X is not running dont do anything
    if env::var("DISPLAY").is_err() {
        return Err(Error::from(ErrorKind::NotFound));
    };

    // check Desktop Environments:
    for env_var in &[
        "XDG_SESSION_DESKTOP",
        "XDG_CURRENT_DESKTOP",
        "DESKTOP_SESSION",
    ] {
        if let Some(de) = env::var(env_var).ok() {
            let response = (String::from("DE"), de);
            return Ok(response);
        }
    }

    // if reached here then its not a desktop environment

    let path = format!("{}/.xinitrc", env::var("HOME").unwrap());
    let mut buf = String::new();
    {
        File::open(path)?.read_to_string(&mut buf)?;
        let lines = buf.lines().last().unwrap().trim().to_string();
        let last_line: Vec<&str> = lines.split(' ').collect();
        buf = last_line[last_line.len() - 1].to_string();
    }
    let response = (String::from("Wm"), buf);
    Ok(response)
}
