use crate::{Address, U256, __transfer_token, __consume_token, push_return, __approval};

pub trait KRC165 {
    fn supports_interface(interface_id: i32) -> bool;
}

pub trait KRC721 {
    fn balance_of(&self, owner: Address) -> usize;
    fn owner_of(&self, token_id: U256) -> Address;
    fn approve(&mut self, to: Address, token_id: U256);
    fn get_approved(&self, token_id: U256) -> Address;
    fn transfer_from(&mut self, from: Address, to: Address, token_id: U256, data: Vec<u8>);

    fn balance_of_json(&self, owner: Address);
    fn owner_of_json(&self, token_id: U256);
    fn get_approved_json(&self, token_id: U256);
}

/*pub trait KRC721JSON {
    fn balance_of_json(&self, owner: Address);
    fn owner_of_json(&self, token_id: U256);
    fn get_approved_json(&self, token_id: U256);
}

impl KRC721JSON for dyn KRC721 {
    #[export_name = "balance_of"]
    fn balance_of_json(&self, owner: Address) {
        let result = self.balance_of(owner);
        let json = serde_json::to_string(&result).unwrap();
        push_return(json.as_str());
    }

    #[export_name = "owner_of"]
    fn owner_of_json(&self, token_id: U256) {
        let result = self.owner_of(token_id);
        let json = serde_json::to_string(&result).unwrap();
        push_return(json.as_str());
    }

    #[export_name = "get_approved"]
    fn get_approved_json(&self, token_id: U256) {
        let result = self.get_approved(token_id);
        let json = serde_json::to_string(&result).unwrap();
        push_return(json.as_str());
    }
}*/

pub struct KRC721Event;
impl KRC721Event {
    pub fn transfer(from: &Address, to: &Address, token_id: &U256) {
        unsafe {
            __transfer_token(from as *const Address, to as *const Address, token_id as *const U256);
        }
    }

    pub fn consume(owner: &Address, token_id: &U256) {
        unsafe {
            __consume_token(owner as *const Address, token_id as *const U256);
        }
    }

    pub fn approval(from: &Address, to: &Address, token_id: &U256) {
        unsafe {
            __approval(from as *const Address, to as *const Address, token_id as *const U256);
        }
    }
}

pub trait KRC721Metadata {
    fn name(&self) -> String;
    fn symbol(&self) -> String;
    fn token_uri(&self, token_id: U256) -> String;

    fn name_json(&self);
    fn symbol_json(&self);
    fn token_uri_json(&self, token_id: U256);
}

/*pub trait KRC721MetadataJSON {
    fn name_json(&self);
    fn symbol_json(&self);
    fn token_uri_json(&self, token_id: U256);
}

impl KRC721MetadataJSON for &dyn KRC721Metadata {
    #[export_name = "name"]
    fn name_json(&self) {
        let result = self.name();
        let json = serde_json::to_string(&result).unwrap();
        push_return(json.as_str());
    }

    #[export_name = "symbol"]
    fn symbol_json(&self) {
        let result = self.symbol();
        let json = serde_json::to_string(&result).unwrap();
        push_return(json.as_str());
    }

    #[export_name = "token_uri"]
    fn token_uri_json(&self, token_id: U256) {
        let result = self.token_uri(token_id);
        let json = serde_json::to_string(&result).unwrap();
        push_return(json.as_str());
    }
}*/

pub trait KRC721Enumerable {
    fn total_supply(&self);
    fn token_by_index(&self);
    fn token_of_owner_by_index(&self);
}
