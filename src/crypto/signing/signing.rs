
use crypto::kerl::kerl::Kerl;

const HASH_LENGTH: usize = 243;
const KEY_LENGTH: usize = 6561;

pub struct Signing {
    kerl: Kerl
}

impl Signing {
    pub fn new() -> Self {
        Signing {
            kerl: Kerl::new()
        }
    }

    pub fn key(&mut self, in_seed: Vec<i8>, index: u8, mut security: usize) -> Vec<i8> {

        let mut seed = in_seed.clone();
        let seed_len = seed.len();

        for _ in 0..index {
            for j in 0..HASH_LENGTH {
                let tmp_seed = seed[j] + 1;
                if tmp_seed > 1 {
                    seed[j] = -1;
                } else {
                    break;
                }
            }
        }

        self.kerl.reset();
        self.kerl.absorb(&seed, 0, seed_len);
        self.kerl.squeeze(&mut seed, 0, HASH_LENGTH);
        self.kerl.reset();
        self.kerl.absorb(&seed, 0, HASH_LENGTH);

        let mut key: Vec<i8> = vec![0i8; security * HASH_LENGTH * 27];
        let mut offset = 0;

        loop {
            for _ in 0..27 {
                self.kerl.squeeze(&mut key, offset, HASH_LENGTH);
                offset += HASH_LENGTH;
            }

            if security > 1 {
                security -= 1;
            } else {
                break;
            }
        }
        return key;
    }

    pub fn digest(&mut self, key: &Vec<i8>) -> Vec<i8> {
        let security = (((key.len()/KEY_LENGTH)as f64).floor()) as usize;
        let mut digest = vec![0i8; security * HASH_LENGTH];
        let mut key_fragment = vec![0i8; KEY_LENGTH];

        for i in 0..security {
            key_fragment[0..KEY_LENGTH].clone_from_slice(&key[(i*KEY_LENGTH)..((i+1)*KEY_LENGTH)]);

            for j in 0..27 {
                for _ in 0..26 {
                    self.kerl.reset();
                    self.kerl.absorb(&key_fragment, j*HASH_LENGTH, HASH_LENGTH);
                    self.kerl.squeeze(&mut key_fragment, j*HASH_LENGTH, HASH_LENGTH);
                }
            }

            let len = key_fragment.len();

            self.kerl.reset();
            self.kerl.absorb(&key_fragment, 0, len);
            self.kerl.squeeze(&mut digest, i*HASH_LENGTH, HASH_LENGTH);
        }
        return digest;
    }

    pub fn address(&mut self, digest: &Vec<i8>) -> Vec<i8> {
        let mut address = vec![0i8; HASH_LENGTH];

        self.kerl.reset();
        self.kerl.absorb(digest, 0, digest.len());
        self.kerl.squeeze(&mut address, 0, digest.len());

        return address;
    }
}
