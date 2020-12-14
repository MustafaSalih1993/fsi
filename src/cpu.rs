use std::fs::File;
use std::io::Read;

pub fn get_cpu() -> Result<String, std::io::Error> {
    let mut buf = String::new();
    File::open("/proc/cpuinfo")?.read_to_string(&mut buf)?;
    let mut res = String::new();
    for line in buf.lines() {
        if line.starts_with("model name") {
            let tmp_str: String = line.split(':').collect::<Vec<&str>>()[1].trim().to_string();
            res = tmp_str;
            break;
        }
    }
    Ok(res)
}
