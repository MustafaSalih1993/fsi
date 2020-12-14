use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn get_kernel() -> Result<String, std::io::Error> {
    let mut kernel = String::new();

    {
        let line = fetch_data("/proc/sys/kernel/ostype")?;
        kernel.push_str(&line);
    }

    {
        let line = fetch_data("/proc/sys/kernel/osrelease")?;
        kernel.push(' ');
        kernel.push_str(&line);
    }

    Ok(kernel)
}

fn fetch_data(path: &str) -> Result<String, std::io::Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    line = line.trim().to_string();
    if line.ends_with('\n') {
        line.remove(line.len() - 1);
    }
    Ok(line)
}
