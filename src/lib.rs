#![deny(missing_docs)]
#![doc = include_str!("../README.md")]
#[cfg(feature = "server")]
pub use boarding_pass::BoardingPass;
#[cfg(feature = "server")]
pub use gate::{
    Gate,
    MemoryGate,
};
#[cfg(feature = "server")]
pub use passport::{
    Passport,
    PassportType,
};
#[cfg(feature = "server")]
pub use spaceport_setup::SpaceportSetup;
pub use ticket::Ticket;

#[cfg(feature = "server")]
mod boarding_pass;
#[cfg(feature = "server")]
mod gate;
#[cfg(feature = "server")]
mod passport;
#[cfg(feature = "server")]
mod spaceport_setup;
#[cfg(feature = "client")]
mod ticket;
