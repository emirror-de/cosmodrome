#![deny(missing_docs)]
#![feature(doc_cfg)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "server")]
#[doc(cfg(feature = "server"))]
pub(crate) mod server;
#[cfg(feature = "server")]
#[doc(cfg(feature = "server"))]
pub use server::*;

mod ticket;
pub use ticket::Ticket;
