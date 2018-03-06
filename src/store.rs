use std::path::Path;

use kv;

use txn::Txn;

/// A Store holds content adressable values
pub struct Store(kv::Store);

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
    pub fn read_txn<'a, V: kv::Value<'a>>(&'a self) -> Result<Txn<'a, V>, kv::Error> {
        Ok(Txn(self.0.read_txn()?))
    }

    /// Open a writable transaction
    pub fn write_txn<'a, V: kv::Value<'a>>(&'a mut self) -> Result<Txn<'a, V>, kv::Error> {
        Ok(Txn(self.0.write_txn()?))
    }
}
