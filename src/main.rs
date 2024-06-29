use std::{env, fs};

const CONF_PATH: &str = "/sys/class/backlight/amdgpu_bl1/brightness";
const MAX_PATH: &str = "/sys/class/backlight/amdgpu_bl1/max_brightness";

fn main() {
    let curr = fs::read_to_string(CONF_PATH).unwrap();
    let curr = curr.trim().parse::<i64>().unwrap();
    let max = fs::read_to_string(MAX_PATH).unwrap();
    let max = max.trim().parse::<i64>().unwrap();
    let mut args = env::args();
    if let Some(diff) = args.nth(1) {
        let diff = diff.parse::<i64>().unwrap();
        let new = curr + diff;
        if 0 <= new && new <= max {
            fs::write(CONF_PATH, new.to_string()).unwrap();
        }
    } else {
        println!("{curr}");
    }
}
