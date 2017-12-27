
use pow::TCurl::TCurl;

const HASH_LENGTH: usize = 243;
const KEY_LENGTH: usize = 6561;

pub struct Signing<T: TCurl> {
    curl: T
}

impl<T: TCurl> Signing<T> {
    pub fn new(curl: T) -> Self {
        Signing {
            curl: curl
        }
    }

    pub fn key(&mut self, inSeed: Vec<i8>, index: u8, mut security: usize) -> Vec<i8> {

        let mut seed = inSeed.clone();
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

        self.curl.reset();
        self.curl.absorb(&mut seed, 0, seed_len);
        self.curl.squeeze(&mut seed, 0, HASH_LENGTH);
        self.curl.reset();
        self.curl.absorb(&mut seed, 0, HASH_LENGTH);

        let mut key: Vec<i8> = vec![0i8; (security * HASH_LENGTH * 27)];
        let mut offset = 0;

        loop {
            for _ in 0..27 {
                self.curl.squeeze(&mut key, offset, HASH_LENGTH);
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
        let mut keyFragment = vec![0i8; KEY_LENGTH];

        for i in 0..security {
            keyFragment[0..KEY_LENGTH].clone_from_slice(&key[(i*KEY_LENGTH)..((i+1)*KEY_LENGTH)]);

            for j in 0..27 {
                for _ in 0..26 {
                    self.curl.reset();
                    self.curl.absorb(&mut keyFragment, j*HASH_LENGTH, HASH_LENGTH);
                    self.curl.squeeze(&mut keyFragment, j*HASH_LENGTH, HASH_LENGTH);
                }
            }

            let len = keyFragment.len();

            self.curl.reset();
            self.curl.absorb(&mut keyFragment, 0, len);
            self.curl.squeeze(&mut digest, i*HASH_LENGTH, HASH_LENGTH);
        }
        return digest;
    }

    pub fn address(&mut self, digest: &Vec<i8>) -> Vec<i8> {
        let mut address = vec![0i8; HASH_LENGTH];

        self.curl.reset();
        self.curl.absorb(digest, 0, digest.len());
        self.curl.squeeze(&mut address, 0, digest.len());

        return address;
    }
}
