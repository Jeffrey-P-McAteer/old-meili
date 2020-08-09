
use igd;

#[cfg(not(windows))]
use get_if_addrs;

use std::thread;
use std::sync::Arc;
use std::io;
use std::time::Duration;
use std::net::{
  UdpSocket,
  SocketAddrV4,
  IpAddr, Ipv4Addr,
};

use crate::punwrap_r;
use crate::config::Config;
use crate::global::Global;

//const NET_BUFF_SIZE: usize = 65535;
const NET_BUFF_SIZE: usize = 32535;

pub fn spawn_listeners(args: Arc<Vec<String>>, config: Arc<Config>, global: Arc<Global>) {
  thread::spawn(move || {
    run_listeners(args, config, global);
  });
}

pub fn run_listeners(args: Arc<Vec<String>>, config: Arc<Config>, global: Arc<Global>) {
  // First bind to everything config tells us to bind to...

  let mut udp_sockets: Vec<UdpSocket> = Vec::new();

  for conf_socket in &config.udp_sockets_to_listen_on {
    let name = conf_socket.name.clone().unwrap_or("".to_string());
    println!("Listening to '{}' ({:?})", name, conf_socket.socket);
    match UdpSocket::bind(&conf_socket.socket) {
      Ok(s) => {
        punwrap_r!(s.set_nonblocking(true), nothing);

        if conf_socket.socket.ip().is_multicast() {
          match conf_socket.socket.ip() {
            IpAddr::V4(ip_a) => {
              punwrap_r!(s.join_multicast_v4(&ip_a, &Ipv4Addr::new(0,0,0,0)), nothing);
            }
            IpAddr::V6(ip_a) => {
              punwrap_r!(s.join_multicast_v6(&ip_a, 0), nothing);
            }
          }
        }

        udp_sockets.push(s);
      }
      Err(e) => {
        println!("e={:?}", e);
      }
    }
  }

  // We spawn this to a thread b/c attempt_upnp_setup blocks
  let upnp_args = args.clone();
  let upnp_config = config.clone();
  let upnp_global = global.clone();
  thread::spawn(move || {
    if let Err(e) = attempt_upnp_setup(&upnp_args, &upnp_config, &upnp_global) {
      println!("upnp e={:?}", e);
    }
  });

  let mut net_buf = [0; NET_BUFF_SIZE];
  loop {
    // Poll sockets for incoming packets...
    for s in &udp_sockets {
      match s.recv_from(&mut net_buf) {
        Ok((num_bytes, client_sockaddr)) => {
          // Handle the packet
          println!("Got {} bytes from {:?}", num_bytes, client_sockaddr);

        }
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
          continue;
        }
        Err(e) => {
          println!("socket e={:?}", e);
        },
      }
    }

    std::thread::sleep( Duration::from_nanos(*&config.poll_delay_ns as u64) );
  }
}

pub fn attempt_upnp_setup(_args: &Vec<String>, config: &Config, _global: &Global) -> Result<(), igd::Error> {
  if !config.attempt_upnp_port_forward {
    return Ok(());
  }

  let igd_opts = igd::SearchOptions {
    timeout: Some(Duration::from_millis(config.upnp_gw_timeout_ms as u64)),
    ..Default::default()
  };
  let gw = igd::search_gateway(igd_opts)?;
  //println!("gw={:?}", &gw);

  let mut lan_ip_a: Ipv4Addr = Ipv4Addr::BROADCAST;
  // We pick the first 
  #[cfg(not(windows))]
  {
    match get_if_addrs::get_if_addrs() {
      Ok(if_addrs) => {
        for addr in if_addrs {
          if addr.is_loopback() {
            continue;
          }
          // igd only takes SocketAddrV4 so we cannot use ipv6 for this :(
          if let IpAddr::V4(addr) = addr.ip() {
            lan_ip_a = addr;
            break;
          }
        }
      }
      Err(e) => {
        println!("e={:?}", e);
        // Not exactly the most appropriate error but we'll use it
        return Err(igd::Error::GetExternalIpError( igd::GetExternalIpError::ActionNotAuthorized ));
      }
    }
  }
  #[cfg(windows)]
  {
    // TODO windows IP addr lookup
  }
  println!("lan_ip_a={:?}", &lan_ip_a);

  let local_addr = SocketAddrV4::new(lan_ip_a.clone(), config.upnp_local_port as u16);
  let mut external_port = config.upnp_pref_public_port as u16;

  // First search existing ports + exit if one has our lan_ip_a and config.upnp_local_port and config.upnp_pref_public_port
  let mut i = 0;
  loop {
    match gw.get_generic_port_mapping_entry(i) {
      Ok(entry) => {
        println!(
          "i={} external_port={} internal_client={} port_mapping_description={} lease_duration={}",
          i, entry.external_port, entry.internal_client, entry.port_mapping_description, entry.lease_duration
        );
        let is_match = entry.port_mapping_description.contains("meili") &&
                       entry.internal_client == format!("{}", lan_ip_a) &&
                       entry.internal_port == config.upnp_local_port as u16;
        if is_match {
          // return b/c we already have an entry; TODO Write external_port to global?
          println!("Already have requested UPNP port mapping on public port :{}", entry.external_port);
          return Ok(());
        }
      }
      Err(igd::GetGenericPortMappingEntryError::RequestError(re))  => {
        println!("re={:?}", re);
        continue;
      }
      Err(_e)  => {
        break;
      }
    }
    if i > 256 {
      break; // Sanity check
    }
    i += 1;
  }

  let lease_duration_s = 300;
  if let Err(e) = gw.add_port(igd::PortMappingProtocol::UDP, external_port, local_addr, lease_duration_s, "meili port mapping") {
    println!("upnp e={:?}", e);
    // Attempt w/ random public port
    external_port = gw.add_any_port(igd::PortMappingProtocol::UDP, local_addr, lease_duration_s, "meili port mapping")?;
  }
  
  // Write external_port to global? Not sure what the most useful action is here.
  println!("Added requested UPNP port mapping on public port :{}", external_port);

  Ok(())
}

