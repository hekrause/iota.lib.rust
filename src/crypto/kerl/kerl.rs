extern crate tiny_keccak;

use self::tiny_keccak::Keccak;
use crypto::converter::words::*;

pub struct Kerl {
    k: Keccak
}

impl Kerl {
    pub fn new() -> Self {
        Kerl {
            k: Keccak::new_sha3_384()
        }
    }

    pub fn absorb(&mut self, trits: &Vec<i8>, mut offset: usize, mut length: usize) {
        while length > 0 {
            let limit = if length < 243 {length} else {243};
            let trit_state = &trits[offset..(offset + limit)];
            offset += limit;

            let words= trits_to_words(trit_state);
            let bytes= word_array_to_byte_array(&words);

            self.k.update(&bytes);
            length -= limit
        }
    }

    pub fn squeeze(&mut self, trits: &mut Vec<i8>, mut offset: usize, mut length: usize) {
        let limit = if length < 243 {length} else {243};

        while length > 0 {
            let k_copy = self.k.clone();
            let mut final_u8: [u8;BYTE_LENGTH] = [0;BYTE_LENGTH];
            let mut final_u32: [u32;INT_LENGTH];

            k_copy.finalize(&mut final_u8);
            final_u32 = byte_array_to_word_array(&final_u8);

            let trit_state: [i8;243] = words_to_trits(&mut final_u32);

            let mut i = 0;
            while i < limit {
                trits[offset] = trit_state[i] as i8;
                offset += 1;
                i += 1;

            }

            self.reset();

            for index in 0..final_u8.len() {
                final_u8[index] = final_u8[index] ^ 0xFFFF;
            }

            for index in 0..final_u32.len() {
                final_u32[index] = final_u32[index] ^ 0xFFFFFFFF;
            }

            self.k.update(&final_u8);
            length -= limit;
        }
    }

    pub fn reset(&mut self) {
        self.k = Keccak::new_sha3_384();
    }
}