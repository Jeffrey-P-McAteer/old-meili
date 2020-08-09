
/**
 * Each OS target gets it's own implementation, which we re-export
 * under the same name.
 * This lets non-gui code not have to worry about
 * the fact that windows requires .ico files, linux likes .png, and 
 * macos has it's own .icns file formats;
 * among other OS-specific requirements for graphics.
 */

use std::sync::Arc;

use crate::config::Config;
use crate::global::Global;

#[allow(dead_code, unused_variables)]
const ICON_PNG: &'static [u8] = include_bytes!("../../res/icon.png");
#[allow(dead_code, unused_variables)]
const ICON_ICO: &'static [u8] = include_bytes!("../../res/icon.ico");
//#[allow(dead_code, unused_variables)]
//const icon_: &'static [u8; N] icon_png = include_bytes!("../res/icon.png");


#[cfg(target_os = "macos")]
#[allow(dead_code, unused_variables)]
mod macos;

#[cfg(target_os = "windows")]
#[allow(dead_code, unused_variables)]
mod win;

#[cfg(target_os = "linux")]
#[allow(dead_code, unused_variables)]
mod linux;

mod cli;

pub fn open_gui(args: Arc<Vec<String>>, config: Arc<Config>, global: Arc<Global>) {
  // TODO spawn a thread to perform bg tasks using global

  #[cfg(target_os = "linux")]
  linux::open_gui(&args, &config, &global);
  
  #[cfg(target_os = "windows")]
  win::open_gui(&args, &config, &global);

  #[cfg(target_os = "macos")]
  macos::open_gui(&args, &config, &global);
}


pub fn open_cli(args: Arc<Vec<String>>, config: Arc<Config>, global: Arc<Global>) {
  cli::open_cli(&args, &config, &global);
}

pub fn start_tcp_cli(args: Arc<Vec<String>>, config: Arc<Config>, global: Arc<Global>) {
  cli::start_tcp_cli(&args, &config, &global);
}

