
use super::memory::Memory;
use windows::Win32::Foundation::HANDLE;
use windows::core::Result;

#[derive(Clone, Copy, Debug)]
pub struct Address(pub Option<usize>);

impl Address {
    pub fn new(address: usize) -> Self {
        Self (Some(address))
    }

    pub fn calculate_address(
        handle: HANDLE,
        base_address: Address,
        offsets: &[i32],
    ) -> Result<usize> {
        let offsets_count = offsets.len();
        unsafe {
            let base_value = Memory::read_from::<i32>(handle, base_address)?;
            let mut current = base_value;
            for i in 0..offsets_count {
                current += offsets[i];
                if i < offsets_count - 1 {
                    current = Memory::read_from::<i32>(handle, current.into())?;
                }
            }
            Ok(current as usize)
        }
    }
}
