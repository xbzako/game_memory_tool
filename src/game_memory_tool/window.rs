use windows::Win32::Foundation::{HWND, POINT, RECT};
use windows::Win32::Graphics::Gdi::{ClientToScreen, ScreenToClient};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowW, GetClientRect, GetCursorPos};
use windows::core::*;

#[derive(Default, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

pub struct Window {
    hwnd: HWND,
}

impl Window {
    pub unsafe fn new(name: &str) -> Result<Self> {
        unsafe {
            let hwnd = FindWindowW(None, &HSTRING::from(name))?;
            Ok(Self { hwnd })
        }
    }

    pub unsafe fn get_client_rect(&self) -> Result<Rect> {
        unsafe {
            let mut rect = RECT::default();
            let _ = GetClientRect(self.hwnd, &mut rect)?;
            Ok(rect.into())
        }
    }

    pub unsafe fn get_cursor_pos() -> Point {
        let mut raw = POINT::default();
        unsafe {
            let _ = GetCursorPos(&mut raw);
            let point: Point = raw.into();
            point
        }
    }

    pub fn client_to_screen(&self, point: &mut Point) -> bool {
        let mut raw: POINT = (*point).into();
        unsafe {
            if !ClientToScreen(self.hwnd, &mut raw).as_bool() {
                return false;
            }
        }
        *point = raw.into();
        true
    }

    pub fn screen_to_client(&self, point: &mut Point) -> bool {
        let mut raw: POINT = (*point).into();
        unsafe {
            if !ScreenToClient(self.hwnd, &mut raw).as_bool() {
                return false;
            }
            *point = raw.into();
            true
        }
    }

    pub fn screen_to_client_from(hwnd: HWND, point: &mut Point) -> bool {
        let mut raw = (*point).into();
        unsafe {
            if !ScreenToClient(hwnd, &mut raw).as_bool() {
                false;
            }
            *point = raw.into();
            true
        }
    }
}

impl From<POINT> for Point {
    fn from(point: POINT) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}

impl From<Point> for POINT {
    fn from(point: Point) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}

impl From<RECT> for Rect {
    fn from(rect: RECT) -> Self {
        Self {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        }
    }
}

impl From<Rect> for RECT {
    fn from(rect: Rect) -> Self {
        Self {
            left: rect.left,
            top: rect.top,
            right: rect.right,
            bottom: rect.bottom,
        }
    }
}
