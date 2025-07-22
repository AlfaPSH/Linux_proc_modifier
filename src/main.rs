mod menu;
mod proc_utils;
mod mem_editor;
mod ui;

use clap::Parser;
use log::info;

#[derive(Parser)]
#[command(name = "proc_editor-rs")]
#[command(about = "Dynamic variable/memory editor for Android processes", long_about = None)]
struct Cli {
    /// Package name of the target app
    #[arg(long)]
    pkg: Option<String>,
    /// Variable name to search and modify
    #[arg(long)]
    var: Option<String>,
    /// Value to search for
    #[arg(long)]
    search_value: Option<String>,
    /// New value to set
    #[arg(long)]
    new_value: Option<String>,
    /// Run in automated mode
    #[arg(long)]
    auto: bool,
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    info!("Starting proc_editor-rs");
    menu::run(cli);
}
