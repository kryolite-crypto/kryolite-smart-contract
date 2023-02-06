use super::__transfer;
use serde::Serialize;

pub static NULL_ADDRESS: Address = Address([0; 26]);

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct Address([u8; 26]);

#[allow(dead_code)]
impl Address {
  pub fn transfer(&self, amount :u64) {
    unsafe {
      __transfer(self as *const Address, amount);
    }
  }

  pub fn as_str(&self) -> &str {
    std::str::from_utf8(&self.0).unwrap()
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn as_ptr(&self) -> *const u8 {
    self.0.as_ptr() as *const u8
  }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
          serializer.serialize_str(self.as_str())
    }
}

/*impl <'de> Deserialize<'de> for Address {
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
        let arr: [char; 26] = v.as_bytes().try_into().unwrap();
        Ok(Address(arr))
    }
}
*/