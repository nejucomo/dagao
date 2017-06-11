#![deny(warnings)]

extern crate hashstore;

#[cfg(test)]
#[macro_use]
mod testutils;

mod store;
mod ioutil;
mod reference;

// Library Public API:
pub mod datanode;
pub mod linknode;
pub use store::Store;
pub use reference::{Reference, RefType};
