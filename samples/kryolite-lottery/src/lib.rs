extern crate kryolite_smart_contract;

use kryolite_smart_contract::*;
use std::{mem::take};

#[smart_contract_state]
pub struct KryoliteLottery {
    pub ticket_price: u64,
    pub registration_open: bool,
    pub registrants: Vec<Address>
}

#[smart_contract]
impl KryoliteLottery {

  #[exported]
  pub unsafe fn new() -> KryoliteLottery {
    KryoliteLottery {
        ticket_price: 100kryo,
        registration_open: true,
        registrants: Vec::new()
    }
  }

  #[exported]
  pub fn buy_ticket(&mut self) {
    require(TRANSACTION.value == self.ticket_price);
    require(self.registration_open);

    let fee: u64 = TRANSACTION.value / 100;
    CONTRACT.owner.transfer(fee);
    CONTRACT.address.transfer(TRANSACTION.value - fee);

    self.registrants.push(TRANSACTION.from);

    event!(TicketSold, &TRANSACTION.from, &self.registrants.len());
  }

  #[exported]
  pub fn draw_winner(&mut self) {
    require(TRANSACTION.from == CONTRACT.owner);
    require(!self.registration_open);
    require(self.registrants.len() > 0);

    let registrants: Vec<Address> = take(&mut self.registrants);
    let prize_pool = CONTRACT.balance;

    let count: f32 = registrants.len() as f32;
    let random: usize = (rand() * count) as usize;
    let winner: Address = registrants[random];

    winner.transfer(prize_pool);
    self.registration_open = true;

    event!(AnnounceWinner, &winner, &prize_pool);
  }

  #[exported]
  pub fn open_registration(&mut self) {
    require(TRANSACTION.from == CONTRACT.owner);
    self.registration_open = true;
    
    event!(RegistrationsOpen);
  }

  #[exported]
  pub fn close_registration(&mut self) {
    require(TRANSACTION.from == CONTRACT.owner);
    self.registration_open = false;

    event!(RegistrationsClosed);
  }

  #[exported]
  pub fn set_ticket_price(&mut self, new_price: u64) {
    require(TRANSACTION.from == CONTRACT.owner);
    require(!self.registration_open);
    require(self.registrants.len() == 0);

    self.ticket_price = new_price;
  }

  #[exported]
  pub fn testfn(&mut self, addr: &Address) {
    event!(TestEvent, &"foo", addr);
  }
}
