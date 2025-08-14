use super::process::ProcessMemory;
use std::error::Error;

pub fn search_pattern(
    process: &mut ProcessMemory,
    pattern: &[u8],
) -> Result<Vec<u64>, Box<dyn Error>> {
    let max_region_size = 100 * 1024 * 1024; // 100 MB

    // Clone regions to avoid borrowing conflicts
    let regions = process.regions.clone();

    let mut all_matches = Vec::new();
    for region in &regions {
        if !region.permissions.contains('r') {
            continue;
        }
        if (region.end - region.start) > max_region_size as u64 {
            continue;
        }

        match process.read_memory(region.start, (region.end - region.start) as usize) {
            Ok(data) => {
                for (i, window) in data.windows(pattern.len()).enumerate() {
                    if window == pattern {
                        all_matches.push(region.start + i as u64);
                    }
                }
            }
            Err(_) => continue,
        }
    }

    Ok(all_matches)
}

pub fn search_value<T: bytemuck::Pod + Copy>(
    process: &mut ProcessMemory,
    value: T,
) -> Result<Vec<u64>, Box<dyn Error>> {
    let bytes = bytemuck::bytes_of(&value);
    search_pattern(process, bytes)
}