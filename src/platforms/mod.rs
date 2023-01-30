pub mod linux;
pub use self::linux::{Connection, Linux};

use std::{fmt, io};

/// Configuration for a wifi network.
#[derive(Debug, Clone)]
pub struct Config<'a> {
    /// The interface the wifi module is situated.
    pub interface: Option<&'a str>,
}

#[derive(Debug)]
pub enum WifiError {
    // The specified wifi  is currently disabled. Try switching it on.
    WifiDisabled,
   /// IO Error occurred.
    IoError(io::Error),
}

/// Wifi interface for an operating system.
/// This provides basic functionalities for wifi interface.
pub trait WifiInterface: fmt::Debug {
    /// Check if the wifi interface on host machine is enabled.
    fn is_wifi_enabled() -> Result<bool, WifiError> {
        unimplemented!();
    }

    /// Turn on the wifi interface of host machine.
    fn turn_on() -> Result<(), WifiError> {
        unimplemented!();
    }

    /// Turn off the wifi interface of host machine.
    fn turn_off() -> Result<(), WifiError> {
        unimplemented!();
    }
}
