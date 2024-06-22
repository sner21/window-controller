use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{GetWindowLongA, GWL_STYLE, SetWindowLongA, SHOW_WINDOW_CMD, ShowWindow};


pub fn frameless(hwnd: HWND) {
    unsafe {
        let style = GetWindowLongA(hwnd, GWL_STYLE);
        println!("{}", style);
        SetWindowLongA(hwnd, GWL_STYLE, style - 0x00C00000);
        ShowWindow(hwnd, SHOW_WINDOW_CMD(3));
    }
}
pub fn overhead(hwnd: HWND) {
    unsafe {
        let style = GetWindowLongA(hwnd, GWL_STYLE);
        println!("{}", style);
        SetWindowLongA(hwnd, GWL_STYLE, style | 128u32 as i32 | 262144u32 as i32);
    }
}
