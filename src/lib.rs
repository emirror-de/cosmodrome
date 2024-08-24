#![deny(missing_docs)]
#![feature(doc_cfg)]
#![doc(
    html_logo_url = "https://github.com/emirror-de/cosmodrome/blob/unstable/resources/icon.png?raw=true",
    html_favicon_url = "https://github.com/emirror-de/cosmodrome/blob/unstable/resources/icon.png?raw=true"
)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "server")]
#[doc(cfg(feature = "server"))]
pub(crate) mod server;
#[cfg(feature = "server")]
#[doc(cfg(feature = "server"))]
pub use server::*;

mod ticket;
pub use ticket::Ticket;
