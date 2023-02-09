// use nix::unistd::getppid;
use std::process::exit;
use std::env;
use std::fs;

static NUMBERS: &'static [i32] = &[0, 200, 508, 711, 996, 1394, 1952, 2733, 3827, 5347, 7142];
static BRIGHT_FILE: &'static str = "brightness";
static DEF_OPACITY: usize = 8;

static USAGE: &str = 
"bright [-+=h]
- -- decrease
+ -- increase
= -- set 100%
h -- print this help message";
macro_rules! usage {
    () => { 
        println!("{}", USAGE);
    };
}

fn main() {
    let argv: Vec<String> = env::args().collect();

    let bright = match fs::read_to_string(&BRIGHT_FILE) {
        Ok(data) => data.parse().unwrap(),
        Err(_) => NUMBERS[DEF_OPACITY],
    };
    let mut index: usize = NUMBERS.iter().position(|&x| x == bright).unwrap();

    if argv.len() <= 1 {
        println!("🔆 {}", index);
        exit(1);
    }

    match argv[1].as_str() {
        "-" => index = if index >= 1 { index - 1 } else { index },
        "+" => index = if index < NUMBERS.len() - 1 { index + 1 } else { index },
        "=" => index = NUMBERS.len() - 1,
        _ => { 
            usage!(); 
            exit(1);
        }
    };

    fs::write(BRIGHT_FILE, NUMBERS[index].to_string()).expect("Unable to write file");
    println!("🔆 {}", index);

    let bright_env = env::var("BRIGHT").unwrap();
    println!("BRIGHT: {}", bright_env);
    return;
}
