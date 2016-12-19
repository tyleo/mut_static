#![recursion_limit="200"]

#[macro_use]
extern crate error_chain;

mod error;

mod force_some_rw_lock_read_guard;

mod mut_static;

pub use error::*;

pub use force_some_rw_lock_read_guard::*;

pub use mut_static::*;
