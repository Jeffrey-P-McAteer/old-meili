
use sysbar;

use crate::config::Config;

pub fn open_gui(args: &Vec<String>, config: &Config) {
  if ! args.contains(&"--gui".to_string()) {
    // This attempts to hide the Terminal.app window which macos
    // attaches to us by default.
    
  }

  let mut bar = sysbar::Sysbar::new("meili");
  
  let hostname_s = format!("h: {}", config.hostname);
  bar.add_item(
    &hostname_s,
    Box::new(move || {
        
        // TODO real menu items
        println!("Printing a thing!");

    }),
  );

  bar.add_quit_item("Quit");

  bar.display();

}
