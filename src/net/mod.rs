
use igd;

use std::thread;
use std::sync::Arc;
use std::io;
use std::time::Duration;
use std::net::{
  UdpSocket,
  SocketAddr, SocketAddrV4,
  IpAddr, Ipv4Addr,
};

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
      Ok(mut s) => {
        s.set_nonblocking(true);

        if conf_socket.socket.ip().is_multicast() {
          match conf_socket.socket.ip() {
            IpAddr::V4(ip_a) => {
              if let Err(e) = s.join_multicast_v4(&ip_a, &Ipv4Addr::new(0,0,0,0)) {
                println!("e={:?}", e);
              }
            }
            IpAddr::V6(ip_a) => {
              if let Err(e) = s.join_multicast_v6(&ip_a, 0) {
                println!("e={:?}", e);
              }
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

fn attempt_upnp_setup(args: &Vec<String>, config: &Config, global: &Global) -> Result<(), igd::Error> {
  if !config.attempt_upnp_port_forward {
    return Ok(());
  }

  let igd_opts = igd::SearchOptions {
    timeout: Some(Duration::from_millis(config.upnp_gw_timeout_ms as u64)),
    ..Default::default()
  };
  let gw = igd::search_gateway(igd_opts)?;

  let local_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), config.upnp_local_port as u16);
  gw.add_port(igd::PortMappingProtocol::UDP, 80, local_addr, 60, "meili port mapping")?;


  Ok(())
}

