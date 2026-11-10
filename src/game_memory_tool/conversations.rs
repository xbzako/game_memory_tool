use std::ffi::c_void;
use std::fmt::Display;
use std::ops::Add;
use super::address::Address;

// 为 Address 类型实现打印地址样式的 trait
impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:08X}", self.0.unwrap_or_default())
    }
}

// usize 类型 实现与 Address相加的 trait
impl Add<usize> for Address {
    type Output = usize;
    fn add(self, rhs: usize) -> usize {
        self.0.unwrap_or_default() + rhs
    }
}

// 为 uszie 类型实现转换到 Address的 trait
impl From<usize> for Address {
    fn from(value: usize) -> Self {
        Address(Some(value))
    }
}
// 为 Address 类型实现转换到 usize的 trait
impl From<Address> for usize {
    fn from(value: Address) -> usize {
        value.0.unwrap_or_default()
    }
}

// 为 Address 类型实现转换到 *const c_void 的 triat
impl From<Address> for *const c_void {
    fn from(value: Address) ->*const c_void {
        value.0.unwrap_or_default() as *const c_void
    }
}
// 为 *const c_void 类型实现转换到 Address 的 triat
impl From<*const c_void> for Address {
    fn from(value: *const c_void) -> Address {
        Address(Some(value as usize))
    }
}

// 为 Address 类型实现转换到 *mut c_void 的 trait
impl From<Address> for *mut c_void {
    fn from(value: Address) -> *mut c_void {
        value.0.unwrap_or_default() as *mut c_void
    }
}
// 为 *mut c_void 类型实现转换到 Address 的triat
impl From<*mut c_void> for Address {
    fn from(value: *mut c_void) -> Self {
        Address(Some(value as usize))
    }
}

impl From<i32> for Address {
    fn from(value: i32) -> Self {
        Address(Some(value as usize))
    }
}