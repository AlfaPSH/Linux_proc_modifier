use std::process::Command;
use std::error::Error;

pub fn get_pid_by_name(process_name: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    let output = Command::new("pidof")
        .arg(process_name)
        .output()?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let pid_str = String::from_utf8(output.stdout)?;
    let pids: Result<Vec<u32>, _> = pid_str
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u32>())
        .collect();

    Ok(pids?)
}

pub fn check_root() -> bool {
    unsafe { libc::geteuid() == 0 }
}