
use winapi;

use std;
use std::cell::RefCell;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use winapi::{
    ctypes::{c_ulong, c_ushort},
    shared::{
        basetsd::ULONG_PTR,
        guiddef::GUID,
        minwindef::{DWORD, HINSTANCE, LPARAM, LRESULT, PBYTE, TRUE, UINT, WPARAM},
        ntdef::LPCWSTR,
        windef::{HBITMAP, HBRUSH, HICON, HMENU, HWND, POINT},
    },
    um::{
        errhandlingapi, libloaderapi,
        shellapi::{
            self, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE, NIM_MODIFY, NOTIFYICONDATAW,
        },
        winuser::{
            self, CW_USEDEFAULT, IMAGE_ICON, LR_DEFAULTCOLOR, LR_LOADFROMFILE, MENUINFO,
            MENUITEMINFOW, MFT_SEPARATOR, MFT_STRING, MIIM_FTYPE, MIIM_ID, MIIM_STATE, MIIM_STRING,
            MIM_APPLYTOSUBMENUS, MIM_STYLE, MNS_NOTIFYBYPOS, WM_DESTROY, WM_USER, WNDCLASSW,
            WS_OVERLAPPEDWINDOW,
        },
    },
};

// Got this idea from glutin. Yay open source! Boo stupid winproc! Even more boo
// doing SetLongPtr tho.
//thread_local!(static WININFO_STASH: RefCell<Option<WindowsLoopData>> = RefCell::new(None));

use crate::config::Config;
use crate::global::Global;

pub fn open_gui(args: &Vec<String>, config: &Config, global: &Global) {
  // When no arguments are presented
  // we instruct the OS to close our console. If the user runs the meili
  // from a console it reads/writes to that console, and if they run it with "--gui"
  // the console will remain open which is nice for debugging.
  if ! args.contains(&"--gui".to_string()) {
    // This delay exists to show the console opening, then closing.
    // Environment variable must be assigned at build time to take effect.
    if let Some(val) = option_env!("MEILI_BUILD_ADD_DELAYS") {
      if val.contains("1") || val.contains("t") {
        println!("Sleeping for 800ms to show windows console...");
        std::thread::sleep( std::time::Duration::from_millis(800) );
      }
    }

    println!("Closing windows console...");
    unsafe {
      winapi::um::wincon::FreeConsole();
    }
  }



}
