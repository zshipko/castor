#![deny(missing_docs)]

//! `castor` provides content addressable storage on top of LMDB

extern crate kv;
extern crate sha2;

mod token;
mod store;
mod txn;

pub use token::Token;
pub use store::Store;
pub use txn::Txn;
