use super::region::MemoryRegion;
use super::search::{search_pattern, search_value};
use crate::types::SearchFilter;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::error::Error;


#[derive(Debug)]
pub struct ProcessMemory {
    pub pid: u32,
    mem_file: File,
    pub regions: Vec<MemoryRegion>,
    pub search_results: Vec<(u64, Vec<u8>)>, // Direcciones y valores encontrados
    saved_addresses: Vec<(u64, String)>, // Direcciones guardadas con descripción
}

impl ProcessMemory {
    pub fn new(pid: u32) -> Result<Self, Box<dyn Error>> {
        let mem_path = format!("/proc/{}/mem", pid);
        let mem_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&mem_path)?;

        let regions = MemoryRegion::parse_maps(pid)?;

        Ok(ProcessMemory {
            pid,
            mem_file,
            regions,
            search_results: Vec::new(),
            saved_addresses: Vec::new(),
        })
    }

    pub fn read_memory(&mut self, address: u64, size: usize) -> Result<Vec<u8>, Box<dyn Error>> {
        self.mem_file.seek(SeekFrom::Start(address))?;
        let mut buffer = vec![0u8; size];
        self.mem_file.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    pub fn write_memory(&mut self, address: u64, data: &[u8]) -> Result<(), Box<dyn Error>> {
        self.mem_file.seek(SeekFrom::Start(address))?;
        self.mem_file.write_all(data)?;
        self.mem_file.flush()?;
        Ok(())
    }

    pub fn search_pattern(&mut self, pattern: &[u8]) -> Result<Vec<u64>, Box<dyn Error>> {
        let matches = search_pattern(self, pattern)?;
        self.search_results = matches.iter().map(|&addr| (addr, pattern.to_vec())).collect();
        Ok(matches)
    }

    pub fn search_value<T: bytemuck::Pod + Copy>(
        &mut self,
        value: T,
    ) -> Result<Vec<u64>, Box<dyn Error>> {
        let matches = search_value(self, value)?;
        let bytes = bytemuck::bytes_of(&value).to_vec();
        self.search_results = matches.iter().map(|&addr| (addr, bytes.clone())).collect();
        Ok(matches)
    }

    pub fn filter_results(
        &mut self,
        filter: SearchFilter,
        value: Option<Vec<u8>>,
    ) -> Result<Vec<u64>, Box<dyn Error>> {
        let mut new_results = Vec::new();
        let search_results = self.search_results.clone(); // Clone to avoid borrowing conflicts
        for (addr, old_value) in search_results.iter() {
            let current_value = self.read_memory(*addr, old_value.len())?;
            match filter {
                SearchFilter::Exact => {
                    if let Some(ref val) = value {
                        if current_value == *val {
                            new_results.push((*addr, current_value));
                        }
                    }
                }
                SearchFilter::Changed => {
                    if current_value != *old_value {
                        new_results.push((*addr, current_value));
                    }
                }
                SearchFilter::Unchanged => {
                    if current_value == *old_value {
                        new_results.push((*addr, current_value));
                    }
                }
                SearchFilter::Increased => {
                    if current_value > *old_value {
                        new_results.push((*addr, current_value));
                    }
                }
                SearchFilter::Decreased => {
                    if current_value < *old_value {
                        new_results.push((*addr, current_value));
                    }
                }
                SearchFilter::Range(min, max) => {
                    if let Ok(val) = bytemuck::try_pod_read_unaligned::<f64>(&current_value) {
                        if val >= min && val <= max {
                            new_results.push((*addr, current_value));
                        }
                    }
                }
            }
        }
        self.search_results = new_results;
        Ok(self.search_results.iter().map(|(addr, _)| *addr).collect())
    }

    pub fn save_address(&mut self, address: u64, description: String) {
        self.saved_addresses.push((address, description));
    }

    pub fn get_saved_addresses(&self) -> &[(u64, String)] {
        &self.saved_addresses
    }
}

