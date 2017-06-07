#[deny(warnings)]

extern crate hashstore;

#[cfg(test)]
#[macro_use]
mod testutils;

mod dagaostore;

// Library Public API:
pub use dagaostore::DagaoStore;
