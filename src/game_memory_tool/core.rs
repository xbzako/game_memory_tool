use super::address::Address;
use super::memory::Memory;
use super::process::Process;
use windows::Win32::Foundation::HANDLE;
pub type Result<T> = core::result::Result<T, windows_result::Error>;

pub struct Core<'a> {
    process: Process<'a>,
    memory: Memory,
}

impl<'a> Core<'a> {
    pub fn new(name: &'a str) -> Result<Self> {
        unsafe {
            let process = Process::new(name)?;
            let memory = Memory::new(process.handle());
            Ok(Self { process, memory })
        }
    }

    pub fn read<T>(&self, address: usize) -> Result<T> {
        unsafe { self.memory.read(address) }
    }

    pub fn write<T>(&self, address: usize, write_value: T) -> Result<usize> {
        unsafe { self.memory.write(address, write_value) }
    }

    pub fn get_pid_by_name(name: &str) -> Result<Option<u32>> {
        unsafe { Process::get_pid_by_name(name) }
    }

    pub fn calculate_address(&self, base_address: usize, offsets: &[i32]) -> Result<usize> {
        unsafe { Address::calculate_address(self.process.handle(), base_address, offsets) }
    }

    pub fn write_assembly(&self, address: usize, bytes: &[u8]) -> Result<usize> {
        unsafe { self.memory.write_assembly(address, bytes) }
    }

    pub fn scan_pattern(&self, pattern: &[u8], start: usize, end: usize) -> Vec<usize> {
        unsafe { Memory::scan_pattern(self.handle(), pattern, start, end) }
    }
}

impl<'a> Core<'a> {
    pub fn handle(&self) -> HANDLE {
        unsafe { self.process.handle() }
    }

    pub fn name(&self) -> &str {
        unsafe { self.process.name() }
    }

    pub fn pid(&self) -> Option<u32> {
        unsafe { self.process.pid() }
    }
}

pub fn bytes_string_to_vec_u8(pattern: &str) -> Vec<u8> {
    Memory::bytes_string_to_vec_u8(pattern)
}