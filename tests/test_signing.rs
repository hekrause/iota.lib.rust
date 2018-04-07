
extern crate iota_lib_rust;

use iota_lib_rust::crypto::signing::signing::*;
use iota_lib_rust::crypto::converter::converter::*;
use iota_lib_rust::utils::seed_random_generator::*;

#[test]
fn test_key() {
    let mut signing = Signing::new();
    let seed = generate_new_seed();
    println!("SEED: {:?}", seed);
    let key = signing.key(trits_string(seed).unwrap(), 0, 1);
    println!("KEY: {:?}", key);
}