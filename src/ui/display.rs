use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::stdout;
use super::super::memory::region::MemoryRegion;

pub fn clear_screen() {
    stdout()
        .execute(crossterm::terminal::Clear(crossterm::terminal::ClearType::All))
        .unwrap();
    stdout()
        .execute(crossterm::cursor::MoveTo(0, 0))
        .unwrap();
}

pub fn print_header() {
    stdout()
        .execute(SetForegroundColor(Color::Cyan))
        .unwrap();
    println!("╔════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                        CHEATENGINE - LINUX MEMORY TOOL                        ║");
    println!("║                      Root Shell Memory Manipulator                            ║");
    println!("╚════════════════════════════════════════════════════════════════════════════════╝");
    stdout()
        .execute(ResetColor)
        .unwrap();
}

pub fn list_regions(pid: u32, regions: &[MemoryRegion]) {
    clear_screen();
    print_header();
    stdout()
        .execute(SetForegroundColor(Color::Yellow))
        .unwrap();
    println!("\n╔════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                            MEMORY REGIONS FOR PID {}                            ║", pid);
    println!("╠════════════════════════════════════════════════════════════════════════════════╣");
    println!("║ {:<16} │ {:<16} │ {:<8} │ {}", "Start", "End", "Perms", "Path");
    println!("╠════════════════════════════════════════════════════════════════════════════════╣");

    for region in regions {
        println!(
            "║ {:016x} │ {:016x} │ {:<8} │ {}",
            region.start,
            region.end,
            region.permissions,
            if region.pathname.len() > 40 {
                format!("{}...", &region.pathname[..37])
            } else {
                region.pathname.clone()
            }
        );
    }
    println!("╚════════════════════════════════════════════════════════════════════════════════╝");
    stdout()
        .execute(ResetColor)
        .unwrap();
}
