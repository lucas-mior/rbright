use std::process::exit;
use std::env;
use std::fs;
use libc;

struct Brightness {
    file: String,
    string: String,
    num: i32,
    index: usize,
}

const NLEVELS: usize = 11;
static mut LEVELS: [i32; NLEVELS] = [0; NLEVELS];

static BRIGHT_DIR: &'static str = "/sys/class/backlight/intel_backlight";
static EXE_NAME: &str = "dwmblocks";

fn between(a: i32, x: i32, b: i32) -> bool {
    return x < b && a <= x;
}

fn find_index(value: i32) -> usize {
    let mut i: usize = 0;

    unsafe {
        while i <= NLEVELS - 2 {
            if between(LEVELS[i], value, LEVELS[i+1]) {
                return i;
            } else {
                i += 1;
            }
        }
    }

    return NLEVELS - 1;
}

fn create_levels(last: i32) {
    let first = last / 60;
    let n = NLEVELS - 2;
    let m: f64 = 1 as f64 / (n-1) as f64;
    let quotient: f64 = f64::powf(last as f64 / first as f64, m);

    unsafe {
        let mut i: usize;
        LEVELS[0] = 0;
        LEVELS[1] = 1;
        LEVELS[2] = first;
        for i in 3..NLEVELS-1 {
            LEVELS[i] = (LEVELS[i - 1] as f64 * quotient) as i32;
        }
        LEVELS[NLEVELS - 1] = last;
    }
    return;
}

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

    let bright = env::var("BRIGHT").unwrap();
    let bright: i32 = bright.parse().unwrap();
    
    let dirs = match fs::read_dir("/proc/") {
        Ok(dirs) => dirs,
        Err(error) => {
            println!("Error: {}", error); 
            exit(1);
        },
    };

    for dir in dirs {
        let d = dir.unwrap();
        let pid = check_pid(&d.path().to_string_lossy().to_string());
        if pid != 0 {
            unsafe {
                libc::kill(pid, libc::SIGRTMIN() + bright);
            }
        }
    }

    return;
}

fn check_pid(path: &String) -> i32 {
    let statfile = format!("{}/{}", path, "stat");
    let data = match fs::read_to_string(&statfile) {
        Ok(data) => data,
        Err(_) => return 0,
    };
    let data: Vec<&str> = data.split("(").collect();
    if data.len() < 2 {
        return 0;
    }
    let data = data[1];
    let data: Vec<&str> = data.split(")").collect(); 
    if data.len() < 1 {
        return 0;
    }
    let data = data[0];
    if data == EXE_NAME {
        let pid: Vec<&str> = path.split("/").collect();
        if pid.len() >= 3 {
            let pid = pid[2];
            return pid.parse().unwrap();
        }
    }
    return 0;
}
