use std::ffi::OsString;
use std::mem::size_of;
use std::os::windows::ffi::OsStringExt;

use windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW,
        TH32CS_SNAPPROCESS,
    },
    System::Threading::OpenProcess,
    System::Threading::PROCESS_ALL_ACCESS,
};
use windows::core::Result;

pub struct Process<'a> {
    name: &'a str,
    pid: Option<u32>,
    handle: HANDLE,
}

impl<'a> Drop for Process<'a> {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.handle);
        }
    }
}

impl<'a> Process<'a> {
    pub unsafe fn new(name: &'a str) -> Result<Self> {
        unsafe {
            let pid = Process::get_pid_by_name(name)?;
            let handle = Process::open_handle(pid.unwrap_or_default())?;
            Ok(Self { handle, name, pid })
        }
    }

    pub unsafe fn get_pid_by_name(name: &str) -> Result<Option<u32>> {
        unsafe {
            let handle = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;

            let mut pe32 = PROCESSENTRY32W {
                dwSize: size_of::<PROCESSENTRY32W>() as u32,
                ..Default::default()
            };

            let mut ok = Process32FirstW(handle, &mut pe32).is_ok();
            while ok {
                if c_char_to_os_string(&pe32.szExeFile) == name {
                    CloseHandle(handle)?;
                    return Ok(Some(pe32.th32ProcessID));
                }
                ok = Process32NextW(handle, &mut pe32).is_ok();
            }
            CloseHandle(handle)?;
            Ok(None)
        }
    }

    pub unsafe fn open_handle(pid: u32) -> Result<HANDLE> {
        unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, pid) }
    }
}

impl<'a> Process<'a> {
    pub unsafe fn handle(&self) -> HANDLE {
        self.handle
    }

    pub unsafe fn name(&self) -> &'a str {
        self.name
    }

    pub unsafe fn pid(&self) -> Option<u32> {
        self.pid
    }
}

pub fn c_char_to_os_string(buf: &[u16]) -> OsString {
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    let s = OsString::from_wide(&buf[..len]);
    s
}
