
extern crate sha3;

use self::sha3::{Digest, Sha3_384};
use pow::TCurl::TCurl;
use pow::TCurl::Mode;

pub struct Kerl {
    pub keccak: sha3::Sha3_384,
    byte_state: Box<[u8]>,
    trit_state: Box<[u64]>
}

impl TCurl for Kerl {
    fn new(_mode: Mode) -> Self {
        Kerl {
            keccak: Sha3_384::default(),
            byte_state: Box::new([]),
            trit_state: Box::new([])
        }
    }

    fn absorb(&mut self, _trits: &Vec<i8>, _offset: usize, _length: usize) {}
    fn squeeze(&mut self, _trits: &mut Vec<i8>, _offset: usize, _length: usize) {}
    fn transform(&mut self) {}
    fn reset(&mut self ) {}
    fn get_state(&self) {}
    fn set_state(&self) {}
    fn clone(&self) {}
}