
extern crate iota_lib_rust;
extern crate sha3;

use iota_lib_rust::utils::SeedRandomGenerator;

use iota_lib_rust::pow::RCurl::*;
use iota_lib_rust::pow::TCurl::*;
use iota_lib_rust::utils::Signing::*;
use iota_lib_rust::utils::Converter::*;

#[test]
#[ignore]
fn test_new_address() {
    let mut signing: Signing<RCurl> = Signing::new(TCurl::new(Mode::CURLP81));
    let seed = SeedRandomGenerator::generateNewSeed();
    let key = signing.key(from_tryte_to_trits(seed), 0, 1);
    let digest = signing.digest(&key);
    let address_trits = signing.address(&digest);
    let address = from_trits_to_tryte(address_trits);
}
