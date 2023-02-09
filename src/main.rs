use std::process::exit;
use std::env;
use std::fs;
use libc;

static NUMBERS: &'static [i32] = &[0, 200, 508, 711, 996, 1394, 1952, 2733, 3827, 5347, 7142];
static BRIGHT_FILE: &'static str = "brightness";
static DEF_OPACITY: usize = 8;
static EXE_NAME: &str = "dwmblocks";

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
