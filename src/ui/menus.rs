use super::super::memory::process::ProcessMemory;
use super::super::types::SearchFilter;
use super::display::{clear_screen, print_header, list_regions};
use super::super::utils::input::get_input;
use super::super::utils::system::get_pid_by_name;
use std::error::Error;
use std::io::stdout;
use crossterm::{ExecutableCommand, style::{Color, SetForegroundColor, ResetColor}};

pub fn get_process() -> Result<u32, Box<dyn Error>> {
    clear_screen();
    print_header();

    stdout()
        .execute(SetForegroundColor(Color::Green))
        .unwrap();
    println!("\n╔════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                              PROCESS SELECTION                                ║");
    println!("╠════════════════════════════════════════════════════════════════════════════════╣");
    println!("║ [1] Search by process name                                                     ║");
    println!("║ [2] Enter PID directly                                                         ║");
    println!("║ [0] Exit                                                                       ║");
    println!("╚════════════════════════════════════════════════════════════════════════════════╝");
    stdout()
        .execute(ResetColor)
        .unwrap();

    let choice = get_input("\n> Enter your choice: ");

    match choice.as_str() {
        "1" => {
            let process_name = get_input("Enter process name: ");
            println!("\nSearching for process '{}'...", process_name);

            let pids = get_pid_by_name(&process_name)?;

            if pids.is_empty() {
                println!("❌ No process found with name: {}", process_name);
                get_input("Press Enter to continue...");
                return get_process();
            }

            if pids.len() == 1 {
                println!("✅ Found process: PID {}", pids[0]);
                get_input("Press Enter to continue...");
                return Ok(pids[0]);
            }

            println!("\n🔍 Multiple processes found:");
            for (i, pid) in pids.iter().enumerate() {
                println!("  [{}] PID {}", i + 1, pid);
            }

            let selection = get_input(&format!("Select process (1-{}): ", pids.len()));
            let index: usize = selection.parse()?;

            if index == 0 || index > pids.len() {
                println!("❌ Invalid selection");
                get_input("Press Enter to try again...");
                return get_process();
            }

            Ok(pids[index - 1])
        }
        "2" => {
            let pid_str = get_input("Enter PID: ");
            match pid_str.parse::<u32>() {
                Ok(pid) => Ok(pid),
                Err(_) => {
                    println!("❌ Invalid PID");
                    get_input("Press Enter to try again...");
                    get_process()
                }
            }
        }
        "0" => {
            println!("👋 Goodbye!");
            std::process::exit(0);
        }
        _ => {
            println!("❌ Invalid choice");
            get_input("Press Enter to try again...");
            get_process()
        }
    }
}

pub fn main_menu(process_mem: &mut ProcessMemory) -> Result<(), Box<dyn Error>> {
    loop {
        clear_screen();
        print_header();

        stdout()
            .execute(SetForegroundColor(Color::Green))
            .unwrap();
        println!("\n╔════════════════════════════════════════════════════════════════════════════════╗");
        println!("║                              MAIN MENU - PID {}                              ║", process_mem.pid);
        println!("╠════════════════════════════════════════════════════════════════════════════════╣");
        println!("║ [1] 📋 List memory regions                                                     ║");
        println!("║ [2] 🔍 Search for values                                                       ║");
        println!("║ [3] 👁️  Read memory                                                            ║");
        println!("║ [4] ✏️  Write memory                                                           ║");
        println!("║ [5] 🔄 Filter search results                                                  ║");
        println!("║ [6] 📍 Manage saved addresses                                                 ║");
        println!("║ [7] 🔄 Change process                                                          ║");
        println!("║ [0] 🚪 Exit                                                                    ║");
        println!("╚════════════════════════════════════════════════════════════════════════════════╝");
        stdout()
            .execute(ResetColor)
            .unwrap();

        let choice = get_input("\n> Enter your choice: ");

        match choice.as_str() {
            "1" => list_regions_menu(process_mem)?,
            "2" => search_menu(process_mem)?,
            "3" => read_memory_menu(process_mem)?,
            "4" => write_memory_menu(process_mem)?,
            "5" => filter_menu(process_mem)?,
            "6" => manage_addresses_menu(process_mem)?,
            "7" => return Ok(()),
            "0" => {
                println!("👋 Goodbye!");
                std::process::exit(0);
            }
            _ => {
                println!("❌ Invalid choice");
                get_input("Press Enter to continue...");
            }
        }
    }
}

fn list_regions_menu(process_mem: &ProcessMemory) -> Result<(), Box<dyn Error>> {
    list_regions(process_mem.pid, &process_mem.regions);
    get_input("\nPress Enter to return to main menu...");
    Ok(())
}

fn search_menu(process_mem: &mut ProcessMemory) -> Result<(), Box<dyn Error>> {
    clear_screen();
    print_header();

    stdout()
        .execute(SetForegroundColor(Color::Magenta))
        .unwrap();
    println!("\n╔════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                               SEARCH VALUES                                   ║");
    println!("╠════════════════════════════════════════════════════════════════════════════════╣");
    println!("║ [1] 🔢 32-bit integer (i32)                                                   ║");
    println!("║ [2] 🔢 64-bit integer (i64)                                                   ║");
    println!("║ [3] 🔢 32-bit unsigned (u32)                                                  ║");
    println!("║ [4] 🔢 64-bit unsigned (u64)                                                  ║");
    println!("║ [5] 🔢 32-bit float (f32)                                                     ║");
    println!("║ [6] 🔢 64-bit float (f64)                                                     ║");
    println!("║ [7] 📝 String                                                                  ║");
    println!("║ [8] 🖥️  Raw bytes (hex)                                                       ║");
    println!("║ [0] ⬅️  Back to main menu                                                      ║");
    println!("╚════════════════════════════════════════════════════════════════════════════════╝");
    stdout()
        .execute(ResetColor)
        .unwrap();

    let choice = get_input("\n> Enter search type: ");

    let matches = match choice.as_str() {
        "1" => {
            let value_str = get_input("Enter i32 value: ");
            match value_str.parse::<i32>() {
                Ok(value) => {
                    println!("🔍 Searching for i32 value: {}...", value);
                    process_mem.search_value(value)?
                }
                Err(_) => {
                    println!("❌ Invalid i32 value");
                    get_input("Press Enter to continue...");
                    return Ok(());
                }
            }
        }
        "2" => {
            let value_str = get_input("Enter i64 value: ");
            match value_str.parse::<i64>() {
                Ok(value) => {
                    println!("🔍 Searching for i64 value: {}...", value);
                    process_mem.search_value(value)?
                }
                Err(_) => {
                    println!("❌ Invalid i64 value");
                    get_input("Press Enter to continue...");
                    return Ok(());
                }
            }
        }
        "3" => {
            let value_str = get_input("Enter u32 value: ");
            match value_str.parse::<u32>() {
                Ok(value) => {
                    println!("🔍 Searching for u32 value: {}...", value);
                    process_mem.search_value(value)?
                }
                Err(_) => {
                    println!("❌ Invalid u32 value");
                    get_input("Press Enter to continue...");
                    return Ok(());
                }
            }
        }
        "4" => {
            let value_str = get_input("Enter u64 value: ");
            match value_str.parse::<u64>() {
                Ok(value) => {
                    println!("🔍 Searching for u64 value: {}...", value);
                    process_mem.search_value(value)?
                }
                Err(_) => {
                    println!("❌ Invalid u64 value");
                    get_input("Press Enter to continue...");
                    return Ok(());
                }
            }
        }
        "5" => {
            let value_str = get_input("Enter f32 value: ");
            match value_str.parse::<f32>() {
                Ok(value) => {
                    println!("🔍 Searching for f32 value: {}...", value);
                    process_mem.search_value(value)?
                }
                Err(_) => {
                    println!("❌ Invalid f32 value");
                    get_input("Press Enter to continue...");
                    return Ok(());
                }
            }
        }
        "6" => {
            let value_str = get_input("Enter f64 value: ");
            match value_str.parse::<f64>() {
                Ok(value) => {
                    println!("🔍 Searching for f64 value: {}...", value);
                    process_mem.search_value(value)?
                }
                Err(_) => {
                    println!("❌ Invalid f64 value");
                    get_input("Press Enter to continue...");
                    return Ok(());
                }
            }
        }
        "7" => {
            let value = get_input("Enter string: ");
            println!("🔍 Searching for string: '{}'...", value);
            process_mem.search_pattern(value.as_bytes())?
        }
        "8" => {
            let value_str = get_input("Enter hex bytes (e.g., DEADBEEF): ");
            let value_str = value_str.trim();
            if value_str.len() % 2 != 0 {
                println!("❌ Invalid hex string length");
                get_input("Press Enter to continue...");
                return Ok(());
            }
            let bytes = (0..value_str.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&value_str[i..i + 2], 16))
                .collect::<Result<Vec<u8>, _>>();
            match bytes {
                Ok(bytes) => {
                    println!("🔍 Searching for bytes: {:02X?}...", bytes);
                    process_mem.search_pattern(&bytes)?
                }
                Err(_) => {
                    println!("❌ Invalid hex string");
                    get_input("Press Enter to continue...");
                    return Ok(());
                }
            }
        }
        "0" => return Ok(()),
        _ => {
            println!("❌ Invalid choice");
            get_input("Press Enter to continue...");
            return Ok(());
        }
    };

    println!("\n✅ Found {} matches:", matches.len());
    for (i, addr) in matches.iter().take(20).enumerate() {
        println!("  [{}] 0x{:016x}", i + 1, addr);
    }
    if matches.len() > 20 {
        println!("  ... and {} more matches", matches.len() - 20);
    }

    get_input("\nPress Enter to continue...");
    Ok(())
}

fn filter_menu(process_mem: &mut ProcessMemory) -> Result<(), Box<dyn Error>> {
    clear_screen();
    print_header();

    stdout()
        .execute(SetForegroundColor(Color::Magenta))
        .unwrap();
    println!("\n╔════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                              FILTER SEARCH RESULTS                            ║");
    println!("╠════════════════════════════════════════════════════════════════════════════════╣");
    println!("║ [1] 🔢 Exact value                                                             ║");
    println!("║ [2] 🔄 Changed value                                                           ║");
    println!("║ [3] 🔄 Unchanged value                                                         ║");
    println!("║ [4] 📈 Increased value                                                         ║");
    println!("║ [5] 📉 Decreased value                                                         ║");
    println!("║ [6] 📏 Value in range                                                          ║");
    println!("║ [0] ⬅️  Back to main menu                                                      ║");
    println!("╚════════════════════════════════════════════════════════════════════════════════╝");
    stdout()
        .execute(ResetColor)
        .unwrap();

    let choice = get_input("\n> Enter filter type: ");

    let (filter, value) = match choice.as_str() {
        "1" => {
            let value_str = get_input("Enter exact value (hex bytes or number): ");
            let bytes = if value_str.starts_with("0x") {
                let value_str = value_str.trim_start_matches("0x");
                (0..value_str.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&value_str[i..i + 2], 16))
                    .collect::<Result<Vec<u8>, _>>()
                    .map_err(|_| "Invalid hex string")?
            } else {
                // Asumimos que es un número (i32 por defecto, podría expandirse)
                let value: i32 = value_str.parse().map_err(|_| "Invalid number")?;
                bytemuck::bytes_of(&value).to_vec()
            };
            (SearchFilter::Exact, Some(bytes))
        }
        "2" => (SearchFilter::Changed, None),
        "3" => (SearchFilter::Unchanged, None),
        "4" => (SearchFilter::Increased, None),
        "5" => (SearchFilter::Decreased, None),
        "6" => {
            let min_str = get_input("Enter minimum value: ");
            let max_str = get_input("Enter maximum value: ");
            let min: f64 = min_str.parse().map_err(|_| "Invalid minimum value")?;
            let max: f64 = max_str.parse().map_err(|_| "Invalid maximum value")?;
            (SearchFilter::Range(min, max), None)
        }
        "0" => return Ok(()),
        _ => {
            println!("❌ Invalid choice");
            get_input("Press Enter to continue...");
            return Ok(());
        }
    };

    let matches = process_mem.filter_results(filter, value)?;
    println!("\n✅ Found {} matches after filtering:", matches.len());
    for (i, addr) in matches.iter().take(20).enumerate() {
        println!("  [{}] 0x{:016x}", i + 1, addr);
    }
    if matches.len() > 20 {
        println!("  ... and {} more matches", matches.len() - 20);
    }

    get_input("\nPress Enter to continue...");
    Ok(())
}

fn read_memory_menu(process_mem: &mut ProcessMemory) -> Result<(), Box<dyn Error>> {
    clear_screen();
    print_header();

    stdout()
        .execute(SetForegroundColor(Color::Blue))
        .unwrap();
    println!("\n╔════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                                READ MEMORY                                    ║");
    println!("╚════════════════════════════════════════════════════════════════════════════════╝");
    stdout()
        .execute(ResetColor)
        .unwrap();

    let addr_str = get_input("\nEnter address (hex, with or without 0x): ");
    let size_str = get_input("Enter size (bytes): ");

    let addr_clean = if addr_str.starts_with("0x") {
        &addr_str[2..]
    } else {
        &addr_str
    };

    match (u64::from_str_radix(addr_clean, 16), size_str.parse::<usize>()) {
        (Ok(addr), Ok(size)) => {
            println!("\n📖 Reading {} bytes from 0x{:016x}...", size, addr);
            match process_mem.read_memory(addr, size) {
                Ok(data) => {
                    stdout()
                        .execute(SetForegroundColor(Color::Yellow))
                        .unwrap();
                    println!("\n╔════════════════════════════════════════════════════════════════════════════════╗");
                    println!("║                              MEMORY CONTENTS                                  ║");
                    println!("╠════════════════════════════════════════════════════════════════════════════════╣");
                    for (i, chunk) in data.chunks(16).enumerate() {
                        print!("║ {:08x}: ", addr as u32 + (i * 16) as u32);
                        for byte in chunk {
                            print!("{:02x} ", byte);
                        }
                        for _ in chunk.len()..16 {
                            print!("   ");
                        }
                        print!("│ ");
                        for byte in chunk {
                            if *byte >= 32 && *byte <= 126 {
                                print!("{}", *byte as char);
                            } else {
                                print!(".");
                            }
                        }
                        println!(" ║");
                    }
                    println!("╚════════════════════════════════════════════════════════════════════════════════╝");
                    stdout()
                        .execute(ResetColor)
                        .unwrap();
                }
                Err(e) => println!("❌ Error reading memory: {}", e),
            }
        }
        _ => println!("❌ Invalid address or size"),
    }

    get_input("\nPress Enter to continue...");
    Ok(())
}

fn write_memory_menu(process_mem: &mut ProcessMemory) -> Result<(), Box<dyn Error>> {
    clear_screen();
    print_header();

    stdout()
        .execute(SetForegroundColor(Color::Blue))
        .unwrap();
    println!("\n╔════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                               WRITE MEMORY                                    ║");
    println!("╠════════════════════════════════════════════════════════════════════════════════╣");
    println!("║ [1] 🔢 32-bit integer (i32)                                                   ║");
    println!("║ [2] 🔢 64-bit integer (i64)                                                   ║");
    println!("║ [3] 🔢 32-bit unsigned (u32)                                                  ║");
    println!("║ [4] 🔢 64-bit unsigned (u64)                                                  ║");
    println!("║ [5] 🔢 32-bit float (f32)                                                     ║");
    println!("║ [6] 🔢 64-bit float (f64)                                                     ║");
    println!("║ [7] 📝 String                                                                  ║");
    println!("║ [8] 🖥️  Raw bytes (hex)                                                       ║");
    println!("║ [0] ⬅️  Back to main menu                                                      ║");
    println!("╚════════════════════════════════════════════════════════════════════════════════╝");
    stdout()
        .execute(ResetColor)
        .unwrap();

    let choice = get_input("\n> Enter data type: ");

    if choice == "0" {
        return Ok(());
    }

    let addr_str = get_input("Enter address (hex, with or without 0x): ");
    let addr_clean = if addr_str.starts_with("0x") {
        &addr_str[2..]
    } else {
        &addr_str
    };

    let addr = match u64::from_str_radix(addr_clean, 16) {
        Ok(addr) => addr,
        Err(_) => {
            println!("❌ Invalid address");
            get_input("Press Enter to continue...");
            return Ok(());
        }
    };

    let result = match choice.as_str() {
        "1" => {
            let value_str = get_input("Enter i32 value: ");
            match value_str.parse::<i32>() {
                Ok(value) => {
                    println!("✏️ Writing i32 value {} to 0x{:016x}...", value, addr);
                    let bytes = bytemuck::bytes_of(&value);
                    process_mem.write_memory(addr, bytes)
                }
                Err(_) => {
                    println!("❌ Invalid i32 value");
                    get_input("Press Enter to continue...");
                    return Ok(());
                }
            }
        }
        "2" => {
            let value_str = get_input("Enter i64 value: ");
            match value_str.parse::<i64>() {
                Ok(value) => {
                    println!("✏️ Writing i64 value {} to 0x{:016x}...", value, addr);
                    let bytes = bytemuck::bytes_of(&value);
                    process_mem.write_memory(addr, bytes)
                }
                Err(_) => {
                    println!("❌ Invalid i64 value");
                    get_input("Press Enter to continue...");
                    return Ok(());
                }
            }
        }
        "3" => {
            let value_str = get_input("Enter u32 value: ");
            match value_str.parse::<u32>() {
                Ok(value) => {
                    println!("✏️ Writing u32 value {} to 0x{:016x}...", value, addr);
                    let bytes = bytemuck::bytes_of(&value);
                    process_mem.write_memory(addr, bytes)
                }
                Err(_) => {
                    println!("❌ Invalid u32 value");
                    get_input("Press Enter to continue...");
                    return Ok(());
                }
            }
        }
        "4" => {
            let value_str = get_input("Enter u64 value: ");
            match value_str.parse::<u64>() {
                Ok(value) => {
                    println!("✏️ Writing u64 value {} to 0x{:016x}...", value, addr);
                    let bytes = bytemuck::bytes_of(&value);
                    process_mem.write_memory(addr, bytes)
                }
                Err(_) => {
                    println!("❌ Invalid u64 value");
                    get_input("Press Enter to continue...");
                    return Ok(());
                }
            }
        }
        "5" => {
            let value_str = get_input("Enter f32 value: ");
            match value_str.parse::<f32>() {
                Ok(value) => {
                    println!("✏️ Writing f32 value {} to 0x{:016x}...", value, addr);
                    let bytes = bytemuck::bytes_of(&value);
                    process_mem.write_memory(addr, bytes)
                }
                Err(_) => {
                    println!("❌ Invalid f32 value");
                    get_input("Press Enter to continue...");
                    return Ok(());
                }
            }
        }
        "6" => {
            let value_str = get_input("Enter f64 value: ");
            match value_str.parse::<f64>() {
                Ok(value) => {
                    println!("✏️ Writing f64 value {} to 0x{:016x}...", value, addr);
                    let bytes = bytemuck::bytes_of(&value);
                    process_mem.write_memory(addr, bytes)
                }
                Err(_) => {
                    println!("❌ Invalid f64 value");
                    get_input("Press Enter to continue...");
                    return Ok(());
                }
            }
        }
        "7" => {
            let value = get_input("Enter string: ");
            println!("✏️ Writing string '{}' to 0x{:016x}...", value, addr);
            process_mem.write_memory(addr, value.as_bytes())
        }
        "8" => {
            let value_str = get_input("Enter hex bytes (e.g., DEADBEEF): ");
            let value_str = value_str.trim();
            if value_str.len() % 2 != 0 {
                println!("❌ Invalid hex string length");
                get_input("Press Enter to continue...");
                return Ok(());
            }
            let bytes = (0..value_str.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&value_str[i..i + 2], 16))
                .collect::<Result<Vec<u8>, _>>();
            match bytes {
                Ok(bytes) => {
                    println!("✏️ Writing bytes {:02X?} to 0x{:016x}...", bytes, addr);
                    process_mem.write_memory(addr, &bytes)
                }
                Err(_) => {
                    println!("❌ Invalid hex string");
                    get_input("Press Enter to continue...");
                    return Ok(());
                }
            }
        }
        _ => {
            println!("❌ Invalid choice");
            get_input("Press Enter to continue...");
            return Ok(());
        }
    };

    match result {
        Ok(_) => println!("✅ Successfully wrote to memory!"),
        Err(e) => println!("❌ Error writing memory: {}", e),
    }

    let desc = get_input("Enter description for this address (or leave empty): ");
    if !desc.is_empty() {
        process_mem.save_address(addr, desc);
    }

    get_input("Press Enter to continue...");
    Ok(())
}

fn manage_addresses_menu(process_mem: &mut ProcessMemory) -> Result<(), Box<dyn Error>> {
    clear_screen();
    print_header();

    stdout()
        .execute(SetForegroundColor(Color::Cyan))
        .unwrap();
    println!("\n╔════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                             MANAGE SAVED ADDRESSES                            ║");
    println!("╠════════════════════════════════════════════════════════════════════════════════╣");
    println!("║ [1] 📋 List saved addresses                                                    ║");
    println!("║ [2] ✏️  Edit saved address                                                     ║");
    println!("║ [0] ⬅️  Back to main menu                                                      ║");
    println!("╚════════════════════════════════════════════════════════════════════════════════╝");
    stdout()
        .execute(ResetColor)
        .unwrap();

    let choice = get_input("\n> Enter choice: ");

    match choice.as_str() {
        "1" => {
            clear_screen();
            print_header();
            println!("\n╔════════════════════════════════════════════════════════════════════════════════╗");
            println!("║                             SAVED ADDRESSES                                   ║");
            println!("╠════════════════════════════════════════════════════════════════════════════════╣");
            println!("║ {:<4} │ {:<16} │ {}", "ID", "Address", "Description");
            println!("╠════════════════════════════════════════════════════════════════════════════════╣");
            for (i, (addr, desc)) in process_mem.get_saved_addresses().iter().enumerate() {
                println!("║ {:<4} │ {:016x} │ {}", i + 1, addr, desc);
            }
            println!("╚════════════════════════════════════════════════════════════════════════════════╝");
            get_input("\nPress Enter to continue...");
        }
        "2" => {
            let index_str = get_input("Enter address ID to edit: ");
            let index: usize = match index_str.parse() {
                Ok(idx) => idx,
                Err(_) => {
                    println!("❌ Invalid ID");
                    get_input("Press Enter to continue...");
                    return Ok(());
                }
            };
            if index == 0 || index > process_mem.get_saved_addresses().len() {
                println!("❌ Invalid ID");
                get_input("Press Enter to continue...");
                return Ok(());
            }
            let (_addr, _) = process_mem.get_saved_addresses()[index - 1];
            write_memory_menu(process_mem)?;
        }
        "0" => return Ok(()),
        _ => {
            println!("❌ Invalid choice");
            get_input("Press Enter to continue...");
        }
    }
    Ok(())
}
