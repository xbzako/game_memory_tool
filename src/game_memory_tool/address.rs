use std::ffi::c_void;
use std::fmt::{Display};
use std::ops::Add;


use super::memory::Memory;
use windows::Win32::Foundation::HANDLE;
use windows::core::Result;

impl Add<usize> for Address {
    type Output = usize;
    fn add(self, rhs: usize) -> usize {
        self.0 + rhs
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:08X}", self.0)
    }
}

impl From<*mut c_void> for Address {
    fn from(value: *mut c_void) -> Self {
        Address(value as usize)
    }
}

impl From<usize> for Address {
    fn from(value: usize) -> Self {
        Address(value)
    }
}


impl From<Address> for usize {
    fn from(value: Address) -> usize {
        value.0
    }
}

impl From<Address> for *const c_void {
    fn from(value: Address) ->*const c_void {
        value.0 as *const c_void
    }
}



impl From<Address> for *mut c_void {
    fn from(value: Address) ->*mut c_void {
        value.0 as *mut c_void
    }
}

#[derive(Clone, Copy)]
pub struct Address(pub usize);

impl Address {
    pub fn new(address: usize) -> Self {
        Self (address)
    }

    pub fn calculate_address(
        handle: HANDLE,
        base_address: usize,
        offsets: &[i32],
    ) -> Result<usize> {
        let offsets_count = offsets.len();
        unsafe {
            let base_value = Memory::read_from::<i32>(handle, base_address)?;
            let mut current = base_value;
            for i in 0..offsets_count {
                current += offsets[i];
                if i < offsets_count - 1 {
                    current = Memory::read_from::<i32>(handle, current as usize)?;
                }
            }
            Ok(current as usize)
        }
    }
}
