use std::env;
use std::fs::File;
use std::io::Read;

pub fn get_wm() -> Result<(String, String), std::io::Error> {
    // if X is not running dont do anything
    if env::var("DISPLAY").is_err() {
        return Ok((String::from(""), String::from("")));
    };

    // response is what this function will return
    let response: (String, String);

    /* TODO: too much IFs and repeated code, i dont like it,
    i have to find another way.*/

    // check Desktop Environments:
    if env::var("XDG_SESSION_DESKTOP").is_ok() {
        let val = env::var("XDG_SESSION_DESKTOP").unwrap();
        response = (String::from("DE"), val);
        return Ok(response);
    };

    if env::var("XDG_CURRENT_DESKTOP").is_ok() {
        let val = env::var("XDG_CURRENT_DESKTOP").unwrap();
        response = (String::from("DE"), val);
        return Ok(response);
    };

    if env::var("DESKTOP_SESSION").is_ok() {
        let val = env::var("DESKTOP_SESSION").unwrap();
        response = (String::from("DE"), val);
        return Ok(response);
    };

    // if reached here then its not a desktop environment

    let path = format!("{}/.xinitrc", env::var("HOME").unwrap());
    let mut buf = String::new();
    {
        File::open(path)?.read_to_string(&mut buf)?;
        let lines = buf.lines().last().unwrap().trim().to_string();
        let last_line: Vec<&str> = lines.split(' ').collect();
        buf = last_line[last_line.len() - 1].to_string();
    }
    response = (String::from("WM"), buf);
    Ok(response)
}
