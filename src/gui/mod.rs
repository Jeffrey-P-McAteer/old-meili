
/**
 * Each OS target gets it's own implementation, which we re-export
 * under the same name.
 * This lets non-gui code not have to worry about
 * the fact that windows requires .ico files, linux likes .png, and 
 * macos has it's own .icns file formats;
 * among other OS-specific requirements for graphics.
 */

use crate::config::Config;
use crate::global::Global;

const ICON_PNG: &'static [u8] = include_bytes!("../../res/icon.png");
const ICON_ICO: &'static [u8] = include_bytes!("../../res/icon.ico");
//const icon_: &'static [u8; N] icon_png = include_bytes!("../res/icon.png");


#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod win;

#[cfg(target_os = "linux")]
mod linux;

pub fn open_gui(args: &Vec<String>, config: &Config, global: &Global) {
  // TODO spawn a thread to perform bg tasks using global

  #[cfg(target_os = "linux")]
  linux::open_gui(args, config, global);
  
  #[cfg(target_os = "windows")]
  win::open_gui(args, config, global);

  #[cfg(target_os = "macos")]
  macos::open_gui(args, config, global);
}
