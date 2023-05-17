use std::fs;
use std::process;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn send_signal(signal_number: i32, program: &str) {
    let dirs = match fs::read_dir("/proc/") {
        Ok(dirs) => dirs,
        Err(error) => {
            println!("Error opening /proc/: {}", error); 
            process::exit(1);
        },
    };

    for dir in dirs {
        let d = dir.unwrap();
        let name = &d.file_name().to_string_lossy().to_string();
        if let Some(pid) = check_pid(program, name) {
            unsafe {
                libc::kill(pid, libc::SIGRTMIN() + signal_number);
            }
        }
    }
}

fn check_pid(executable: &str, number: &str) -> Option<i32> {
    let pid = match number.parse::<i32>() {
        Ok(pid) if pid > 0 => pid,
        _ => return None,
    };

    let cmdline_path = format!("/proc/{}/cmdline", number);
    if let Ok(cmdline_file) = File::open(cmdline_path) {
        let mut command = String::new();
        if let Ok(_) = BufReader::new(cmdline_file).read_line(&mut command) {
            command.pop();
            if command == executable {
                return Some(pid);
            }
        }
    }
    None
}
