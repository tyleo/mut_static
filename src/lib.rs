#![recursion_limit="200"]

#[macro_use]
extern crate error_chain;

pub mod error;

pub mod force_some_rw_lock_read_guard;

pub mod safe_static;

pub use error::*;

pub use force_some_rw_lock_read_guard::ForceSomeRwLockReadGuard;

pub use safe_static::SafeStatic;
