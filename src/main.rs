mod memory;
mod ui;
mod utils;
mod types;

use std::error::Error;
use ui::menus::main_menu;
use utils::system::check_root;

fn main() -> Result<(), Box<dyn Error>> {
    // Verificar si se ejecuta como root
    if !check_root() {
        ui::display::print_header();
        println!("\n⚠️  WARNING: Not running as root. Some operations may fail.");
        utils::input::get_input("Press Enter to continue anyway...");
    }

    loop {
        match ui::menus::get_process() {
            Ok(pid) => {
                println!("🔗 Attaching to PID: {}...", pid);
                match memory::process::ProcessMemory::new(pid) {
                    Ok(mut process_mem) => {
                        println!("✅ Successfully attached!");
                        utils::input::get_input("Press Enter to continue...");
                        main_menu(&mut process_mem)?;
                    }
                    Err(e) => {
                        println!("❌ Failed to attach to process: {}", e);
                        utils::input::get_input("Press Enter to try another process...");
                    }
                }
            }
            Err(e) => {
                println!("❌ Error selecting process: {}", e);
                utils::input::get_input("Press Enter to try again...");
            }
        }
    }
}
