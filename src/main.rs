use std::process;
use std::fs;
use std::env;
use std::io::Write;

#[derive(Default)]
struct Brightness {
    file: String,
    num: i32,
    index: usize,
}

const NLEVELS: usize = 11;
static mut LEVELS: [i32; NLEVELS] = [0; NLEVELS];

static BRIGHT_DIR: &str = "/sys/class/backlight/intel_backlight";
static EXE_NAME: &str = "dwmblocks";

fn between(a: i32, x: i32, b: i32) -> bool {
    x < b && a <= x
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

    NLEVELS - 1
}

fn create_levels(last: i32) {
    let first = last / 60;
    let n = NLEVELS - 2;
    let m: f64 = 1_f64 / (n-1) as f64;
    let quotient: f64 = f64::powf(last as f64 / first as f64, m);

    unsafe {
        LEVELS[0] = 0;
        LEVELS[1] = 1;
        LEVELS[2] = first;
        for i in 3..NLEVELS-1 {
            LEVELS[i] = (LEVELS[i - 1] as f64 * quotient) as i32;
        }
        LEVELS[NLEVELS - 1] = last;
    }
}

fn get_bright(bright: &mut Brightness) {
    let current = match fs::read_to_string(&bright.file) {
        Ok(data) => { data.trim().parse().unwrap() },
        Err(e) => { 
            eprintln!("Error reading brightness: {e}"); 
            process::exit(1) 
        },
    };
    bright.num = current;
    bright.index = find_index(bright.num);
}

fn save_new(new_bright: &Brightness) {
    let mut file = match fs::File::create(&new_bright.file) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating file: {}", e);
            return;
        }
    };

    unsafe {
        if writeln!(&mut file, "{}", LEVELS.get(new_bright.index).unwrap_or(&0)).is_err() {
            // eprintln!("Error writing to file: {}", e);
        }
    }
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

    let mut max_bright: Brightness = Brightness::default();
    let mut old_bright: Brightness = Brightness::default();
    let mut new_bright: Brightness = Brightness::default();

    max_bright.file = format!("{}/max_brightness", BRIGHT_DIR);
    old_bright.file = format!("{}/brightness", BRIGHT_DIR);
    new_bright.file = format!("{}/brightness", BRIGHT_DIR);

    get_bright(&mut max_bright);
    create_levels(max_bright.num);
    get_bright(&mut old_bright);

    if argv.len() <= 1 {
        println!("🔆 {}", old_bright.index);
        process::exit(1);
    }

    unsafe {
        new_bright.index = match argv[1].as_str() {
            "-" => if old_bright.index >= 1 { old_bright.index - 1 } else { old_bright.index },
            "+" => if old_bright.index < LEVELS.len() - 1 { old_bright.index + 1 } else { old_bright.index },
            "=" => LEVELS.len() - 1,
            _ => { 
                usage!(); 
                process::exit(1);
            }
        };
    }

    save_new(&new_bright);
    unsafe {
        fs::write(&new_bright.file, LEVELS[new_bright.index].to_string()).expect("Unable to write file");
        println!("🔆 {}", new_bright.index);
    }

    let bright = env::var("BRIGHT").unwrap();
    let bright: i32 = bright.parse().unwrap();
    
    let dirs = match fs::read_dir("/proc/") {
        Ok(dirs) => dirs,
        Err(error) => {
            println!("Error: {}", error); 
            process::exit(1);
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
}

fn check_pid(path: &str) -> i32 {
    let statfile = format!("{}/{}", path, "stat");
    let data = match fs::read_to_string(&statfile) {
        Ok(data) => data,
        Err(_) => return 0,
    };
    let data: Vec<&str> = data.split('(').collect();
    if data.len() < 2 {
        return 0;
    }
    let data = data[1];
    let data: Vec<&str> = data.split(')').collect(); 
    if data.is_empty() {
        return 0;
    }
    let data = data[0];
    if data == EXE_NAME {
        let pid: Vec<&str> = path.split('/').collect();
        if pid.len() >= 3 {
            let pid = pid[2];
            return pid.parse().unwrap();
        }
    }
    0
}
