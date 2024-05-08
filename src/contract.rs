use soroban_sdk::{
    contract, contractimpl, Address, Env, Map, Vec
};

#[contract]
pub struct MapParser;

#[contractimpl]
impl MapParser {
    pub fn parse_map(e: Env, map: Map<Address, i128>) -> Vec<(Address, i128)> {
        let mut as_vec: Vec<(Address, i128)> = Vec::new(&e);
        for (k, v) in map.iter() {
            as_vec.push_back((k, v));
        }
        as_vec
    }

    pub fn parse_vec(e: Env, vec: Vec<(Address, i128)>) -> Map<Address, i128> {
        let mut as_map: Map<Address, i128> = Map::new(&e);
        for (k, v) in vec.iter() {
            as_map.set(k, v);
        }
        as_map
    }
}
