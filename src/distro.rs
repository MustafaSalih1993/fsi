use std::fs::File;
use std::io::Read;

pub struct Name {
    pub basic: String,
    pub pretty: String,
}
pub fn get_name() -> Result<Name, std::io::Error> {
    let path = "/etc/os-release";
    let mut names = Name {
        basic: String::new(),
        pretty: String::new(),
    };

    let mut buf = String::new();
    File::open(path)?.read_to_string(&mut buf)?;
    for line in buf.lines() {
        if line.starts_with("NAME") {
            let splited: Vec<&str> = line.split('=').collect();
            if let Some(name) = splited.get(1) {
                let mut tmp_name = name.to_string().trim().to_string();
                tmp_name.remove(0);
                tmp_name.remove(tmp_name.len() - 1);
                names.basic = tmp_name
            }
        } else if line.starts_with("PRETTY_NAME") {
            let splited: Vec<&str> = line.split('=').collect();
            if let Some(name) = splited.get(1) {
                let mut tmp_name = name.to_string().trim().to_string();
                tmp_name.remove(0);
                tmp_name.remove(tmp_name.len() - 1);
                names.pretty = tmp_name
            }
        }
    }
    Ok(names)
}
