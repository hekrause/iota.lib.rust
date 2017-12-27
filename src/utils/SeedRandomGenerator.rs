
extern crate rand;

use utils::SeedRandomGenerator::rand::distributions::IndependentSample;
use utils::SeedRandomGenerator::rand::distributions::Range;

static TRYTE_ALPHABET: [char; 27] = ['9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
                                'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

static SEED_LENGTH_MAX: u8 = 81;

pub fn generateNewSeed() -> String {
    let mut builder = "".to_string();

    let range = Range::new(0, TRYTE_ALPHABET.len());
    let mut rng = rand::thread_rng();

    for _ in 0..SEED_LENGTH_MAX {
        builder = format!("{}{}", builder, TRYTE_ALPHABET[range.ind_sample(&mut rng)]);
    }
    return builder;
}