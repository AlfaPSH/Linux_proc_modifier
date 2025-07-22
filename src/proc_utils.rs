use procfs::process::all_processes;

pub fn find_pid_by_package(pkg: &str) -> Option<i32> {
    for proc in all_processes().ok()? {
        if let Ok(p) = proc {
            if let Ok(cmdline) = p.cmdline() {
                if cmdline.iter().any(|c| c.contains(pkg)) {
                    return Some(p.pid);
                }
            }
        }
    }
    None
}
