use std::marker::PhantomData;
use std::path::Path;

use crate::token::{Hash, Token};
use crate::txn::Txn;

/// A Store holds content adressable values
pub struct Store<T: Hash>(kv::Store, PhantomData<T>);

impl<T: Hash> From<kv::Store> for Store<T> {
    fn from(store: kv::Store) -> Store<T> {
        Store(store, PhantomData)
    }
}

impl<T: Hash> From<Store<T>> for kv::Store {
    fn from(store: Store<T>) -> kv::Store {
        store.0
    }
}

/// Castor configuration, parameterized by the digest type
pub struct Config<T: Hash>(kv::Config, PhantomData<T>);

impl<T: Hash> From<kv::Config> for Config<T> {
    fn from(cfg: kv::Config) -> Config<T> {
        Config(cfg, PhantomData)
    }
}

impl<T: Hash> From<Config<T>> for kv::Config {
    fn from(cfg: Config<T>) -> kv::Config {
        cfg.0
    }
}

impl<T: Hash> Store<T> {
    /// Create a new configuration builder
    pub fn config<P: AsRef<Path>>(path: P) -> Config<T> {
        kv::Config::default(path).into()
    }

    /// Create a new store from the given configuration
    pub fn new<C: Into<kv::Config>>(cfg: C) -> Result<Store<T>, kv::Error> {
        Ok(Store(kv::Store::new(cfg.into())?, PhantomData))
    }

    /// Open a readonly transaction
    pub fn read_txn<'a>(&'a self) -> Result<Txn<'a, T>, kv::Error> {
        Ok(Txn::from(self.0.read_txn()?))
    }

    /// Create a readonly transaction and pass it to the provided function
    pub fn with_read_txn<'a, Res, F: FnOnce(&Txn<'a, T>) -> Result<Res, kv::Error>>(
        &'a self,
        f: F,
    ) -> Result<Res, kv::Error> {
        let txn = self.read_txn()?;
        f(&txn)
    }

    /// Open a writable transaction
    pub fn write_txn<'a>(&'a mut self) -> Result<Txn<'a, T>, kv::Error> {
        Ok(Txn::from(self.0.write_txn()?))
    }

    /// Create a writable transaction and pass it to the provided function
    pub fn with_write_txn<'a, Res, F: FnOnce(&mut Txn<'a, T>) -> Result<Res, kv::Error>>(
        &'a mut self,
        f: F,
    ) -> Result<Res, kv::Error> {
        let mut txn = self.write_txn()?;
        let res = f(&mut txn)?;
        txn.0.commit()?;
        Ok(res)
    }

    /// Get handle to kv::Bucket
    pub fn bucket<'a, V: kv::Value<'a>>(
        &self,
        name: Option<&str>,
    ) -> Result<kv::Bucket<'a, Token<T>, V>, kv::Error> {
        self.0.bucket(name)
    }
}
