#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod auth_type;

pub use ticket::Ticket;

#[cfg(feature = "server")]
pub mod boarding_pass;
#[cfg(feature = "server")]
pub mod ciphering;
#[cfg(feature = "server")]
pub mod gate;
#[cfg(feature = "server")]
pub mod passport;
#[cfg(feature = "server")]
pub mod passport_register;
#[cfg(feature = "server")]
pub mod storage;
#[cfg(feature = "client")]
mod ticket;
