use std::ops;

use serde::{Serialize, Deserialize, de::Visitor};

#[repr(C)]
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct U256(pub bigint::U256);

impl U256 {
    pub fn as_string(&self) -> String {
        let bytes: [u8; 32] = unsafe { std::mem::transmute(self.0) };
        bs58::encode(bytes)
            .with_alphabet(bs58::Alphabet::FLICKR)
            .into_string()
    }
}

impl ops::Add<U256> for U256 {
    type Output = U256;

    fn add(self, rhs: U256) -> U256 {
        U256(self.0 + rhs.0)
    }
}

impl ops::Sub<U256> for U256 {
    type Output = U256;

    fn sub(self, rhs: U256) -> Self::Output {
        U256(self.0 - rhs.0)
    }
}

impl ops::Mul<U256> for U256 {
    type Output = U256;

    fn mul(self, rhs: U256) -> Self::Output {
        U256(self.0 * rhs.0)
    }
}

impl ops::Div<U256> for U256 {
    type Output = U256;

    fn div(self, rhs: U256) -> Self::Output {
        U256(self.0 / rhs.0)
    }
}

impl ops::Rem<U256> for U256 {
    type Output = U256;

    fn rem(self, rhs: U256) -> Self::Output {
        U256(self.0 % rhs.0)
    }
}

// TODO: implement rest

impl Serialize for U256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
          serializer.serialize_str(self.as_string().as_str())
    }
}

impl <'de> Deserialize<'de> for U256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        deserializer.deserialize_str(StringVisitor)
    }
}

struct StringVisitor;

impl<'de> Visitor<'de> for StringVisitor {
    type Value = U256;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string represents Address")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
      let addr = bs58::decode(v)
        .with_alphabet(bs58::Alphabet::FLICKR)
        .into_vec()
        .unwrap();

      let bytes: [u8; 32] = addr.try_into().unwrap();

      Ok(U256(bytes.into()))
    }
}
