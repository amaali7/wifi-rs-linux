mod connectivity;
mod hotspot;
mod platforms;

use crate::connectivity::{Connectivity, WifiConnectionError};
use crate::platforms::{Config, Linux};

fn main() -> Result<(), WifiConnectionError> {
    let config = Some(Config {
        interface: Some("wlo1"),
    });

    let mut wifi = Linux::new(config);

    match wifi.connect("CSIS_MH", "") {
        Ok(result) => println!(
            "{}",
            if result == true {
                "Connection Successfull."
            } else {
                "Invalid password."
            }
        ),
        Err(err) => println!("The following error occurred: {:?}", err),
    }

    Ok(())
}
