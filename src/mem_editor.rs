/// Edits an environment variable for a process
pub fn automated_edit(pid: i32, var: &str, new_value: &str) {
    if let Ok(proc) = Process::new(pid) {
        if let Ok(mut env) = proc.environ() {
            let key = OsString::from(var);
            if env.contains_key(&key) {
                let old_value = env.get(&key).unwrap().clone();
                env.insert(key.clone(), OsString::from(new_value));
                let environ_path = format!("/proc/{}/environ", pid);
                backup_file(&environ_path);
                let new_env: Vec<u8> = env.iter().map(|(k, v)| format!("{}={}", k.to_string_lossy(), v.to_string_lossy())).collect::<Vec<_>>().join("\0").into_bytes();
                if let Ok(mut file) = OpenOptions::new().write(true).open(&environ_path) {
                    if let Err(e) = file.write_all(&new_env) {
                        ui::error(&format!("Failed to write environ: {}", e));
                        return;
                    }
                    ui::info(&format!("Changed {}: {} -> {}", var, old_value.to_string_lossy(), new_value));
                    log_change(&format!("PID {}: {} changed from {} to {}", pid, var, old_value.to_string_lossy(), new_value));
                } else {
                    ui::error("Could not open environ for writing (need root)");
                }
            } else {
                ui::error(&format!("Variable '{}' not found in environment", var));
            }
        } else {
            ui::error("Could not read environment");
        }
    } else {
        ui::error("Could not open process");
    }
}

/// Interactive environment variable editor
pub fn edit_env_vars(pid: i32) {
    if let Ok(proc) = Process::new(pid) {
        if let Ok(mut env) = proc.environ() {
            let vars: Vec<(OsString, OsString)> = env.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            let display_vars: Vec<String> = vars.iter().map(|(k, v)| format!("{}={}", k.to_string_lossy(), v.to_string_lossy())).collect();
            let selection = dialoguer::Select::new()
                .items(&display_vars)
                .with_prompt("Select variable to edit")
                .default(0)
                .interact();
            if let Ok(idx) = selection {
                let (var, val) = &vars[idx];
                ui::info(&format!("Current value of {}: {}", var.to_string_lossy(), val.to_string_lossy()));
                let new_value: String = Input::new()
                    .with_prompt(&format!("Enter new value for {}", var.to_string_lossy()))
                    .interact_text()
                    .unwrap();
                if Confirm::new().with_prompt(&format!("Confirm change of {} → {}?", var.to_string_lossy(), new_value)).interact().unwrap() {
                    let environ_path = format!("/proc/{}/environ", pid);
                    backup_file(&environ_path);
                    env.insert(var.clone(), OsString::from(new_value.clone()));
                    let new_env: Vec<u8> = env.iter().map(|(k, v)| format!("{}={}", k.to_string_lossy(), v.to_string_lossy())).collect::<Vec<_>>().join("\0").into_bytes();
                    if let Ok(mut file) = OpenOptions::new().write(true).open(&environ_path) {
                        if let Err(e) = file.write_all(&new_env) {
                            ui::error(&format!("Failed to write environ: {}", e));
                            return;
                        }
                        ui::info(&format!("Changed {}: {} -> {}", var.to_string_lossy(), val.to_string_lossy(), new_value));
                        log_change(&format!("PID {}: {} changed from {} to {}", pid, var.to_string_lossy(), val.to_string_lossy(), new_value));
                    } else {
                        ui::error("Could not open environ for writing (need root)");
                    }
                }
            }
        } else {
            ui::error("Could not read environment");
        }
    } else {
        ui::error("Could not open process");
    }
}
use crate::ui;
use procfs::process::Process;
use colored::*;
use std::fs::{OpenOptions, File};
use std::io::{Read, Write, Seek, SeekFrom};
// use std::path::Path;
use chrono::Local;
use libc;
use std::os::unix::fs::OpenOptionsExt;
use std::ffi::OsString;
use dialoguer::{Input, Confirm};


fn log_change(msg: &str) {
    let log_path = "/data/local/tmp/proc_editor_rs.log";
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(log_path) {
        let now = Local::now();
        let _ = writeln!(file, "[{}] {}", now.format("%Y-%m-%d %H:%M:%S"), msg);
    }
}

fn backup_file(path: &str) {
    if let Ok(mut src) = File::open(path) {
        let backup_path = format!("{}.bak", path);
        if let Ok(mut dst) = File::create(&backup_path) {
            let mut buf = Vec::new();
            let _ = src.read_to_end(&mut buf);
            let _ = dst.write_all(&buf);
        }
    }
}

pub fn search_and_edit_value(pid: i32, search_value: &str, new_value: &str) {
    if let Ok(proc) = Process::new(pid) {
        // Search in environment variables
        if let Ok(mut env) = proc.environ() {
            let mut found: Option<(OsString, OsString)> = None;
            for (k, v) in env.iter() {
                if v.to_string_lossy() == search_value {
                    found = Some((k.clone(), v.clone()));
                    break;
                }
            }
            if let Some((k, v)) = found {
                ui::info(&format!("Found env var {} with value {}", k.to_string_lossy(), v.to_string_lossy()));
                if Confirm::new().with_prompt(&format!("Change {} from {} to {}?", k.to_string_lossy(), v.to_string_lossy(), new_value)).interact().unwrap() {
                    env.insert(k.clone(), OsString::from(new_value));
                    let environ_path = format!("/proc/{}/environ", pid);
                    backup_file(&environ_path);
                    let new_env: Vec<u8> = env.iter().map(|(k, v)| format!("{}={}", k.to_string_lossy(), v.to_string_lossy())).collect::<Vec<_>>().join("\0").into_bytes();
                    if let Ok(mut file) = OpenOptions::new().write(true).open(&environ_path) {
                        if let Err(e) = file.write_all(&new_env) {
                            ui::error(&format!("Failed to write environ: {}", e));
                            return;
                        }
                        ui::info(&format!("Changed {}: {} -> {}", k.to_string_lossy(), v.to_string_lossy(), new_value));
                        log_change(&format!("PID {}: {} changed from {} to {}", pid, k.to_string_lossy(), v.to_string_lossy(), new_value));
                    } else {
                        ui::error("Could not open environ for writing (need root)");
                    }
                }
                return;
            }
        }
        // TODO: Search in memory maps and /proc/<pid>/mem
        // TODO: Search in memory maps and /proc/<pid>/mem
        // ...existing code...
    } else {
        ui::error("Could not open process");
    }
}

pub fn show_fds(pid: i32) {
    if let Ok(proc) = Process::new(pid) {
        if let Ok(fds) = proc.fd() {
            let fd_list: Vec<String> = fds.map(|fd| {
                let info = fd.unwrap();
                format!("{}: {:?}", info.fd, info.target)
            }).collect();
            if fd_list.is_empty() {
                ui::info("No open file descriptors found");
            } else {
                ui::info("Open File Descriptors:");
                for fd in fd_list {
                    println!("{}", fd.yellow());
                }
            }
        } else {
            ui::error("Could not read file descriptors");
        }
    } else {
        ui::error("Could not open process");
    }
}

pub fn show_maps(pid: i32) {
    if let Ok(proc) = Process::new(pid) {
        if let Ok(maps) = proc.maps() {
            ui::info("Memory Maps:");
            for map in maps {
                println!("{}", format!("{:?}", map).cyan());
            }
        } else {
            ui::error("Could not read memory maps");
        }
    } else {
        ui::error("Could not open process");
    }
}

pub fn direct_mem_mod(pid: i32) {
    ui::info("Direct memory modification is a dangerous operation and requires root/ptrace.");
    let addr: String = Input::new()
        .with_prompt("Enter memory address (hex)")
        .interact_text()
        .unwrap();
    let value: String = Input::new()
        .with_prompt("Enter new value (as string)")
        .interact_text()
        .unwrap();
    if Confirm::new().with_prompt(&format!("Write value '{}' to address {}?", value, addr)).interact().unwrap() {
        let mem_path = format!("/proc/{}/mem", pid);
        backup_file(&mem_path);
        if let Ok(mut file) = OpenOptions::new().write(true).custom_flags(libc::O_RDWR).open(&mem_path) {
            if let Ok(addr_num) = usize::from_str_radix(addr.trim_start_matches("0x"), 16) {
                if let Err(e) = file.seek(SeekFrom::Start(addr_num as u64)) {
                    ui::error(&format!("Failed to seek: {}", e));
                    return;
                }
                if let Err(e) = file.write_all(value.as_bytes()) {
                    ui::error(&format!("Failed to write memory: {}", e));
                    return;
                }
                ui::info(&format!("Wrote '{}' to address {} in PID {}", value, addr, pid));
                log_change(&format!("PID {}: wrote '{}' to address {}", pid, value, addr));
            } else {
                ui::error("Invalid address format");
            }
        } else {
            ui::error("Could not open mem for writing (need root)");
        }
    }
}
