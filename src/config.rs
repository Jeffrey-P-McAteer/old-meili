
use serde::{
  Serialize, Deserialize,
  Serializer, Deserializer,
  de,
  de::Visitor, de::Unexpected,
};
use toml;
use hostname;

use cidr_utils;

use humantime;

use std::path::{Path};
use std::fs;
use std::net::{SocketAddr};
use std::fmt;

#[derive(Debug)]
pub struct MeiliIpCidr(cidr_utils::cidr::IpCidr);

impl Serialize for MeiliIpCidr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string()[..])
    }
}

struct MeiliIpCidrVisitor;
impl<'de> Visitor<'de> for MeiliIpCidrVisitor {
    type Value = MeiliIpCidr;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an ipv4 or ipv6 CIDR string like 192.168.0.0/24 or fe80:1::1/64")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(ip_cidr) = cidr_utils::cidr::IpCidr::from_str(s) {
            Ok( MeiliIpCidr(ip_cidr) )
        } else {
            Err(de::Error::invalid_value(Unexpected::Str(s), &self))
        }
    }

}

impl<'de> Deserialize<'de> for MeiliIpCidr {
    fn deserialize<D>(deserializer: D) -> Result<MeiliIpCidr, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(MeiliIpCidrVisitor)
    }
}


#[derive(Debug)]
pub struct MeiliHumanDuration(humantime::Duration);

impl Serialize for MeiliHumanDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string()[..])
    }
}

struct MeiliHumanDurationVisitor;
impl<'de> Visitor<'de> for MeiliHumanDurationVisitor {
    type Value = MeiliHumanDuration;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a human duration like '24h' or '1day 2hours'. See https://docs.rs/humantime/2.0.1/humantime/fn.parse_duration.html.")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(ip_cidr) = s.parse::<humantime::Duration>() {
            Ok( MeiliHumanDuration(ip_cidr) )
        } else {
            Err(de::Error::invalid_value(Unexpected::Str(s), &self))
        }
    }

}

impl<'de> Deserialize<'de> for MeiliHumanDuration {
    fn deserialize<D>(deserializer: D) -> Result<MeiliHumanDuration, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(MeiliHumanDurationVisitor)
    }
}







#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
  pub hostname: String,
  
  pub poll_delay_ns: usize,
  
  pub attempt_upnp_port_forward: bool,
  pub upnp_gw_timeout_ms: usize,
  pub upnp_pref_public_port: usize,
  pub upnp_local_port: usize,

  #[serde(default = "default_ip_range_scan_seed")]
  pub ip_range_scan_seed: usize,
  pub ip_ranges_to_scan: Vec<IPRange>,
  
  pub udp_sockets_to_listen_on: Vec<ConfSocket>
}

fn default_ip_range_scan_seed() -> usize {
  12345 // TODO replace w/ hash of hostname
}
fn default_max_ips_per_second() -> usize {
  100
}
fn default_rescan_age() -> MeiliHumanDuration {
  MeiliHumanDuration( "24h".parse::<humantime::Duration>().unwrap().into() )
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IPRange {
  pub name: Option<String>,
  pub cidr: MeiliIpCidr,

  #[serde(default = "default_max_ips_per_second")]
  pub max_ips_per_second: usize,
  
  #[serde(default = "default_rescan_age")]
  pub rescan_age: MeiliHumanDuration,
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
      poll_delay_ns: 4000000,

      attempt_upnp_port_forward: true,
      upnp_gw_timeout_ms: 5000,
      upnp_pref_public_port: 1337,
      upnp_local_port: 1337,

      ip_range_scan_seed: default_ip_range_scan_seed(),
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
