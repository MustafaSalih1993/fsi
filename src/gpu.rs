use std::collections::HashMap;
use std::process::Command;

/* TODO: im not happy with this way (calling sh), i have to find another way to get GPUs.

   TODO: GPUs output looks ugly, fix it you asshole.

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
