
use systray;
use winapi;

use crate::config::Config;


pub fn open_gui(args: &Vec<String>, config: &Config) {
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

  let mut app = systray::Application::new().expect("Cannot create graphics");
  
  // TODO embed + extract + assign icon file
  //app.set_icon_from_file("/usr/share/gxkb/flags/ua.png").unwrap();

  let hostname_s = format!("h: {}", config.hostname);
  app.add_menu_item(&hostname_s, |_| {
      
      // TODO real menu items
      println!("Printing a thing!");

      Ok::<_, systray::Error>(())
  }).unwrap();

  app.add_menu_item("quit", |_| -> Result<(), systray::Error> {
      std::process::exit(0)
  }).unwrap();

  app.wait_for_message().unwrap();
}
