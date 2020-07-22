
use std::thread;
use std::sync::Arc;
use std::io;
use std::net::{
  UdpSocket,
  SocketAddr,
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
              s.join_multicast_v4(&ip_a, &Ipv4Addr::new(0,0,0,0));
            }
            IpAddr::V6(ip_a) => {
              s.join_multicast_v6(&ip_a, 0);
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
          println!("e={:?}", e);
        },
      }
    }

    std::thread::sleep( std::time::Duration::from_nanos(*&config.poll_delay_ns as u64) );
  }
}
