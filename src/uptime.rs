use std::fs::File;
use std::io::Read;

pub fn get_uptime() -> Result<String, std::io::Error> {
    let mut buf = String::new();
    File::open("/proc/uptime")?.read_to_string(&mut buf)?;

    let buf: f32 = buf.split(' ').collect::<Vec<&str>>()[0].parse().unwrap();

    let hour = buf.round() as u32 / 3600;
    let rem = buf as u32 - hour * 3600;
    let minutes = rem / 60;

    let result = if hour > 0 {
        format!("{} hours, {} minutes", hour, minutes)
    } else {
        format!("{} minutes", minutes)
    };
    Ok(result)
}
