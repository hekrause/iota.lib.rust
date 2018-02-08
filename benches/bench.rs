
#![feature(test)]

extern crate test;
extern crate iota_lib_rust;

use test::test::Bencher;

use iota_lib_rust::utils::Signing::*;
use iota_lib_rust::utils::Converter::*;
use iota_lib_rust::pow::RCurl::*;
use iota_lib_rust::pow::TCurl::*;
use iota_lib_rust::utils::SeedRandomGenerator;

#[bench]
#[ignore]
fn bench_generate_new_seed(b: &mut Bencher) {
    b.iter(|| {
        SeedRandomGenerator::generateNewSeed();
    });
}

#[bench]
#[ignore]
fn bench_from_tryte_to_trits(b: &mut Bencher) {
    b.iter(|| {
        let seed = "FMLDUZJUKVORHGJLYOTFSLZWPYEIFGTSH9VOOCU9SKPNYHAPHQJJUMCRSCJECBATLMOWCXUROIPWOLSVM".to_string();
        let _trits = from_tryte_to_trits(seed);
    });
}

#[bench]
#[ignore]
fn bench_from_trits_to_tryte(b: &mut Bencher) {
    b.iter(|| {
        let trits = vec![1, -1, 0, 1, 1, -1, 1, -1, 0, 1, 1, 0, 0, -1, 0, 1, -1, 1, -1, 0, 1, -1, 0, 1, 0, 0, 0, -1, 0, 1, 1, 1, 0, 0, 0, 0, -1, 1, 1, 0, 0, 1, 0, -1, 1, -1, 1, 1, -1, -1, -1, 1, 1, 0, 1, 0, -1, 0, 0, -1, 1, 1, -1, 1, -1, 0, 0, -1, 1, 0, -1, 1, 0, -1, 0, 1, 0, 0, -1, 0, -1, -1, 1, 0, 1, -1, -1, 1, -1, 1, 1, -1, -1, 0, 0, 1, 1, -1, 1, 1, 0, 0, 0, -1, 1, 0, -1, -1, 1, 1, -1, -1, 0, 0, -1, -1, -1, 1, 0, -1, 0, -1, -1, 1, 1, -1, -1, -1, 0, -1, -1, 0, -1, -1, 0, 0, 1, 0, 0, 1, 0, -1, 0, 1, -1, 1, 1, -1, 0, 0, 1, 0, 0, 1, 0, 0, -1, 1, 0, -1, 1, 1, -1, -1, 0, 0, 1, -1, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, 0, -1, -1, 0, -1, -1, -1, 1, -1, -1, 1, 1, -1, 0, -1, 1, -1, -1, -1, 1, 0, 0, -1, 0, -1, 1, -1, 0, 1, 1, 1, -1, -1, 1, 1, 0, 1, 1, 1, -1, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, 1, 0, -1, 1, -1, 0, -1, 1, 1, 0, 1, 1, -1, 0];
        let _tryte= from_trits_to_tryte(trits);
    });
}

#[bench]
#[ignore]
fn bench_generate_new_address_with_security_18(b: &mut Bencher) {
    b.iter(|| {
        let mut signing: Signing<RCurl> = Signing::new(TCurl::new(Mode::CURLP81));
        let seed = SeedRandomGenerator::generateNewSeed();
        let key = signing.key(from_tryte_to_trits(seed), 0, 1);
        let digest = signing.digest(&key);
        let address_trits = signing.address(&digest);
        let _address = from_trits_to_tryte(address_trits);
    });
}