
use serde::{Serialize, Deserialize};
use toml;
use hostname;

use std::path::{Path};
use std::fs;
use std::net::{IpAddr,SocketAddr};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
  pub hostname: String,
  pub poll_delay_ns: usize,
  pub ip_ranges_to_scan: Vec<IPRange>,
  pub udp_sockets_to_listen_on: Vec<ConfSocket>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IPRange {
  pub name: Option<String>,
  pub begin: IpAddr,
  pub end: IpAddr,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfSocket {
  pub name: Option<String>,
  pub socket: SocketAddr,
}

impl Default for Config {
  fn default() -> Config {
    Config {
      hostname: String::new(),
      poll_delay_ns: 4000,
      ip_ranges_to_scan: Vec::new(),
      udp_sockets_to_listen_on: Vec::new(),
    }
  }
}

pub fn read_config(conf_file: &Path) -> Config {
  let mut c = read_config_from_file(conf_file);
  if c.hostname.len() < 4 {
    c.hostname = hostname::get().unwrap_or( std::ffi::OsString::from("localhost") ).to_string_lossy().to_string();
  }
  return c;
}

pub fn read_config_from_file(conf_file: &Path) -> Config {
  match fs::read_to_string(conf_file) {
    Ok(conf_contents) => {
      match toml::from_str(&conf_contents) {
        Ok(config_data) => config_data,
        Err(e) => {
          println!("Error reading config: {}", e);
          Config::default()
        }
      }
    }
    Err(e) => {
      println!("Error opening config: {}", e);
      Config::default()
    }
  }
}
