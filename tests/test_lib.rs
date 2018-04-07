
extern crate iota_lib_rust;

use iota_lib_rust::crypto::kerl::kerl::*;

#[test]
fn test_main() {
    let input: Vec<u8> = From::from("9ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ9ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let test_data_trytes = input.iter().map(|x| *x as i8).collect::<Vec<i8>>();
    let test_data_len = test_data_trytes.len();
    let mut test_data_trits: Vec<i8> = vec![0; test_data_len];

    //println!("{:?}", test_data_trytes);

    let mut kerl = Kerl::new();
    kerl.absorb(&test_data_trytes, 0, test_data_len);
    kerl.squeeze(&mut test_data_trits, 0, test_data_len);

    //println!("{:?}", test_data_trits);
}