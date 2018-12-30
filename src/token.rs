use std::marker::PhantomData;
use std::str;

use digest::Digest;
use kv::Value;

/// Hash adds the `hex` method to types that already implement `Digest` to simplify the process of
/// generating a new token
pub trait Hash: Digest {
    /// `hex` should return the hexidecimal representation of the hash without the "0x" prefix
    fn hex(self) -> String;
}

/// A Token is a unique identifier for a stored value
pub struct Token<T: Hash>(String, PhantomData<T>);

impl<T: Hash> Clone for Token<T> {
    fn clone(&self) -> Self {
        Token(self.0.clone(), PhantomData)
    }
}

impl<'a, T: Hash> kv::Value<'a> for Token<T> {
    fn from_raw(data: &[u8]) -> Token<T> {
        Token::from(data)
    }
}

impl<T: Hash> Token<T> {
    /// Generate a new token for the given value
    pub fn generate<'a, V: Value<'a>>(val: &V) -> Token<T> {
        let mut hasher = T::new();
        hasher.input(val.as_ref());
        let hash = hasher.hex();
        Token::new(hash)
    }

    /// Create a new token from an existing token string
    pub fn new(s: String) -> Token<T> {
        Token(s, PhantomData)
    }

    /// Return a reference to the inner string
    pub fn as_string(&self) -> &String {
        &self.0
    }
}

impl<T: Hash> AsRef<[u8]> for Token<T> {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

/// This is unsafe but is needed to implement kv::Key
impl<'a, T: Hash> From<&'a [u8]> for Token<T> {
    fn from(x: &'a [u8]) -> Token<T> {
        let s = unsafe { str::from_utf8_unchecked(x) };
        Token::new(String::from(s))
    }
}
