use std::marker::PhantomData;

use crate::token::{Hash, Token};

/// A Txn is used to read and write from a Store
pub struct Txn<'a, T: Hash>(pub kv::Txn<'a>, PhantomData<T>);

impl<'a, T: Hash> From<Txn<'a, T>> for kv::Txn<'a> {
    fn from(txn: Txn<'a, T>) -> kv::Txn<'a> {
        txn.0
    }
}

impl<'a, T: Hash> From<kv::Txn<'a>> for Txn<'a, T> {
    fn from(txn: kv::Txn<'a>) -> Txn<'a, T> {
        Txn(txn, PhantomData)
    }
}

impl<'a, T: Hash> AsRef<kv::Txn<'a>> for Txn<'a, T> {
    fn as_ref(&self) -> &kv::Txn<'a> {
        &self.0
    }
}

impl<'a, T: Hash> AsMut<kv::Txn<'a>> for Txn<'a, T> {
    fn as_mut(&mut self) -> &mut kv::Txn<'a> {
        &mut self.0
    }
}

impl<'a, T: Hash> Txn<'a, T> {
    /// Add a value to the store and return a token
    pub fn put<V: kv::Value<'a>>(
        &mut self,
        bucket: &kv::Bucket<'a, Token<T>, V>,
        val: V,
    ) -> Result<Token<T>, kv::Error> {
        let token = Token::generate(&val);
        match self.0.set_no_overwrite(bucket, token.clone(), val) {
            Ok(()) => (),
            Err(ref err) if err.key_exists_error() => (),
            Err(err) => return Err(err),
        }
        Ok(token)
    }

    /// Fetch the value associated with the given token
    pub fn fetch<V: kv::Value<'a>>(
        &'a self,
        bucket: &kv::Bucket<'a, Token<T>, V>,
        token: Token<T>,
    ) -> Result<V, kv::Error> {
        self.0.get(bucket, token)
    }

    /// Delete the value associated with the given token from the store
    pub fn del<V: kv::Value<'a>>(
        &mut self,
        bucket: &kv::Bucket<'a, Token<T>, V>,
        token: Token<T>,
    ) -> Result<(), kv::Error> {
        self.0.del(bucket, token)
    }
}
