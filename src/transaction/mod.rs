use super::{Address, NULL_ADDRESS};

use lazy_static::lazy_static;

#[repr(C, packed)]
#[derive(Copy, Clone, PartialEq)]
pub struct TransactionData {
  pub from: Address,
  pub to: Address,
  pub value: u64
}

#[no_mangle]
static mut _TRANSACTION: TransactionData = TransactionData {
    from: NULL_ADDRESS,
    to: NULL_ADDRESS,
    value: 0
};

fn transaction() -> &'static TransactionData {
  unsafe {
    return &_TRANSACTION;
  }
}

lazy_static! {
  pub static ref TRANSACTION: &'static TransactionData = transaction(); //*transaction();
}
