use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind};
use std::process::Command;
use std::{env, env::VarError};
use std::{fs::File, io::Read, process};

/* Handling arguments and returning configuration struct  */

pub struct Config {
    pub program_name: String,
    pub pretty_name: bool,
}
impl Config {
    pub fn new(program_name: String, pretty_name: bool) -> Config {
        Config {
            program_name,
            pretty_name,
        }
    }
}

pub fn configuration() -> Config {
    let mut args: Vec<String> = env::args().collect();
    let program_name = args.remove(0);

    let mut config = Config::new(program_name, false);

    if !args.is_empty() {
        for arg in args.iter() {
            if arg == "--help" || arg == "-h" {
                print_usage(config.program_name);
                process::exit(0);
            } else if arg == "--pretty" || arg == "-p" {
                config.pretty_name = true;
                continue;
            }
            println!(
                "Unknown option: {}, check all options with -h, --help options.",
                arg
            );
            print_usage(config.program_name);
            process::exit(1);
        }
    }
    config
}
fn print_usage(program_name: String) {
    println!("{0}: USAGE: {0} <Args>\n", program_name);
    println!("Args:");
    println!("-p, --pretty:");
    println!("\tPrints the pretty name of the Distro instead of the default name (basic).\n");
    println!("-h, --help:");
    println!("\tPrints this help message.\n");
}

/* ###################Running the application ########################  */

pub fn run(config: Config) {
    // a vector of tuples containing (key,value) to final data to Display
    let mut data_holder: Vec<(String, String)> = Vec::new();
    {
        // insert distro name
        if config.pretty_name {
            if let Ok(name) = get_distro() {
                data_holder.push((String::from("Distro:\t\t"), name.pretty));
            }
        } else if let Ok(name) = get_distro() {
            data_holder.push((String::from("Distro:\t\t"), name.basic));
        }

        // insert shell (String)
        if let Ok(shell) = get_shell() {
            data_holder.push((String::from("Shell:\t\t"), shell));
        }

        // insert kernel (String)
        if let Ok(kernel) = get_kernel() {
            data_holder.push((String::from("Kernel:\t\t"), kernel));
        }

        // insert uptime  (String)
        if let Ok(uptime) = get_uptime() {
            data_holder.push((String::from("Uptime:\t\t"), uptime));
        }

        // insert host name (String)
        if let Ok(host) = get_host() {
            data_holder.push((String::from("Host:\t\t"), host));
        }

        // insert Terminal (String)
        if let Ok(term) = get_term() {
            data_holder.push((String::from("Terminal:\t"), term));
        }

        // insert memory (String)
        if let Ok(mem) = get_mem() {
            data_holder.push((String::from("Memory:\t\t"), mem));
        }

        // insert cpu (String)
        if let Ok(cpu) = get_cpu() {
            data_holder.push((String::from("Cpu:\t\t"), cpu));
        }

        // insert Gpu(s) (Hashmap)
        if let Ok(gpus) = get_gpus() {
            let mut i = 0;
            for (_, gpu) in gpus {
                let name = format!("Gpu{}:\t\t", i);
                data_holder.push((name, gpu));
                i += 1;
            }
        }

        // insert window manager / desktop environment tuple=(str,String)
        if let Ok((key, val)) = get_wm() {
            let key = format!("{}:\t\t", key);
            data_holder.push((key, val));
        }
    }

    /* FINAL OUTPUT  */
    println!("─────────────────────────────────────\n");

    // iterating thro the vector that holds the collected data.
    for (k, v) in data_holder {
        println!("{}{}", k, v);
    }
    println!("\n─────────────────────────────────────");
}

/* ######################### Distro Name Returns Result<Distro> ############# */
pub struct Distro {
    pub basic: String,
    pub pretty: String,
}

pub fn get_distro() -> Result<Distro, std::io::Error> {
    let path = "/etc/os-release";
    let mut names = Distro {
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

/*######################### Shell Name Returns Result<String> ################ */

pub fn get_shell() -> Result<String, VarError> {
    match env::var("SHELL") {
        Ok(s) => {
            let s = s.split('/').last().unwrap().to_string();
            Ok(s)
        }
        Err(e) => Err(e),
    }
}

/* ####################### Kernel name/version Returns Result<String> ####### */

pub fn get_kernel() -> Result<String, std::io::Error> {
    let mut kernel = String::new();

    {
        let line = fetch_kernel_line("/proc/sys/kernel/ostype")?;
        kernel.push_str(&line);
    }

    {
        let line = fetch_kernel_line("/proc/sys/kernel/osrelease")?;
        kernel.push(' ');
        kernel.push_str(&line);
    }

    Ok(kernel)
}

fn fetch_kernel_line(path: &str) -> Result<String, std::io::Error> {
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

/* ####################### Uptime Returns Result<String> ####### */

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

/* ####################### Host Name Returns Result<String> ####### */

pub fn get_host() -> Result<String, std::io::Error> {
    let mut buf = String::new();
    File::open("/sys/devices/virtual/dmi/id/product_name")?.read_to_string(&mut buf)?;
    Ok(buf.trim().to_string())
}

/* ####################### Terminal Name Returns Result<String> ####### */

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

/* ####################### Memory Usage/Total Returns Result<String> ####### */

pub fn get_mem() -> Result<String, std::io::Error> {
    let mut buf = String::new();

    File::open("/proc/meminfo")?.read_to_string(&mut buf)?;

    // values below to calculate the usage
    let mut mem_total: u32 = 0;
    let mut shmem: u32 = 0;
    let mut mem_free: u32 = 0;
    let mut mem_buffers: u32 = 0;
    let mut mem_cached: u32 = 0;
    let mut mem_srecl: u32 = 0;

    for line in buf.lines() {
        if mem_total > 0
            && shmem > 0
            && mem_free > 0
            && mem_buffers > 0
            && mem_cached > 0
            && mem_srecl > 0
        {
            break;
        }
        if line.starts_with("MemTotal") {
            assign_val(line, &mut mem_total);
        }
        if line.starts_with("SReclaimable") {
            assign_val(line, &mut mem_srecl)
        }
        if line.starts_with("Cached") {
            assign_val(line, &mut mem_cached)
        }

        if line.starts_with("Shmem") {
            assign_val(line, &mut shmem);
        }

        if line.starts_with("MemFree") {
            assign_val(line, &mut mem_free);
        }
        if line.starts_with("Buffers") {
            assign_val(line, &mut mem_buffers);
        }
    }

    let mem_used = (mem_total + shmem - mem_free - mem_buffers - mem_cached - mem_srecl) / 1024; //calculating and converting to Megabyte
    let mem_total = (mem_total as f32 / 1024 as f32) / 1000.0; //converting to GB

    let result: String;
    if mem_used > 1000 {
        result = format!("{:.1}G / {:.1}G", mem_used as f32 / 1000.0, mem_total);
    } else {
        result = format!("{}M / {:.1}G", mem_used, mem_total);
    }
    Ok(result)
}

fn assign_val(line: &str, assignable: &mut u32) {
    let parsed: u32 = line.split(':').collect::<Vec<&str>>()[1]
        .trim()
        .split(' ')
        .collect::<Vec<&str>>()[0]
        .parse()
        .unwrap();
    *assignable = parsed;
}

/* ####################### Cpu Model Returns Result<String> ####### */

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

/* #################### Gpu Names Returns Result<Hashmap<String,String>> ####### */

/* TODO: im not happy with this way (calling sh), so this module depends
   on `lspci` command, i have to find another way to get GPUs.

   TODO: the output looks ugly, fix it you asshole.

*/
pub fn get_gpus() -> Result<HashMap<String, String>, std::io::Error> {
    let mut map = HashMap::new();

    let res = Command::new("sh")
        .args(&["-c", "lspci | grep -I 'VGA\\|Display\\|3D'"])
        .output()?;
    let lines_of_gpus = String::from_utf8_lossy(&res.stdout);

    // this way we handle multiple gpus

    for line in lines_of_gpus.lines() {
        let k = line.split(':').collect::<Vec<&str>>()[1].trim().to_string();
        let v = line.split(':').collect::<Vec<&str>>()[2].trim().to_string();
        map.insert(k, v);
    }

    Ok(map)
}

/* ####### DesktopEnvironment/WindowManager Name Returns Result<Tuple> ####### */

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
