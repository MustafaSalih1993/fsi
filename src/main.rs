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

fn main() -> Result<(), std::io::Error> {
    /*                                            Return values: */

    let distro = distro::get_name()?; /*     struct of  String "basic" and "pretty" */
    let shell = shell::get_shell()?; /*      String                                 */
    let kernel = kernel::get_kernel()?; /*   String                                 */
    let uptime = uptime::get_uptime()?; /*   String                                 */
    let gpus = gpu::get_gpus()?; /*          Hashmap contains the gpus              */
    let mem = mem::get_mem()?; /*            String                                 */
    let cpu = cpu::get_cpu()?; /*            String                                 */
    let wm = wm::get_wm()?; /*               Tuple (WM/DE, NAME)                    */
    let host = host::get_host()?; /*         String                                 */
    let term = term::get_term()?; /*         String                                 */

    // args handling:

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let mut pretty_name: bool = false; // signal to print distro pretty_name or not

    if !args.is_empty() {
        for arg in args.iter() {
            if arg == "--pretty" || arg == "-p" {
                pretty_name = true;
            }
        }
    }

    /*                 OUTPUT                 */

    println!("─────────────────────────────────────\n");

    // handling the -p || --pretty arg to print pretty name
    if pretty_name {
        println!(" Distro:  {}", distro.pretty);
    } else {
        println!(" Distro:  {}", distro.basic); //default
    }

    println!(" Kernel:  {}", kernel);
    println!(" Uptime:  {}", uptime);
    println!(" Host:    {}", host);
    println!(" Shell:   {}", shell);
    println!(" Cpu:     {}", cpu);

    //iterate in the hashmap of GPUs
    for (_, v) in gpus {
        println!(" Gpu:     {}", v);
    }

    println!(" Mem:     {}", mem);

    // check if $DISPLAY variable not set, will not show WM/DE.
    if !wm.0.is_empty() {
        println!(" {}:      {}", wm.0, wm.1);
    };
    println!(" Term:    {}", term);

    println!("\n─────────────────────────────────────");

    Ok(())
}
