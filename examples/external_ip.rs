extern crate igd_async_std;

fn main() {
  match igd_async_std::search_gateway(Default::default()) {
    Err(ref err) => println!("Error: {}", err),
    Ok(gateway) => match gateway.get_external_ip() {
      Err(ref err) => {
        println!("There was an error! {}", err);
      }
      Ok(ext_addr) => {
        println!(
          "Local gateway: {}, External ip address: {}",
          gateway, ext_addr
        );
      }
    },
  }
}
