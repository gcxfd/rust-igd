extern crate igd_async_std;

fn main() {
  match igd_async_std::search_gateway(Default::default()) {
    Err(ref err) => println!("Error: {}", err),
    Ok(gateway) => match gateway.remove_port(igd_async_std::PortMappingProtocol::TCP, 80) {
      Err(ref err) => {
        println!("There was an error! {}", err);
      }
      Ok(()) => {
        println!("It worked");
      }
    },
  }
}
