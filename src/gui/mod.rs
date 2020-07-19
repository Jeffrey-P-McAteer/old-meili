
/**
 * Each OS target gets it's own implementation, which we re-export
 * under the same name.
 * This lets non-gui code not have to worry about
 * the fact that windows requires .ico files, linux likes .png, and 
 * macos has it's own .icns file formats;
 * among other OS-specific requirements for graphics.
 */

#[cfg(target_os = "macos")]
mod osx;

#[cfg(target_os = "macos")]
pub use osx::open_gui as open_gui;



#[cfg(target_os = "windows")]
mod win;

#[cfg(target_os = "windows")]
pub use win::open_gui as open_gui;



#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::open_gui as open_gui;

