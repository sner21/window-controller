use winapi::{
    shared::windef::HWND,
    um::{
        winnt::{LONG, LPCSTR},
        winuser::{
            {
                FindWindowA,
                GetWindowLongA,
                SetWindowLongA,
                GWL_EXSTYLE,
                WS_EX_TRANSPARENT,
                WS_EX_LAYERED,
                WS_EX_TOOLWINDOW,
                WS_EX_APPWINDOW,
                GWL_STYLE,
                ShowWindow,
            }
        }
    },
    ctypes::{__int32},
};

pub fn frameless(hwnd: HWND) {
    unsafe {
        let style = GetWindowLongA(hwnd, GWL_STYLE);
        println!("{}", style);
        SetWindowLongA(hwnd, GWL_STYLE, style - 0x00C00000);
        ShowWindow(hwnd, 3);
    }
}

pub fn overhead(hwnd: HWND) {
    unsafe {
        let style = GetWindowLongA(hwnd, GWL_STYLE);
        SetWindowLongA(hwnd, GWL_STYLE, style | 128u32 as i32 | 262144u32 as i32);
    }
}
