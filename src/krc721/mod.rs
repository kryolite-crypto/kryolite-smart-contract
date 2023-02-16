use bigint::U256;

use crate::Address;

pub trait KRC165 {
    fn supports_interface() -> bool;
}

pub trait KRC721 {
    fn balance_of(&self, owner: Address);
    fn owner_of(&self, token_id: U256);
    fn approve(&self, to: Address, token_id: U256);
    fn get_approved(&self, token_id: U256);
    fn set_approval_for_all(&self, operator: Address, approved: bool);
    fn is_approved_for_all(&self, owner: Address, operator: Address);
    fn safe_transfer_from<T>(&self, from: Address, to: Address, token_id: U256, data: T) where T: Into<Option<U256>>;
}

pub trait KRC721Metadata {
    fn name();
    fn symbol();
    fn token_uri();
}

pub trait KRC721Enumerable {
    fn total_supply(&self);
    fn token_by_index(&self);
    fn token_of_owner_by_index(&self);
}

pub struct KRC721Event;