use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{GetWindowLongA, GWL_STYLE,SetWindowLongA, SHOW_WINDOW_CMD, ShowWindow,SetWindowPos,HWND_TOPMOST,SWP_NOMOVE, SWP_NOSIZE};


pub fn frameless(hwnd: HWND) {
    unsafe {
        let style = GetWindowLongA(hwnd, GWL_STYLE);
        ShowWindow(hwnd, SHOW_WINDOW_CMD(3));
        SetWindowLongA(hwnd, GWL_STYLE, style - 12582912u32 as i32 );
        ShowWindow(hwnd, SHOW_WINDOW_CMD(3));
    }
}
pub fn overhead(hwnd: HWND) {
    unsafe {
        SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE);
    }
}
