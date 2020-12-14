use std::fs::File;
use std::io::Read;
use std::process;

pub fn get_term() -> Result<String, std::io::Error> {
    let mut terminal: String = String::new();
    let mut id = process::id();
    let mut path = format!("/proc/{}/status", id);
    let mut buf = String::new();

    for i in 0..3 {
        buf.clear();
        File::open(&path)?.read_to_string(&mut buf)?;
        for line in buf.lines() {
            if line.starts_with("PPid:") {
                id = line.split('\t').collect::<Vec<&str>>()[1].parse().unwrap();
                path = format!("/proc/{}/status", id);
            }

            if i == 2 && line.starts_with("Name:") {
                terminal = line.split('\t').collect::<Vec<&str>>()[1].to_string();
                break;
            }
        }
    }
    Ok(terminal)
}
