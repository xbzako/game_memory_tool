use std::os::raw::c_void;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
use windows::core::Result;

use std::mem::MaybeUninit;
use std::mem::size_of;
use windows::Win32::System::Memory::{
    MEM_COMMIT, MEMORY_BASIC_INFORMATION, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE,
    PAGE_EXECUTE_WRITECOPY, PAGE_GUARD, PAGE_NOACCESS, PAGE_PROTECTION_FLAGS, PAGE_READONLY,
    PAGE_READWRITE, PAGE_WRITECOPY, VirtualQueryEx,
};

pub struct Memory {
    handle: HANDLE,
}

pub struct MemoryInfo {
    pub base_address: usize,
    pub region_size: usize,
}

impl Default for MemoryInfo {
    fn default() -> Self {
        Self {
            base_address: 0x00000000,
            region_size: 0,
        }
    }
}

impl Memory {
    pub fn new(handle: HANDLE) -> Self {
        Self { handle }
    }

    pub unsafe fn read<T>(&self, address: usize) -> Result<T> {
        let mut buffer = MaybeUninit::<T>::uninit();
        unsafe {
            ReadProcessMemory(
                self.handle,
                address as *const c_void,
                buffer.as_mut_ptr() as *mut c_void,
                size_of::<T>(),
                None,
            )?;
        }
        unsafe { Ok(buffer.assume_init()) }
    }

    pub unsafe fn write<T>(&self, address: usize, write_value: T) -> Result<usize> {
        let mut write_bytes = 0;
        unsafe {
            WriteProcessMemory(
                self.handle,
                address as *mut c_void,
                &write_value as *const T as *const c_void,
                size_of::<T>(),
                Some(&mut write_bytes),
            )?;
        }
        Ok(write_bytes)
    }

    pub unsafe fn read_assembly(handle: HANDLE, address: usize, size: usize) -> Result<Vec<u8>> {
        let mut read_bytes = vec![0u8; size];
        let mut bytes_read = 0;
        unsafe {
            ReadProcessMemory(
                handle,
                address as *const c_void,
                read_bytes.as_mut_ptr() as *mut c_void,
                size,
                Some(&mut bytes_read),
            )?;
        }
        read_bytes.truncate(bytes_read);
        Ok(read_bytes)
    }

    pub unsafe fn read_from<T>(handle: HANDLE, address: usize) -> Result<T> {
        let mut buffer = MaybeUninit::<T>::uninit();
        unsafe {
            ReadProcessMemory(
                handle,
                address as *const c_void,
                buffer.as_mut_ptr() as *mut c_void,
                size_of::<T>(),
                None,
            )?;
        }
        unsafe { Ok(buffer.assume_init()) }
    }

    pub unsafe fn write_assembly(&self, address: usize, bytes: &[u8]) -> Result<usize> {
        let mut write_bytes = 0;
        unsafe {
            WriteProcessMemory(
                self.handle,
                address as *mut c_void,
                bytes.as_ptr() as *const c_void,
                bytes.len(),
                Some(&mut write_bytes),
            )?;
        }
        Ok(write_bytes)
    }

    pub fn is_memory_readable(
        handle: HANDLE,
        address: usize,
        memory_info: &mut MemoryInfo,
    ) -> bool {
        let mut memory_basic_information = MEMORY_BASIC_INFORMATION::default();
        let read_bytes = unsafe {
            VirtualQueryEx(
                handle,
                Some(address as *const c_void),
                &mut memory_basic_information,
                size_of::<MEMORY_BASIC_INFORMATION>(),
            )
        };

        memory_info.base_address = memory_basic_information.BaseAddress as usize;
        memory_info.region_size = memory_basic_information.RegionSize;

        if read_bytes == 0 {
            return false;
        }
        if memory_basic_information.State != MEM_COMMIT {
            return false;
        }

        if memory_basic_information.Protect & PAGE_GUARD != PAGE_PROTECTION_FLAGS(0)
            || memory_basic_information.Protect & PAGE_NOACCESS != PAGE_PROTECTION_FLAGS(0)
        {
            return false;
        }

        let readable_flags = PAGE_READONLY
            | PAGE_READWRITE
            | PAGE_WRITECOPY
            | PAGE_EXECUTE_READ
            | PAGE_EXECUTE_READWRITE
            | PAGE_EXECUTE_WRITECOPY;

        if memory_basic_information.Protect & readable_flags == PAGE_PROTECTION_FLAGS(0) {
            return false;
        }

        true
    }

    pub unsafe fn scan_pattern(
        handle: HANDLE,
        pattern: &[u8],
        start: usize,
        end: usize,
    ) -> Vec<usize> {
        let mut collect = Vec::new();
        let mut memory_info = MemoryInfo::default();
        let mut current = start;
        while current < end {
            if Memory::is_memory_readable(handle, current, &mut memory_info) {
                unsafe {
                    let bytes = Memory::read_assembly(
                        handle,
                        memory_info.base_address,
                        memory_info.region_size,
                    )
                    .expect("读取字节错误");

                    let patters_index = brute_force_all(&bytes, pattern);
                    for index in patters_index {
                        collect.push(memory_info.base_address + index);
                    }
                }
            }
            current = memory_info.base_address + memory_info.region_size;
        }
        collect
    }
}

pub fn brute_force_all(text: &[u8], pattern: &[u8]) -> Vec<usize> {
    let mut collect = Vec::new();
    let text_len = text.len();
    let pattern_len = pattern.len();

    let mut i = 0;
    let mut j = 0;
    while i < text_len && j < pattern_len {
        if text[i] == pattern[j] {
            i += 1;
            j += 1;
            if j == pattern_len {
                collect.push(i - j);
                i = i - j + 1;
                j = 0;
            }
        } else {
            i = i - j + 1;
            j = 0;
        }
    }
    collect
}

impl Memory {
    pub fn bytes_string_to_vec_u8(pattern: &str) -> Vec<u8> {
        pattern
            .split_whitespace()
            .map(|s| u8::from_str_radix(s, 16).unwrap())
            .collect()
    }
}
