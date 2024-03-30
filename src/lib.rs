#![deny(missing_docs)]
#![doc = include_str!("../README.md")]
#[cfg(feature = "server")]
pub use boarding_pass::BoardingPass;
#[cfg(feature = "server")]
pub use passport::{
    Passport,
    PassportType,
};
pub use ticket::Ticket;

#[cfg(feature = "server")]
mod boarding_pass;
#[cfg(feature = "server")]
pub mod gate;
#[cfg(feature = "server")]
mod passport;
#[cfg(feature = "client")]
mod ticket;
