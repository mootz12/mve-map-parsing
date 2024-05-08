#![cfg(test)]

use std::println;

use crate::{
    MapParser, MapParserClient
};
use soroban_sdk::{
    testutils::Address as _, vec, Address, Env
};

mod map_parser_wasm {
    soroban_sdk::contractimport!(file = "./target/wasm32-unknown-unknown/release/mve_map_parsing.wasm");
}

#[test]
fn test_small_map_rust() {
    let env = Env::default();

    let map_parser_id = env.register_contract(None, MapParser {});
    let map_parser_client = MapParserClient::new(&env, &map_parser_id);

    let addr1 = Address::generate(&env);
    let addr2 = Address::generate(&env);
    let addr3 = Address::generate(&env);
    let addr4 = Address::generate(&env);
    let as_vec = vec![
        &env,
        (addr1.clone(), 1),
        (addr2.clone(), 2),
        (addr3.clone(), 3),
        (addr4.clone(), 4),
    ];

    let as_map = map_parser_client.parse_vec(&as_vec);

    assert!(as_map.len() == 4);
    println!("parsed vec");

    let as_vec2 = map_parser_client.parse_map(&as_map);

    assert!(as_vec2.len() == 4);
    println!("parsed map");
}

#[test]
fn test_small_map_wasm() {
    let env = Env::default();

    let map_parser_id = env.register_contract_wasm(None, map_parser_wasm::WASM);
    let map_parser_client = MapParserClient::new(&env, &map_parser_id);

    let addr1 = Address::generate(&env);
    let addr2 = Address::generate(&env);
    let addr3 = Address::generate(&env);
    let addr4 = Address::generate(&env);
    let as_vec = vec![
        &env,
        (addr1.clone(), 1),
        (addr2.clone(), 2),
        (addr3.clone(), 3),
        (addr4.clone(), 4),
    ];

    let as_map = map_parser_client.parse_vec(&as_vec);

    assert!(as_map.len() == 4);
    println!("parsed vec");

    let as_vec2 = map_parser_client.parse_map(&as_map);

    assert!(as_vec2.len() == 4);
    println!("parsed map");
}

