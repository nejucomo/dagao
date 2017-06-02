#[deny(warnings)]

extern crate hashstore;

mod dagaostore;

// Library Public API:
pub use hashstore::{Hash, Hasher};
pub use dagaostore::{DagaoInserter, DagaoStore};
