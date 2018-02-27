use kv;

use token::Token;

/// A Txn is used to read and write from a Store
pub struct Txn<'a>(pub kv::Txn<'a>);

impl <'a> Txn<'a> {

    /// Add a value to the store and return a token
    pub fn put<'b, V: kv::Value<'b>>(&mut self, bucket: &kv::Bucket, val: V) -> Result<Token, kv::Error> {
        let token = Token::generate(&val);
        match self.0.set_unique(bucket, &token, val) {
            Ok(()) => (),
            Err(ref err) if err.key_exists_error() => (),
            Err(err) => return Err(err)
        }
        Ok(token)
    }

    /// Fetch the value associated with the given token
    pub fn fetch<V: kv::Value<'a>>(&'a self, bucket: &kv::Bucket, token: &Token) -> Result<V, kv::Error> {
        self.0.get(bucket, token)
    }

    /// Delete the value associated with the given token from the store
    pub fn del(&mut self, bucket: &kv::Bucket, token: &Token) -> Result<(), kv::Error> {
        self.0.del(bucket, token)
    }
}
