
extern crate iota_lib_rust;

use iota_lib_rust::utils::seed_random_generator::generate_new_seed;

#[test]
fn test_generate_new_seed() {
    let seed = generate_new_seed();
    //println!("{:?}", seed);
}