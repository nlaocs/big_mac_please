use mac_address::get_mac_address;

fn main() {
    match get_mac_address() {
        Ok(Some(mac)) => {
            println!("{}", mac);
        }
        Ok(None) => println!("(._. ))"),
        Err(e) => println!("{:?}", e),
    }
}