use windows::Win32::Foundation::HWND;


mod utils;
pub struct WindowController;
impl WindowController {
    pub fn frameless(hwnd: HWND){
        utils::controller::frameless(hwnd)
    }
    pub fn overhead(hwnd: HWND) {
        utils::controller::overhead(hwnd)
    }
}

