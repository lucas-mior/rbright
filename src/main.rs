use nix::unistd::getppid;
use std::process::exit;
use std::env;
use std::fs;
use std::cmp;

static NUMBERS: &'static [i32] = &[0, 10, 20, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 93, 96, 100];
static DEF_OPACITY: usize = 13;

static USAGE: &str = 
"rurxvt_opacity [-+=h]
- -- decrease
+ -- increase
= -- set 100% opaque
h -- print this help message";
macro_rules! usage {
    () => { 
        println!("{}", USAGE);
    };
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    const NAME: &str = "opacity";
    const CACHE: &str = "/tmp";
    let window_id: i32;

    if argv.len() <= 1 {
        usage!();
        exit(1);
    } else {
        window_id = getppid().into();
    }

    let file = format!("{}/{}_{}", CACHE, NAME, window_id);

    let mut current: usize;
    current = match fs::read_to_string(&file) {
        Ok(data) => data.parse().unwrap(),
        Err(_) => DEF_OPACITY,
    };

    match argv[1].as_str() {
        "-" => current = cmp::max(current - 1, 0),
        "+" => current = cmp::min(current + 1, NUMBERS.len() - 1),
        "=" => current = NUMBERS.len() - 1,
        _ => { 
            usage!(); 
            exit(1);
        }
    };

    fs::write(file, current.to_string()).expect("Unable to write file");
    print!("\x1b]011;[{}]#000000\x07", NUMBERS[current]); //background
    print!("\x1b]708;[{}]#000000\x07", NUMBERS[current]); //border

    return;
}
