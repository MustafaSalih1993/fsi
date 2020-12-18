use std::collections::HashMap;
use std::env;
mod cpu;
mod distro;
mod gpu;
mod host;
mod kernel;
mod mem;
mod shell;
mod term;
mod uptime;
mod wm;

// TODO: too much 'if let' i dont like it, fix that you fuckin asshole!

fn main() -> Result<(), std::io::Error> {
    // args handling:
    let mut args: Vec<String> = env::args().collect();
    let program_name = args.remove(0);

    let mut pretty_name: bool = false; // signal to print distro pretty_name or not

    if !args.is_empty() {
        for arg in args.iter() {
            if arg == "--help" || arg == "-h" {
                print_usage(program_name);
                return Ok(());
            }
            if arg == "--pretty" || arg == "-p" {
                pretty_name = true;
                continue;
            }
            println!(
                "Unknown option: {}, check all options with -h, --help options.",
                arg
            );
        }
    }

    let mut map = HashMap::new(); // this hashmap will hold all the information to output
    {
        // insert distro name from the returning struct = {pretty(String),basic(String)}
        // checking if -p, --pretty options are included
        if pretty_name {
            if let Ok(name) = distro::get_name() {
                map.insert(String::from("Distro:\t\t"), name.pretty);
            }
        } else {
            if let Ok(name) = distro::get_name() {
                map.insert(String::from("Distro:\t\t"), name.basic);
            }
        }

        // insert shell (String)
        if let Ok(shell) = shell::get_shell() {
            map.insert(String::from("Shell:\t\t"), shell);
        }

        // insert kernel (String)
        if let Ok(kernel) = kernel::get_kernel() {
            map.insert(String::from("Kernel:\t\t"), kernel);
        }

        // insert uptime  (String)
        if let Ok(uptime) = uptime::get_uptime() {
            map.insert(String::from("Uptime:\t\t"), uptime);
        }

        // insert host name (String)
        if let Ok(host) = host::get_host() {
            map.insert(String::from("Host:\t\t"), host);
        }

        // insert Terminal (String)
        if let Ok(term) = term::get_term() {
            map.insert(String::from("Terminal:\t"), term);
        }

        // insert memory (String)
        if let Ok(mem) = mem::get_mem() {
            map.insert(String::from("Memory:\t\t"), mem);
        }

        // insert cpu (String)
        if let Ok(cpu) = cpu::get_cpu() {
            map.insert(String::from("Cpu:\t\t"), cpu);
        }

        // insert Gpu(s) (Hashmap)
        if let Ok(gpus) = gpu::get_gpus() {
            let mut i = 1;
            for (_, gpu) in gpus {
                let name = format!("Gpu{}:\t\t", i);
                map.insert(name, gpu);
                i += 1;
            }
        }

        // insert window manager / desktop environment tuple=(str,String)
        if let Ok((key, val)) = wm::get_wm() {
            let key = format!("{}:\t\t", key);
            map.insert(key, val);
        }
    }

    /* FINAL OUTPUT  */
    println!("─────────────────────────────────────\n");

    // iterating thro the hashmap that holds the collected data.
    for (k, v) in map {
        println!("{}{}", k, v);
    }

    println!("\n─────────────────────────────────────");

    Ok(())
}

fn print_usage(s: String) {
    println!("{0}: USAGE: {0} <Args>\n", s);
    println!("Args:");
    println!("-p, --pretty:");
    println!("\tPrints the pretty name of the Distro instead of the default name (basic).\n");
    println!("-h, --help:");
    println!("\tPrints this help message.\n");
}
