extern crate kryolite_smart_contract;

use kryolite_smart_contract::*;
use std::mem::take;

#[derive(Serialize)]
pub struct KryoliteLottery {
  pub ticket_price: u64,
  pub registration_open: bool,
  pub registrants: Vec<Address>,
  pub last_winner: Winner
}

#[derive(Serialize, Clone)]
pub struct Winner {
  pub address: Address,
  pub reward: u64
}

#[smart_contract]
impl KryoliteLottery {

  pub fn new() -> KryoliteLottery {
    KryoliteLottery {
      ticket_price: 100kryo,
      registration_open: true,
      registrants: Vec::new(),
      last_winner: Winner {
        address: NULL_ADDRESS,
        reward: 0
      }
    }
  }

  pub fn buy_ticket(&mut self) {
    require(TRANSACTION.value == self.ticket_price);
    require(self.registration_open);

    let fee: u64 = TRANSACTION.value / 100;
    CONTRACT.owner.transfer(fee); // small fee for owner

    self.registrants.push(TRANSACTION.from);

    event!(TicketSold, &TRANSACTION.from, &self.registrants.len());
  }

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

    self.last_winner = Winner {
      address: winner,
      reward: prize_pool
    };

    event!(AnnounceWinner, &winner, &prize_pool);
  }

  pub fn open_registration(&mut self) {
    require(TRANSACTION.from == CONTRACT.owner);
    self.registration_open = true;
    
    event!(RegistrationsOpen);
  }

  pub fn close_registration(&mut self) {
    require(TRANSACTION.from == CONTRACT.owner);
    self.registration_open = false;

    event!(RegistrationsClosed);
  }

  pub fn set_ticket_price(&mut self, new_price: u64) {
    require(TRANSACTION.from == CONTRACT.owner);
    require(!self.registration_open);
    require(self.registrants.len() == 0);

    self.ticket_price = new_price;
  }

  // non-mutable function, possible to call this without transaction
  pub fn tickets_sold(&self) -> usize {
    self.registrants.len()
  }

  // non-mutable function, possible to call this without transaction
  pub fn get_last_winner(&self) -> Winner {
    self.last_winner.clone()
  }

  // non-mutable function, possible to call this without transaction
  pub fn get_state(&self) -> KryoliteLottery {
    KryoliteLottery {
      ticket_price: self.ticket_price,
      registration_open: self.registration_open,
      registrants: self.registrants.clone(),
      last_winner: self.last_winner.clone()
    }
  }
}
