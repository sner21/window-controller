use clap::Parser;
use inquire::{Select};
use std::collections::HashMap;
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
    let binding = std::env::var("LANG").unwrap();
    let mut os_lang = binding.as_str().split(".").collect::<Vec<_>>()[0];
    let lang_eunm: HashMap<&str, [&str; 6]> = HashMap::from([
        ("zh_CN", ["全屏无边框","顶置","选择窗口","选择模式","方向键切换/enter选择","There was an error, please try again"]),
        ("en_US",  ["Full screen without borders", "Top placement","Select window", "Selection mode", "Direction key switching/enter selection","There was an error, please try again"]),
        ("ja_JP",  ["全画面ボーダレス","最前面表示","ウィンドウを選択する","選択モード","矢印キーで切り替え / エンターキーで選択","There was an error, please try again"]),
    ]);
    if !lang_eunm.contains_key(os_lang){
        os_lang = "en_US"
    }
    unsafe {
        EnumWindows(Some(enum_window), LPARAM(0));
    }
    let mut mode_eunm: HashMap<&str, fn( HWND)> = HashMap::new();
    mode_eunm.insert(lang_eunm[os_lang][0],utils::controller::frameless);
    mode_eunm.insert(lang_eunm[os_lang][1], utils::controller::overhead);
    let args = Args::parse();
    let options: Vec<&str> = vec![lang_eunm[os_lang][0], lang_eunm[os_lang][1]];
    let mut o: Vec<&str> = vec![];
    unsafe {
        for i in 0..PROCESS.len() {
            o.push(PROCESS[i].as_str());
        }
    }
    let mut title: &str = "";
    if args.title == "" {
        title = Select::new(lang_eunm[os_lang][2], o).prompt().unwrap();
    } else {
        title = &*args.title
    }
    let mut ans: Select<&str> = Select::new(lang_eunm[os_lang][3], options);
    ans.help_message = Option::from(lang_eunm[os_lang][4]);
    unsafe {
        let hwnd = FindWindowW(None,  PCWSTR(HSTRING::from(title).as_ptr()));
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

