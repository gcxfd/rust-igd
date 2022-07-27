#![deny(missing_docs)]

//! This library allows you to communicate with an IGD enabled device.
//! Use one of the `search_gateway` functions to obtain a `Gateway` object.
//! You can then communicate with the device via this object.

// data structures
// search of gateway
pub use self::{
  common::{parsing::PortMappingEntry, SearchOptions},
  errors::{
    AddAnyPortError, AddPortError, Error, GetExternalIpError, GetGenericPortMappingEntryError,
    RemovePortError, RequestError, Result, SearchError,
  },
  gateway::Gateway,
  search::search_gateway,
};

#[cfg(feature = "aio")]
pub mod aio;
mod common;
mod errors;
mod gateway;
mod search;

use std::fmt;

/// Represents the protocols available for port mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortMappingProtocol {
  /// TCP protocol
  TCP,
  /// UDP protocol
  UDP,
}

impl fmt::Display for PortMappingProtocol {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match *self {
        PortMappingProtocol::TCP => "TCP",
        PortMappingProtocol::UDP => "UDP",
      }
    )
  }
}
