pub use self::ui_impl::*;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

// Creating these simplifies the wait code for the craft module
pub fn wait_ms(ms: u64) {
    sleep(Duration::from_millis(ms));
}

pub fn wait_secs(s: u64) {
    sleep(Duration::from_secs(s));
}

#[cfg(windows)]
pub(self) mod ui_impl {
    use std::ffi::CStr;
    use std::ptr::null_mut;
    use std::sync::Once;
    use std::thread::sleep;
    use std::time::Duration;
    use winapi::shared::basetsd::LONG_PTR;
    use winapi::shared::minwindef::{BOOL, UINT};
    pub use winapi::shared::windef::{HWND, HWND__};
    pub use winapi::um::winuser::*;
    pub use winapi::um::winuser::{EnumWindows, GetWindowTextA, PostMessageA};

    // TODO: Configurable keybinds
    const KEY_UP: i32 = VK_NUMPAD8;
    const KEY_DOWN: i32 = VK_NUMPAD2;
    const KEY_LEFT: i32 = VK_NUMPAD4;
    const KEY_RIGHT: i32 = VK_NUMPAD6;
    const KEY_CONFIRM: i32 = VK_NUMPAD0;
    const KEY_FORWARD: i32 = VK_NUMPAD9;
    const KEY_BACKWARD: i32 = VK_NUMPAD7;
    const KEY_CANCEL: i32 = VK_DECIMAL;
    const KEY_ENTER: i32 = VK_RETURN;

    // Common public methods the ui_impl modules export
    pub fn cursor_down() {
        send_key(KEY_DOWN);
    }
    pub fn _cursor_up() {
        send_key(KEY_UP);
    }
    pub fn _cursor_left() {
        send_key(KEY_LEFT);
    }
    pub fn _cursor_right() {
        send_key(KEY_RIGHT);
    }
    pub fn move_backward() {
        send_key(KEY_BACKWARD)
    }
    pub fn _move_forward() {
        send_key(KEY_FORWARD);
    }
    pub fn confirm() {
        send_key(KEY_CONFIRM);
    }
    pub fn cancel() {
        send_key(KEY_CANCEL);
    }
    pub fn enter() {
        send_key(KEY_ENTER);
    }
    pub fn escape() {
        send_key(VK_ESCAPE);
    }

    pub fn open_craft_window() {
        send_key('N' as i32);
    }

    pub fn send_key(c: i32) {
        send_msg(WM_KEYDOWN, c);
        send_msg(WM_KEYUP, c);
        sleep(Duration::from_millis(150));
    }

    pub fn send_char(c: char) {
        send_msg(WM_CHAR, c as i32);
        sleep(Duration::from_millis(20));
    }

    // This callback is called for every window the user32 EnumWindows call finds
    // while walking the window list. Use it to find the XIV window by title.
    //
    // To be more foolproof checking process name might be better.
    unsafe extern "system" fn enum_callback(win_hwnd: HWND, arg: LONG_PTR) -> BOOL {
        let mut title: Vec<i8> = vec![0; 256];
        let xiv_hwnd = arg as *mut HWND;

        if GetWindowTextA(win_hwnd, title.as_mut_ptr(), title.len() as i32) > 0 {
            let title = CStr::from_ptr(title.as_ptr()).to_str().unwrap();
            if title.contains("FINAL FANTASY XIV") {
                //println!("Found XIV window: {}", win_hwnd as u64);
                *xiv_hwnd = win_hwnd;
                return 0;
            }
        }
        1
    }

    // Return the handle of the FFXIV window. The first time this is called we make WinAPI
    // calls to find the window and cache it.

    unsafe fn get_window(hwnd: &mut HWND) {
        EnumWindows(Some(enum_callback), hwnd as *mut HWND as LONG_PTR);
    }

    // Send a character/key to the XIV window
    fn send_msg(msg: u32, key: i32) {
        unsafe {
            let mut window: HWND = null_mut();
            get_window(&mut window);
            PostMessageA(window, msg as UINT, key as usize, 0);
        }
    }
}

#[cfg(not(windows))]
pub(self) mod ui_impl {
    // Common public methods the ui_impl modules export
    pub fn cursor_down() {
        print!("<D> ");
    }
    pub fn _cursor_up() {
        print!("<U> ");
    }
    pub fn _cursor_left() {
        print!("<L> ");
    }
    pub fn _cursor_right() {
        print!("<R> ");
    }
    pub fn move_backward() {
        print!("<- ");
    }
    pub fn _move_forward() {
        print!("-> ");
    }
    pub fn enter() {
        println!("<ENTER> ");
    }
    pub fn confirm() {
        println!("<OK> ");
    }
    pub fn cancel() {
        println!("<CANCEL> ");
    }
    pub fn _escape() {
        println!("<ESC> ");
    }
    pub fn send_char(c: char) {
        print!("{}", c);
    }
    pub fn open_craft_window() {}
}
