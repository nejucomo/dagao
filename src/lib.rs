#![deny(warnings)]

extern crate hashstore;

#[cfg(test)]
#[macro_use]
mod testutils;

mod dagaostore;
mod ioutil;
mod reference;

// Library Public API:
pub mod datanode;
pub mod linknode;
pub use dagaostore::DagaoStore;
pub use reference::{Reference, RefType};
