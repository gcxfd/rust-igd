use std::{env, net::SocketAddrV4};

extern crate igd_async_std;

fn main() {
  match igd_async_std::search_gateway(Default::default()) {
    Err(ref err) => match *err {
      igd_async_std::SearchError::IoError(ref ioe) => println!("IoError: {}", ioe),
      _ => println!("{:?}", err),
    },
    Ok(gateway) => {
      let args: Vec<_> = env::args().collect();
      if args.len() != 4 {
        println!("Usage: add_remove <local_ip> <local_port> <remote_port>");
        return;
      }
      let local_ip = args[1].parse().expect("Invalid IP address");
      let local_port = args[2].parse().expect("Invalid local port");
      let remote_port = args[3].parse().expect("Invalid remote port");

      let local_addr = SocketAddrV4::new(local_ip, local_port);

      match gateway.add_port(
        igd_async_std::PortMappingProtocol::TCP,
        remote_port,
        local_addr,
        60,
        "crust",
      ) {
        Err(ref err) => println!("{:?}", err),
        Ok(()) => {
          println!("AddPortMapping successful.");
          match gateway.remove_port(igd_async_std::PortMappingProtocol::TCP, remote_port) {
            Err(ref err) => println!("Error removing: {:?}", err),
            Ok(_) => println!("DeletePortMapping successful."),
          }
        }
      }
    }
  }
}
