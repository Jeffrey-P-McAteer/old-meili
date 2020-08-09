
/**
 * The CLI mod is also cross-platform, but it looks like we can
 * depend on shrust to give us cross-platform plain text input.
 */

use shrust::{Shell, ShellIO, ExecError};

use std::io::prelude::*;

use crate::punwrap_r;
use crate::config::Config;
use crate::global::Global;

pub fn open_cli(args: &Vec<String>, config: &Config, global: &Global) {
  let mut shell = create_shell(args, config, global);
  shell.run_loop(&mut ShellIO::default());
}

pub fn start_tcp_cli(args: &Vec<String>, config: &Config, global: &Global) {
  use std::net::{TcpListener};
  use std::thread;

  let serv = TcpListener::bind("[::]:1339").expect("Cannot open socket");
  println!("Listening on tcp://[::]:1339");
  
  loop {
    match serv.accept() {
      Ok((mut sock, addr)) => {
        let is_localhost = addr.ip().is_loopback();
        if is_localhost {
          let mut shell = create_shell(args, config, global);
          let mut io = ShellIO::new_io(sock);
          thread::spawn(move || shell.run_loop(&mut io));
        }
        else {
          println!("non-local conn addr={:?}", &addr);
          punwrap_r!(sock.write("No non-local connections allowed".as_bytes()), nothing);
          punwrap_r!(sock.flush(), nothing);
        }
      }
      Err(e) => {
        println!("couldn't .accept() client: {:?}", e);
      }
    }
  }
}

/**
 * This creates a shell which may be presented over any IO device.
 */
pub fn create_shell(_args: &Vec<String>, _config: &Config, _global: &Global) -> shrust::Shell<()> {
  let mut shell = Shell::new(());

  shell.new_command("status", "Get the status of network comms and local settings", 0, |io, _shell_data, cmd_args| {
      writeln!(io, "cmd_args={:?}", &cmd_args)?;
      Ok(())
  });

  shell.new_command_noargs("quit", "Exit the meili process", move |_, _shell_data| {
    Err(ExecError::Quit)
  });

  shell
}


