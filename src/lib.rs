mod address;
mod contract;
mod transaction;
mod krc721;
mod u256;

extern crate wee_alloc;
extern crate bigint;

use std::{alloc::{GlobalAlloc, Layout}, mem::size_of, any::type_name};

pub use bs58;
pub use u256::*;
pub use address::*;
pub use contract::*;
pub use transaction::*;
pub use kryolite_macro::*;
pub use krc721::*;
pub use serde::*;
pub use serde_json;

pub fn require(condition: bool) {
  if !condition {
    unsafe {
      __exit(-1);
    }
    unreachable!()
  }
}

pub fn rand() -> f32 {
  unsafe {
    return __rand();
  }
}

pub fn sha256(message: &[u8]) -> U256 {
  let digest = hashes::sha2::sha256::hash(message);
  let ptr = digest.into_bytes().as_ptr() as *const [u64; 4];

  unsafe { U256(bigint::U256(*ptr)) }
}

pub trait Numeric {}
impl Numeric for bool {}
impl Numeric for f64 {}
impl Numeric for f32 {}
impl Numeric for i64 {}
impl Numeric for i32 {}
impl Numeric for i16 {}
impl Numeric for i8 {}
impl Numeric for isize {}
impl Numeric for u64 {}
impl Numeric for u32 {}
impl Numeric for u16 {}
impl Numeric for u8 {}
impl Numeric for usize {}

pub trait PointerTrait {
  fn size(&self) -> usize;
  fn get_type(&self) -> &str;
  fn as_pointer(&self) -> *const u8;
}

impl<'a> PointerTrait for &'a str {
    fn size(&self) -> usize {
      self.len()
    }

    fn get_type(&self) -> &str {
        "str"
    }

    fn as_pointer(&self) -> *const u8 {
      self.as_ptr()
    }
}

impl PointerTrait for Address {
  fn size(&self) -> usize {
    self.len()
  }

  fn get_type(&self) -> &str {
      "Address"
  }

  fn as_pointer(&self) -> *const u8 {
    self.as_ptr()
  }
}

// TODO: U256

impl<T: Numeric> PointerTrait for T {
  fn size(&self) -> usize {
    size_of::<T>()
  }

  fn get_type(&self) -> &str {
    type_of(self)
  }

  fn as_pointer(&self) -> *const u8 {
    let ptr: *const T = self;
    ptr as *const u8
  }
}

#[allow(dead_code)]
pub fn println(val: &dyn PointerTrait) {
  let val_type = val.get_type();

  unsafe {
    __println(val_type.as_ptr(), val_type.len(), val.as_pointer(), val.size());
  }
}

pub fn append_event(val: &dyn PointerTrait) {
  let val_type = val.get_type();

  unsafe {
    __append_event(val_type.as_ptr(), val_type.len(), val.as_pointer(), val.size());
  }
}

pub fn publish_event() {
  unsafe {
    __publish_event();
  }
}

pub fn push_return(val: &str) {
  unsafe {
    __return(val.as_ptr(), val.len());
  }
}

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern "C" {
  pub fn __exit(exitCode: i32);
  pub fn __rand() -> f32;
  pub fn __transfer(addr_ptr: *const Address, value: u64);
  pub fn __transfer_token(from: *const Address, to: *const Address, token_id: *const U256);
  pub fn __consume_token(owner: *const Address, token_id: *const U256);
  pub fn __approval(from: *const Address, to: *const Address, token_id: *const U256);
  pub fn __println(typ: *const u8, type_len: usize, val: *const u8, val_len: usize);
  pub fn __append_event(typ: *const u8, type_len: usize, val: *const u8, val_len: usize);
  pub fn __publish_event();
  pub fn __return(str: *const u8, val_len: usize);
}

#[no_mangle]
pub unsafe fn __malloc(len: usize) -> *mut u8  {
  let layout = Layout::from_size_align(len, 1);
  ALLOC.alloc(layout.unwrap()) as *mut u8
}

#[no_mangle]
pub unsafe fn __free(ptr: *mut u8, len: usize) {
  let layout = Layout::from_size_align(len, 1);
  ALLOC.dealloc(ptr, layout.unwrap())
}

#[macro_export]
macro_rules! event {
  ($x:expr) => {{
    append_event(&stringify!($x));
    publish_event();
  }};
  ($x:expr, $($y:expr),*) => {{
    append_event(&stringify!($x));

    $(
      append_event($y);
    )*

    publish_event();
  }};
}

pub fn type_of<T>(_: T) -> &'static str {
  type_name::<T>()
}

#[derive(Serialize, Clone, PartialEq, Eq, Hash)]
pub struct StandardToken {
    pub name: String,
    pub description: String
}

pub trait KryoliteStandardToken {
    fn get_token(&self, token_id: U256) -> StandardToken;
    fn get_token_json(&self, token_id: U256);
}
