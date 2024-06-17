use winapi::{
    shared::windef::HWND,
};
mod utils;
pub struct window_controller{}
impl window_controller{
    pub fn frameless(hwnd: HWND){
        utils::controller::frameless(hwnd)
    }
    pub fn overhead(hwnd: HWND) {
        utils::controller::overhead(hwnd)
    }
}

