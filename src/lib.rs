#![deny(missing_docs)]

//! `castor` provides content addressable storage on top of [LMDB](https://github.com/LMDB/lmdb)
//! using [kv](https://github.com/zshipko/rust-kv)
//!
//! ```rust
//! use castor::{Config, Store, Blake2s, Txn, Token};
//!
//! fn main() -> Result<(), kv::Error> {
//!     let cfg: Config<Blake2s> = Store::config("/tmp/castor-example");
//!     let mut store: Store<Blake2s> = Store::new(cfg)?;
//!     let bucket: kv::Bucket<Token<Blake2s>, &str> = store.bucket(None)?;
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

#[cfg(feature = "sha2_256")]
/// Sha2_256 Tokens
pub type Sha2_256 = sha2::Sha256;

#[cfg(feature = "sha2_256")]
impl Hash for Sha2_256 {
    fn hex(self) -> String {
        use digest::Digest;
        format!("{:x}", self.result())
    }
}

#[cfg(feature = "sha2_512")]
/// Sha2_512 Tokens
pub type Sha2_512 = sha2::Sha512;

#[cfg(feature = "sha2_512")]
impl Hash for Sha2_512 {
    fn hex(self) -> String {
        use digest::Digest;
        format!("{:x}", self.result())
    }
}

#[cfg(feature = "sha3_256")]
/// Sha3_256 Tokens
pub type Sha3_256 = sha3::Sha3_256;

#[cfg(feature = "sha3_256")]
impl Hash for Sha3_256 {
    fn hex(self) -> String {
        use digest::Digest;
        format!("{:x}", self.result())
    }
}

#[cfg(feature = "sha3_512")]
/// Sha3_512 Tokens
pub type Sha3_512 = sha3::Sha3_512;

#[cfg(feature = "sha3_512")]
impl Hash for Sha3_512 {
    fn hex(self) -> String {
        use digest::Digest;
        format!("{:x}", self.result())
    }
}

#[cfg(feature = "blake2b")]
/// Blake2 Tokens
pub type Blake2b = blake2::Blake2b;

#[cfg(feature = "blake2b")]
impl Hash for Blake2b {
    fn hex(self) -> String {
        use digest::Digest;
        format!("{:x}", self.result())
    }
}

#[cfg(feature = "blake2s")]
/// Blake2 Tokens
pub type Blake2s = blake2::Blake2s;

#[cfg(feature = "blake2s")]
impl Hash for Blake2s {
    fn hex(self) -> String {
        use digest::Digest;
        format!("{:x}", self.result())
    }
}
