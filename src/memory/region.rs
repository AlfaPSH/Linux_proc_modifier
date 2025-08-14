use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub start: u64,
    pub end: u64,
    pub permissions: String,
    pub pathname: String,
}

impl MemoryRegion {
    pub fn parse_maps(pid: u32) -> Result<Vec<MemoryRegion>, Box<dyn Error>> {
        let maps_path = format!("/proc/{}/maps", pid);
        let file = File::open(maps_path)?;
        let reader = BufReader::new(file);
        let mut regions = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let addr_range: Vec<&str> = parts[0].split('-').collect();
                if addr_range.len() == 2 {
                    let start = u64::from_str_radix(addr_range[0], 16)?;
                    let end = u64::from_str_radix(addr_range[1], 16)?;
                    let permissions = parts[1].to_string();
                    let pathname = if parts.len() > 5 {
                        parts[5..].join(" ")
                    } else {
                        String::new()
                    };

                    regions.push(MemoryRegion {
                        start,
                        end,
                        permissions,
                        pathname,
                    });
                }
            }
        }

        Ok(regions)
    }
}
