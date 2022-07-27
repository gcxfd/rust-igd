//! This module implements the same features as the main crate, but using async io.

mod gateway;
mod search;
mod soap;

pub use self::{gateway::Gateway, search::search_gateway};
