use super::{Address, NULL_ADDRESS};

use lazy_static::lazy_static;

#[repr(C, packed)]
#[derive(Copy, Clone, PartialEq)]
pub struct ContractData {
  pub address: Address,
  pub owner: Address,
  pub balance: u64
}

#[no_mangle]
static mut _CONTRACT: ContractData = ContractData {
  address: NULL_ADDRESS,
  owner: NULL_ADDRESS,
  balance: 0,
};

fn contract() -> &'static ContractData {
  unsafe {
    return &_CONTRACT;
  }
}

lazy_static! {
  pub static ref CONTRACT: &'static ContractData = contract();
}
