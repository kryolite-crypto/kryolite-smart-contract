extern crate kryolite_smart_contract;

use kryolite_smart_contract::*;
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Clone)]
pub struct KryoliteLottery {
  pub tickets_sold: u64,
  pub ticket_price: u64,
  pub registration_open: bool,
  pub tickets: HashMap<U256, Ticket>,
  pub ticket_to_address: HashMap<U256, Address>,
  pub address_to_tickets: HashMap<Address, HashSet<Ticket>>,
  pub approved_transfers: HashMap<U256, Address>,
  pub last_winner: Winner
}

#[derive(Serialize, Clone)]
pub struct Winner {
  pub address: Address,
  pub reward: u64
}

#[derive(Serialize, Clone, PartialEq, Eq, Hash)]
pub struct Ticket {
  pub token_id: U256,
  pub name: String
}

#[smart_contract]
impl KryoliteLottery {

  pub fn new() -> KryoliteLottery {
    KryoliteLottery {
      tickets_sold: 0,
      ticket_price: 100kryo,
      registration_open: true,
      tickets: HashMap::new(),
      ticket_to_address: HashMap::new(),
      address_to_tickets: HashMap::new(),
      approved_transfers: HashMap::new(),
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

    let ticket = self.print_ticket();

    self.ticket_to_address.insert(ticket.token_id, TRANSACTION.from);
    self.tickets.insert(ticket.token_id, ticket.clone());
    
    let tickets = self.address_to_tickets.entry(TRANSACTION.from).or_insert(HashSet::new());
    tickets.insert(ticket.clone());

    KRC721Event::transfer(&CONTRACT.address, &TRANSACTION.from, &ticket.token_id);
  }

  pub fn draw_winner(&mut self) {
    require(TRANSACTION.from == CONTRACT.owner);
    require(!self.registration_open);
    require(self.tickets.len() > 0);

    let prize_pool = CONTRACT.balance;
    let count = self.ticket_to_address.len() as f32;
    let random = (rand() * count) as usize;

    let winner = self.ticket_to_address.values().nth(random).unwrap();

    for (token_id, owner) in &self.ticket_to_address {
      KRC721Event::consume(&owner, &token_id);
    }

    winner.transfer(prize_pool);

    self.last_winner = Winner {
      address: *winner,
      reward: prize_pool
    };

    event!(AnnounceWinner, winner, &prize_pool);

    self.tickets.clear();
    self.ticket_to_address.clear();
    self.address_to_tickets.clear();
    self.approved_transfers.clear();
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
    require(self.tickets.len() == 0);

    self.ticket_price = new_price;
  }

  // non-mutable function, possible to call this without transaction
  pub fn tickets_sold(&self) -> usize {
    self.tickets.len()
  }

  // non-mutable function, possible to call this without transaction
  pub fn get_last_winner(&self) -> Winner {
    self.last_winner.clone()
  }

  // non-mutable function, possible to call this without transaction
  pub fn get_state(&self) -> KryoliteLottery {
    self.clone()
  }

  fn print_ticket(&mut self) -> Ticket {
    self.tickets_sold += 1;

    let name = "Kryolite Lottery Ticket #".to_string() + &self.tickets_sold.to_string();
    let token_id = sha256(name.as_bytes()) + sha256(TRANSACTION.from.as_bytes());

    Ticket {
      token_id,
      name
    }
  }

  fn is_approved_for(&self, spender: &Address, ticket_id: &U256) -> bool {
    if !self.ticket_to_address.contains_key(ticket_id) {
      return false;
    }

    let owner = self.ticket_to_address.get(ticket_id).unwrap();

    if spender == owner {
      return true;
    }

    if !self.approved_transfers.contains_key(ticket_id) {
      return false;
    }

    let approved_for = self.approved_transfers.get(ticket_id).unwrap();

    approved_for == spender
  }
}

#[interface]
impl KRC721 for KryoliteLottery {
  fn balance_of(&self, owner: Address) -> usize {
    self.address_to_tickets.get(&owner).unwrap().len()
  }

  fn owner_of(&self, token_id: U256) -> Address {
    self.ticket_to_address.get(&token_id).unwrap().clone()
  }

  fn approve(&mut self, to: Address, token_id: U256) {
    require(TRANSACTION.from == *self.ticket_to_address.get(&token_id).unwrap());
    self.approved_transfers.insert(token_id, to);
    KRC721Event::approval(&TRANSACTION.from, &to, &token_id);
  }

  fn get_approved(&self, token_id: U256) -> Address {
    *self.approved_transfers.get(&token_id).unwrap()
  }

  fn transfer_from(&mut self, from: Address, to: Address, token_id: U256, data: Vec<u8>) {
    require(from != NULL_ADDRESS && to != NULL_ADDRESS);
    require(self.tickets.contains_key(&token_id));
    require(from != to);
    require(self.is_approved_for(&TRANSACTION.from, &token_id));

    let ticket = self.tickets.get(&token_id).unwrap();

    let sender_tickets = self.address_to_tickets.get_mut(&from).unwrap();
    sender_tickets.remove(ticket);
    self.ticket_to_address.remove(&token_id);

    
    let recipient_tickets = self.address_to_tickets.entry(to).or_insert(HashSet::new());
    recipient_tickets.insert(ticket.clone());
    self.ticket_to_address.insert(token_id, to);

    self.approved_transfers.remove(&token_id);

    KRC721Event::transfer(&from, &to, &ticket.token_id);
  }
}

#[interface]
impl KRC721Metadata for KryoliteLottery {
  fn name(&self) -> String {
    "Kryolite Lottery".to_string()
  }

  fn symbol(&self) -> String {
    "LOTTO".to_string()
  }

  fn token_uri(&self, token_id: U256) -> String {
    format!("http://example.com/token/{}", token_id.as_string())
  }
}

#[interface]
impl KryoliteStandardToken for KryoliteLottery {
  fn get_token(&self, token_id: U256) -> StandardToken {
    let ticket = self.tickets.get(&token_id).unwrap();
    
    StandardToken {
      name: ticket.name.clone(),
      description: "Winner Winner Chicken Dinner".to_string()
    }
  }
}