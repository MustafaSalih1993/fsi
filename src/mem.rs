use std::fs::File;
use std::io::Read;

// TODO: add/calculate used memory too

pub fn get_mem() -> Result<String, std::io::Error> {
    let mut mem = String::new();
    let mut total: i32 = 0;
    File::open("/proc/meminfo")?.read_to_string(&mut mem)?;

    for line in mem.lines() {
        if line.starts_with("MemTotal") {
            let tmp_line: String =
                line.trim().split_whitespace().collect::<Vec<&str>>()[1].to_string();
            total = match tmp_line.parse() {
                Ok(v) => v,
                Err(_) => total,
            };
            break;
        }
    }

    let mem = format!("{}MB", total / 1024);
    Ok(mem)
}
