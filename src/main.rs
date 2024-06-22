use clap::Parser;
use inquire::{Select};
use std::collections::HashMap;
use windows::core::*;

use windows::{
    core::{
        PCWSTR,
        HSTRING
    },
    Win32::{
        UI::WindowsAndMessaging::{
            FindWindowW,
            WINDOWINFO,
            EnumWindows,
            GetWindowTextW,
            GetWindowInfo,
        },
    },
};
mod utils;
use windows::Win32::Foundation::{
    LPARAM,
    HWND,
    BOOL,
};
use windows::Win32::UI::WindowsAndMessaging::{GetWindowLongA, GWL_STYLE};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "")]
    mode: String,
    #[arg(short, long, default_value = "")]
    title: String,
}

static mut PROCESS: Vec<String> = vec![];

extern "system" fn enum_window(window: HWND, _: LPARAM) -> BOOL {
    unsafe {
        let mut text: [u16; 512] = [0; 512];
        let len = GetWindowTextW(window, &mut text);
        let text = String::from_utf16_lossy(&text[..len as usize] );
        let mut info = WINDOWINFO {
            cbSize: core::mem::size_of::<WINDOWINFO>() as u32,
            ..Default::default()
        };
        GetWindowInfo(window, &mut info).unwrap();
        if !text.is_empty() && (info.rcWindow.left != 0 || info.rcWindow.top != 0) && (info.dwStyle | 268435456u32 == info.dwStyle) {
            PROCESS.push(text)
        }
        true.into()
    }
}

fn main() {
    unsafe {
        EnumWindows(Some(enum_window), LPARAM(0));
    }
    let mut mode_eunm: HashMap<&str, fn( HWND)> = HashMap::new();
    mode_eunm.insert("全屏无边框",utils::controller::frameless);
    mode_eunm.insert("顶置", utils::controller::overhead);
    let mut title: &str = "";
    let args = Args::parse();
    let options: Vec<&str> = vec!["全屏无边框", "顶置"];
    let mut o: Vec<&str> = vec![];
    unsafe {
        for i in 0..PROCESS.len() {
            o.push(PROCESS[i].as_str());
        }
    }
    if args.title == "" {
        title = Select::new("选择窗口", o).prompt().unwrap();
    } else {
        title = &*args.title
    }
    let mut ans: Select<&str> = Select::new("选择模式", options);
    ans.help_message = Option::from("方向键切换/enter选择");
    unsafe {
        let hwnd = FindWindowW(None,  PCWSTR(HSTRING::from(title).as_ptr()));
        let style = GetWindowLongA(hwnd, GWL_STYLE);
        println!("{}", style);
        if args.mode!="" {
            mode_eunm[args.mode.as_str()](hwnd)
        } else {
            match ans.prompt() {
                Ok(choice) => mode_eunm[choice](hwnd),
                Err(_) => println!("There was an error, please try again"),
            }
        }
    }
}

