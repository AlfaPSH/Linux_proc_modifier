use crate::{proc_utils, mem_editor, ui};
use super::Cli;

pub fn run(cli: Cli) {
    // If automated mode, run automation
    if cli.auto {
        ui::print_title();
        if let (Some(pkg), Some(var), Some(new_value)) = (cli.pkg.as_ref(), cli.var.as_ref(), cli.new_value.as_ref()) {
            let pid = proc_utils::find_pid_by_package(pkg);
            if let Some(pid) = pid {
                mem_editor::automated_edit(pid, var, new_value);
            } else {
                ui::error("Could not find PID for package");
            }
        } else if let (Some(pkg), Some(search_value), Some(new_value)) = (cli.pkg.as_ref(), cli.search_value.as_ref(), cli.new_value.as_ref()) {
            let pid = proc_utils::find_pid_by_package(pkg);
            if let Some(pid) = pid {
                mem_editor::search_and_edit_value(pid, search_value, new_value);
            } else {
                ui::error("Could not find PID for package");
            }
        } else {
            ui::error("Missing arguments for automated mode");
        }
        return;
    }
    // Interactive menu
    ui::print_title();
    let pkg = ui::prompt_package_name();
    let pid = proc_utils::find_pid_by_package(&pkg);
    if pid.is_none() {
        ui::error("Could not find PID for package");
        return;
    }
    let pid = pid.unwrap();
    loop {
        match ui::main_menu() {
            1 => mem_editor::edit_env_vars(pid),
            2 => mem_editor::show_fds(pid),
            3 => mem_editor::show_maps(pid),
            4 => mem_editor::direct_mem_mod(pid),
            5 => break,
            _ => ui::error("Invalid option"),
        }
    }
}
