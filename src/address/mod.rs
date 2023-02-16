use super::__transfer;
use serde::{Serialize, Deserialize, de::Visitor};

pub static NULL_ADDRESS: Address = Address([0; 26]);

#[repr(C)]
#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Address(pub [u8; 26]);

#[allow(dead_code)]
impl Address {
  pub fn transfer(&self, amount :u64) {
    unsafe {
      __transfer(self as *const Address, amount);
    }
  }

  pub fn as_bytes(&self) -> &[u8] {
    &self.0
  }

  pub fn as_string(&self) -> String {
    let addr = bs58::encode(self.0)
      .with_alphabet(bs58::Alphabet::FLICKR)
      .into_string();

    "kryo:".to_owned() + &addr
  }

  pub const fn len(&self) -> usize {
    self.0.len()
  }

  pub fn as_ptr(&self) -> *const u8 {
    self.0.as_ptr() as *const u8
  }
}

impl AsRef<[u8]> for Address {
  fn as_ref(&self) -> &[u8] {
    unsafe { std::slice::from_raw_parts(self.as_ptr(), self.len()) }
  }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
          serializer.serialize_str(self.as_string().as_str())
    }
}

impl <'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        deserializer.deserialize_str(StringVisitor)
    }
}

struct StringVisitor;

impl<'de> Visitor<'de> for StringVisitor {
    type Value = Address;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string represents Address")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
      let addr = bs58::decode(v.replace("kryo:", ""))
        .with_alphabet(bs58::Alphabet::FLICKR)
        .into_vec()
        .unwrap();

      let bytes: [u8; 26] = addr.try_into().unwrap();

      Ok(Address(bytes))
    }
}
