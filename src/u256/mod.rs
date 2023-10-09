use std::ops;
use super::B32;
use num_bigint::BigUint;
use serde::{Serialize, Deserialize, de::Visitor};
use lazy_static::lazy_static;

fn u256_max_value() -> &'static BigUint {
    return &U256_MAX_VALUE;
}

lazy_static! {
    pub static ref U256_MAX_VALUE: BigUint = BigUint::from_bytes_be(&[255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]);
}

#[repr(C)]
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct U256(pub [u8; 32]);

impl U256 {
    pub fn as_string(&self) -> String {
        B32.encode(&self.0)
    }
}

impl ops::Add<U256> for U256 {
    type Output = U256;

    fn add(self, rhs: U256) -> U256 {
        let a = BigUint::from_bytes_be(&self.0);
        let b = BigUint::from_bytes_be(&rhs.0);
        
        let mut bytes = ((a + b) % u256_max_value()).to_bytes_be();
        bytes.truncate(32);

        U256(bytes.try_into().unwrap())
    }
}

impl ops::Sub<U256> for U256 {
    type Output = U256;

    fn sub(self, rhs: U256) -> Self::Output {
        let a = BigUint::from_bytes_be(&self.0);
        let b = BigUint::from_bytes_be(&rhs.0);

        let mut bytes = ((a - b) % u256_max_value()).to_bytes_be();
        bytes.truncate(32);

        U256(bytes.try_into().unwrap())
    }
}

impl ops::Mul<U256> for U256 {
    type Output = U256;

    fn mul(self, rhs: U256) -> Self::Output {
        let a = BigUint::from_bytes_be(&self.0);
        let b = BigUint::from_bytes_be(&rhs.0);

        let mut bytes = ((a * b) % u256_max_value()).to_bytes_be();
        bytes.truncate(32);

        U256(bytes.try_into().unwrap())
    }
}

impl ops::Div<U256> for U256 {
    type Output = U256;

    fn div(self, rhs: U256) -> Self::Output {
        let a = BigUint::from_bytes_be(&self.0);
        let b = BigUint::from_bytes_be(&rhs.0);

        U256((a / b).to_bytes_be().try_into().unwrap())
    }
}

impl ops::Rem<U256> for U256 {
    type Output = U256;

    fn rem(self, rhs: U256) -> Self::Output {
        let a = BigUint::from_bytes_be(&self.0);
        let b = BigUint::from_bytes_be(&rhs.0);

        U256((a % b).to_bytes_be().try_into().unwrap())
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
        write!(formatter, "a string represents U256")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
      let bytes = B32.decode(v.as_bytes()).unwrap();
      Ok(U256(bytes.try_into().unwrap()))
    }
}
