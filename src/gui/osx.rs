
use systray;


use crate::config::Config;

pub fn open_gui(args: &Vec<String>, config: &Config) {
  if ! args.contains(&"--gui".to_string()) {
    // This attempts to hide the Terminal.app window which macos
    // attaches to us by default.
    
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

