
use winres;
use std::path::{Path, PathBuf};

fn main() {
  embed_icon();
}

fn embed_icon() {
  let mut compiling_for_windows = false;

  if let Ok(target_triple) = std::env::var("TARGET") {
    if target_triple.contains("windows") {
      compiling_for_windows = true;
    }
  }

  if compiling_for_windows {

    // Add icon
    let mut res = winres::WindowsResource::new();

    res.set_toolkit_path(".");

    let windres_paths = vec![
      "/usr/bin/x86_64-w64-mingw32-windres"
    ];
    for p in windres_paths {
      if Path::new(p).exists() {
        res.set_windres_path(p);
        break;
      }
    }
    
    let ar_paths = vec![
      "/usr/bin/x86_64-w64-mingw32-ar"
    ];
    for p in ar_paths {
      if Path::new(p).exists() {
        res.set_ar_path(p);
        break;
      }
    }

    let ico_rel_path: PathBuf = ["res", "icon.ico"].iter().collect();

    res.set_icon(&ico_rel_path.to_string_lossy());

    println!("res={:#?}", res);

    //res.compile().unwrap();
    if let Err(e) = res.compile() {
      println!("e={:?}", e);
      panic!();
    }

  }
}


