
use app_dirs;

use std::path::{Path,PathBuf};
use std::env;
use std::fs;
use std::sync::Arc;

mod gui;
mod config;
mod global;
mod net;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_INFO: app_dirs::AppInfo = app_dirs::AppInfo{
  name: "meili",
  author: "meili"
};

enum Action {
  PrintAbout,
  PrintUsage,
  OpenGui,
  RunCLI,
  RunNetCLI,
}

fn main() {
    let mut action = Action::OpenGui;
    let mut app_dir = get_app_dir();

    // arguments modify the variables above, which are then passed into the rest of the program.
    let args: Vec<String> = env::args().collect();
    for i in 0..args.len() {
      match args[i].as_str() {
        "--about" => {
          action = Action::PrintAbout;
        }
        "--help" => {
          action = Action::PrintUsage;
        }
        "--app-dir" => {
          app_dir = PathBuf::from(args.get(i+1).expect("User did not supply argument to --app-dir"));
        }
        "--gui" => {
          action = Action::OpenGui;
        }
        "--cli" => {
          action = Action::RunCLI;
        }
        "--net-cli" => {
          action = Action::RunNetCLI;
        }
        _unk => {
          // likely an arg to another arg. meili just ignores garbage arguments.
        }
      }
      
    }

    // Read in config file, creating the default one if nothing exists.
    if !app_dir.as_path().exists() {
      fs::create_dir_all( app_dir.as_path() ).expect("Could not create app_dir");
    }
    let config_file = {
      let mut c = app_dir.clone();
      c.push("meili.toml");
      c
    };
    if !config_file.as_path().exists() {
      fs::write(config_file.as_path(), include_str!("meili.toml")).expect("Could not write default meili.toml");
    }
    let config = config::read_config( &config_file.as_path() );
    let global = global::Global::default();

    let args = Arc::new(args);
    let config = Arc::new(config);
    let global = Arc::new(global);

    // Now we execute things. This mostly consists of forwarding the input data to functions.
    match action {
      Action::PrintAbout => { print_about(&app_dir, &config); }
      Action::PrintUsage => { print_usage(); }
      Action::OpenGui => {
        net::spawn_listeners(args.clone(), config.clone(), global.clone());
        gui::open_gui(args.clone(), config.clone(), global.clone());
      }
      Action::RunCLI => {
        net::spawn_listeners(args.clone(), config.clone(), global.clone());
        gui::open_cli(args.clone(), config.clone(), global.clone());
      }
      Action::RunNetCLI => {
        net::spawn_listeners(args.clone(), config.clone(), global.clone());
        gui::start_tcp_cli(args.clone(), config.clone(), global.clone());
      }
    }

}

fn get_app_dir() -> PathBuf {
  app_dirs::get_app_root(app_dirs::AppDataType::UserConfig, &APP_INFO)
    .unwrap_or(PathBuf::new())
}

fn print_about(app_dir: &PathBuf, config: &config::Config) {
  println!(r#"Meili {VERSION}
app_dir={app_dir}
config={config:#?}
"#,
  VERSION=VERSION,
  app_dir=app_dir.to_string_lossy(),
  config=config,
);
}

fn print_usage() {
  println!(include_str!("usage.txt"));
}
