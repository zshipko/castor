use kv;

use token::Token;

/// A Txn is used to read and write from a Store
pub struct Txn<'a, V: kv::Value<'a>>(pub kv::Txn<'a, Token, V>);

impl <'a, V: kv::Value<'a>> Txn<'a, V> {
    /// Add a value to the store and return a token
    pub fn put<V0: Into<V>>(&mut self, bucket: &kv::Bucket<'a, Token, V>, val: V0) -> Result<Token, kv::Error> {
        let v = val.into();
        let token = Token::generate(&v);
        match self.0.set_no_overwrite(bucket, token.clone(), v) {
            Ok(()) => (),
            Err(ref err) if err.key_exists_error() => (),
            Err(err) => return Err(err)
        }
        Ok(token)
    }

    /// Fetch the value associated with the given token
    pub fn fetch(&'a self, bucket: &kv::Bucket<'a, Token, V>, token: Token) -> Result<V, kv::Error> {
        self.0.get(bucket, token)
    }

    /// Delete the value associated with the given token from the store
    pub fn del(&mut self, bucket: &kv::Bucket<'a, Token, V>, token: Token) -> Result<(), kv::Error> {
        self.0.del(bucket, token)
    }
}
