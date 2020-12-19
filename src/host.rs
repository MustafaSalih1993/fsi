use std::{fs::File, io::Read};

pub fn get_host() -> Result<String, std::io::Error> {
    let mut buf = String::new();
    File::open("/sys/devices/virtual/dmi/id/product_name")?.read_to_string(&mut buf)?;
    Ok(buf.trim().to_string())
}
