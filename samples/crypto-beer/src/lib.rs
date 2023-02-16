extern crate kryolite_smart_contract;

use kryolite_smart_contract::*;
use std::collections::HashMap;

const DNA_DIGITS: u32 = 10;
const DNA_MODULUS: u64 = 10_u64.pow(DNA_DIGITS);

#[derive(Clone)]
pub struct Beer {
  name: String,
  dna: u32
}

#[smart_contract_state]
pub struct CryptoBeer {
  beers: Vec<Beer>,
  beer_owners: HashMap<Address, Vec<Beer>>
}

#[smart_contract]
impl CryptoBeer {

  pub fn new() -> CryptoBeer {
    CryptoBeer {
        beers: Vec::new(),
        beer_owners: HashMap::new()
    }
  }

  /// Brew random beer with given name
  #[exported]
  pub fn brew_random_beer(&mut self, name: &str) {
    let rand_dna = self.generate_random_dna(name);
    self.brew_beer(name, rand_dna);
  }

  /// Return array of beers for given address
  #[exported]
  pub fn get_beers_by_owner(&self, addr: &Address) -> Vec<Beer> {
    match self.beer_owners.get(addr) {
      Some(beer_collection) => {
        return beer_collection.to_vec().clone();
      }
      None => ()
    };

    Vec::<Beer>::new()
  }

  /// Internal function to brew with name and dna
  fn brew_beer(&mut self, name: &str, dna: u32) {
    require(self.is_unique(name, dna)/*, "Beer already exists."*/);

    let beer = Beer {
      name: name.to_string(),
      dna
    };

    match self.beer_owners.get_mut(&TRANSACTION.from) {
      Some(beer_collection) => {
        beer_collection.push(beer); 
      },
      None => { 
        self.beer_owners.insert(TRANSACTION.from, vec! { beer }); 
      }
    }
  }

  /// Internal function to generate random dna for given name and sender
  fn generate_random_dna(&self, name: &str) -> u32 {
    let hash = sha256(name.as_bytes()) + sha256(TRANSACTION.from.as_bytes());

    let rand = hash % DNA_MODULUS.into();
    rand.as_u32()
  }

  /// Check for unique beer identifier
  fn is_unique(&self, name: &str, dna: u32) -> bool {
    if self.beers.iter().any(|beer| beer.name == name && beer.dna == dna) {
      return false;
    }

    return true;
  }
}

impl KRC721Metadata for CryptoBeer
{
  
}

impl KRC721 for CryptoBeer
{

}

/*
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
  pub fn testfn(&self, addr: &Address) -> Address {
  }

  #[exported]
  pub fn tickets_sold(&self) -> usize {
    self.registrants.len()
  }*/
