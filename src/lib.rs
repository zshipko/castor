#![deny(missing_docs)]

//! `castor` provides content addressable storage on top of [LMDB](https://github.com/LMDB/lmdb)
//! using [kv](https://github.com/zshipko/rust-kv)
//!
//! ```rust
//! use castor::{Config, Store, Sha256, Txn, Token};
//!
//! fn main() -> Result<(), kv::Error> {
//!     let cfg: Config<Sha256> = Store::config("/tmp/castor-example");
//!     let mut store: Store<Sha256> = Store::new(cfg)?;
//!     let bucket: kv::Bucket<Token<Sha256>, &str> = store.bucket(None)?;
//!     let token = store.with_write_txn(|mut txn| {
//!         txn.put(&bucket, "testing")
//!     })?;
//!     let x = store.with_read_txn(move |txn| {
//!         let value = txn.fetch(&bucket, token)?;
//!         Ok(String::from(value))
//!     })?;
//!     assert_eq!(x, "testing");
//!     Ok(())
//! }
//! ```

mod store;
mod token;
mod txn;

pub use crate::store::{Config, Store};
pub use crate::token::{Hash, Token};
pub use crate::txn::Txn;

#[cfg(feature = "sha256")]
/// Sha256 Tokens
pub type Sha256 = sha2::Sha256;

#[cfg(feature = "sha256")]
impl Hash for Sha256 {
    fn hex(self) -> String {
        use digest::Digest;
        format!("{:x}", self.result())
    }
}
