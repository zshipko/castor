use sha2;
use sha2::Digest;
use kv::Value;

use std::str;

/// A Token is a unique identifier for a stored value
#[derive(Clone)]
pub struct Token(String);

impl Token {
    /// Generate a new token for the given value
    ///
    /// Tokens are generated using SHA256
    pub fn generate<'a, V: Value<'a>>(val: &V) -> Token {
        let mut hasher = sha2::Sha256::default();
        hasher.input(val.as_ref());
        let hash = hasher.result();
        Token::new(String::from(format!("{:x}", hash)))
    }

    /// Create a new token from an existing token string
    pub fn new(s: String) -> Token {
        Token(s)
    }

    /// Return a reference to the inner string
    pub fn as_string(&self) -> &String {
        &self.0
    }
}


impl AsRef<[u8]> for Token {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

// Unsafe, should only be used with known input
impl <'a> From<&'a [u8]> for Token {
    fn from(x: &'a [u8]) -> Token {
        unsafe {
            Token(String::from(str::from_utf8_unchecked(x)))
        }
    }
}
