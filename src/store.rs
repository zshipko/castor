use std::path::Path;

use kv;

use txn::Txn;
use token::Token;

/// A Store holds content adressable values
pub struct Store(pub kv::Store);

impl Store {
    /// Create a new configuration builder
    pub fn config<P: AsRef<Path>>(path: P) -> kv::Config {
        kv::Config::default(path)
    }

    /// Create a new store from the given configuration
    pub fn new(cfg: kv::Config) -> Result<Store, kv::Error> {
        Ok(Store(kv::Store::new(cfg)?))
    }

    /// Open a readonly transaction
    pub fn read_txn<'a>(&'a self) -> Result<Txn<'a>, kv::Error> {
        Ok(Txn(self.0.read_txn()?))
    }

    /// Open a writable transaction
    pub fn write_txn<'a>(&'a mut self) -> Result<Txn<'a>, kv::Error> {
        Ok(Txn(self.0.write_txn()?))
    }

    /// Get handle to kv::Bucket
    pub fn bucket<'a, V: kv::Value<'a>>(&self, name: Option<&str>) -> Result<kv::Bucket<'a, Token, V>, kv::Error> {
        self.0.bucket(name)
    }
}
