
extern crate sha3;

use self::sha3::{Digest, Sha3_384};
use pow::TCurl::TCurl;
use pow::TCurl::Mode;
use std::cmp::min;

const HASH_LENGTH: usize = 243;
const STATE_LENGTH: usize = 3 * HASH_LENGTH;
const TRUTH_TABLE: [i8; 11] = [1, 0, -1, 2, 1, -1, 0, 2, -1, 1, 0];

pub struct RCurl {
    pub keccak: sha3::Sha3_384,
    byte_state: Box<[u8]>,
    trit_state: Box<[u64]>,
    state: Vec<i8>,
    scratchpad: Vec<i8>,
    number_of_rounds: u8
}

impl TCurl for RCurl {
    fn new(mode: Mode) -> Self {
        match mode {
            Mode::CURLP81 => {
                return RCurl {
                    keccak: Sha3_384::default(),
                    byte_state: Box::new([]),
                    trit_state: Box::new([]),
                    state: vec![0i8; STATE_LENGTH],
                    scratchpad: vec![0i8; STATE_LENGTH],
                    number_of_rounds: 81
                }
            },
            Mode::CURLP27 => {
                return RCurl {
                    keccak: Sha3_384::default(),
                    byte_state: Box::new([]),
                    trit_state: Box::new([]),
                    state: vec![0i8; STATE_LENGTH],
                    scratchpad: vec![0i8; STATE_LENGTH],
                    number_of_rounds: 27
                }
            },
            Mode::KERL => RCurl::new(Mode::CURLP81)
        }
    }

    fn absorb(&mut self, trits: &Vec<i8>, mut offset: usize, length: usize) {
        let mut len = min(length, HASH_LENGTH);
        loop {
            self.state[0..len].clone_from_slice(&trits[offset..(offset+len)]);
            self.transform();

            offset = offset + HASH_LENGTH;
            len = len - HASH_LENGTH;

            if len <= 0 {
                break;
            }
        }
    }

    fn squeeze(&mut self, trits: &mut Vec<i8>, mut offset: usize, length: usize) {
        let mut len = min(length, HASH_LENGTH);
        loop {
            trits[offset..(offset+len)].clone_from_slice(&self.state[0..len]);
            self.transform();

            offset = offset + HASH_LENGTH;
            len = len - HASH_LENGTH;

            if len <= 0 {
                break;
            }
        }
    }

    fn transform(&mut self) {
        let mut scratchpad_index = 0;
        let mut prev_scratchpad_index = 0;

        for _ in 0..self.number_of_rounds {
            self.scratchpad[0..STATE_LENGTH].clone_from_slice(&self.state[0..STATE_LENGTH]);
            for state_index in 0..STATE_LENGTH {
                prev_scratchpad_index = scratchpad_index;

                if scratchpad_index < 365{
                    scratchpad_index = scratchpad_index + 364;
                } else {
                    scratchpad_index = scratchpad_index - 365;
                }

                let prev = self.scratchpad[prev_scratchpad_index];
                let i = self.scratchpad[scratchpad_index];
                let ix = i << 2;

                self.state[state_index] = TRUTH_TABLE[(prev + ix + 5) as usize]
            }
        }
    }

    fn reset(&mut self ) {
        self.state = vec![0i8; STATE_LENGTH];
    }

    fn get_state(&self) {}
    fn set_state(&self) {}
    fn clone(&self) {}
}