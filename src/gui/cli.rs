
/**
 * The CLI mod is also cross-platform, but it looks like we can
 * depend on shrust to give us cross-platform plain text input.
 */

use shrust::{Shell, ShellIO, ExecError};

use std::io::prelude::*;

use crate::punwrap_r;
use crate::config::Config;
use crate::global::Global;
use crate::net;

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
pub fn create_shell(args: &Vec<String>, config: &Config, global: &Global) -> shrust::Shell<()> {
  // TODO avoid unsafe - ideally by making Shell use some <'a> lifetime bound
  let args: &'static Vec<String> = unsafe {
    std::mem::transmute::<&Vec<String>, &'static Vec<String>>(args)
  };
  let config: &'static Config = unsafe {
    std::mem::transmute::<&Config, &'static Config>(config)
  };
  let global: &'static Global = unsafe {
    std::mem::transmute::<&Global, &'static Global>(global)
  };

  let mut shell = Shell::new(());

  shell.new_command("status", "Get the status of network comms and local settings", 0, move |io, _shell_data, cmd_args| {
      writeln!(io, "cmd_args={:?}", &cmd_args)?;
      writeln!(io, "args={:?}", &args)?;
      writeln!(io, "config={:#?}", &config)?;
      writeln!(io, "global={:#?}", &global)?;
      Ok(())
  });

  shell.new_command("setup-upnp", "Detect the UPNP gateway and ask it to forward ports", 0, move |io, _shell_data, cmd_args| {
      net::attempt_upnp_setup(args, config, global);
      Ok(())
  });

  shell.new_command_noargs("quit", "Exit the meili process", move |_, _shell_data| {
    Err(ExecError::Quit)
  });

  shell
}


