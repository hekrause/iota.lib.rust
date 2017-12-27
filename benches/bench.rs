
#![feature(test)]

extern crate test;
extern crate iota_lib_rust;

use test::test::Bencher;
use iota_lib_rust::utils::Converter::*;
use iota_lib_rust::utils::SeedRandomGenerator;

#[bench]
fn bench_generate_new_seed(b: &mut Bencher) {
    b.iter(|| {
        SeedRandomGenerator::generateNewSeed();
    });
}

#[bench]
fn bench_from_tryte_to_trits(b: &mut Bencher) {
    b.iter(|| {
        let seed = "FMLDUZJUKVORHGJLYOTFSLZWPYEIFGTSH9VOOCU9SKPNYHAPHQJJUMCRSCJECBATLMOWCXUROIPWOLSVM".to_string();
        let _trits = from_tryte_to_trits(seed);
    });
}

#[bench]
fn bench_from_trits_to_tryte(b: &mut Bencher) {
    b.iter(|| {
        let trits = vec![1, -1, 0, 1, 1, -1, 1, -1, 0, 1, 1, 0, 0, -1, 0, 1, -1, 1, -1, 0, 1, -1, 0, 1, 0, 0, 0, -1, 0, 1, 1, 1, 0, 0, 0, 0, -1, 1, 1, 0, 0, 1, 0, -1, 1, -1, 1, 1, -1, -1, -1, 1, 1, 0, 1, 0, -1, 0, 0, -1, 1, 1, -1, 1, -1, 0, 0, -1, 1, 0, -1, 1, 0, -1, 0, 1, 0, 0, -1, 0, -1, -1, 1, 0, 1, -1, -1, 1, -1, 1, 1, -1, -1, 0, 0, 1, 1, -1, 1, 1, 0, 0, 0, -1, 1, 0, -1, -1, 1, 1, -1, -1, 0, 0, -1, -1, -1, 1, 0, -1, 0, -1, -1, 1, 1, -1, -1, -1, 0, -1, -1, 0, -1, -1, 0, 0, 1, 0, 0, 1, 0, -1, 0, 1, -1, 1, 1, -1, 0, 0, 1, 0, 0, 1, 0, 0, -1, 1, 0, -1, 1, 1, -1, -1, 0, 0, 1, -1, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, 0, -1, -1, 0, -1, -1, -1, 1, -1, -1, 1, 1, -1, 0, -1, 1, -1, -1, -1, 1, 0, 0, -1, 0, -1, 1, -1, 0, 1, 1, 1, -1, -1, 1, 1, 0, 1, 1, 1, -1, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, 1, 0, -1, 1, -1, 0, -1, 1, 1, 0, 1, 1, -1, 0];
        let _tryte= from_trits_to_tryte(trits);
    });
}