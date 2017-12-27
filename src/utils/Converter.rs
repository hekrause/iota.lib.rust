
const TRYTE_ALPHABET: [char; 27] = ['9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
                                    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

const TRITS_MAPPING: [[i8; 3]; 27] =   [[ 0,  0,  0],
                                        [ 1,  0,  0],
                                        [-1,  1,  0],
                                        [ 0,  1,  0],
                                        [ 1,  1,  0],
                                        [-1, -1,  1],
                                        [ 0, -1,  1],
                                        [ 1, -1,  1],
                                        [-1,  0,  1],
                                        [ 0,  0,  1],
                                        [ 1,  0,  1],
                                        [-1,  1,  1],
                                        [ 0,  1,  1],
                                        [ 1,  1,  1],
                                        [-1, -1, -1],
                                        [ 0, -1, -1],
                                        [ 1, -1, -1],
                                        [-1,  0, -1],
                                        [ 0,  0, -1],
                                        [ 1,  0, -1],
                                        [-1,  1, -1],
                                        [ 0,  1, -1],
                                        [ 1,  1, -1],
                                        [-1, -1,  0],
                                        [ 0, -1,  0],
                                        [ 1, -1,  0],
                                        [-1,  0,  0]];

pub fn from_tryte_to_trits(tryte: String) -> Vec<i8> {
    let mut d = Vec::with_capacity(3 * tryte.len());

    for i in 0..tryte.len() {
        let c = tryte.chars().nth(i).unwrap();
        let index = TRYTE_ALPHABET.iter().enumerate().find(|tmp|*tmp.1 as char == c).unwrap().0;
        d.push(TRITS_MAPPING[index][0]);
        d.push(TRITS_MAPPING[index][1]);
        d.push(TRITS_MAPPING[index][2]);
    }

    return d;
}

pub fn from_trits_to_tryte(trits: Vec<i8>) -> String {
    let mut tryte = String::new();
    for i in (0..trits.len()).step_by(3) {
        for j in 0..TRITS_MAPPING.len() {
            if TRITS_MAPPING[j][0] == trits[i] && TRITS_MAPPING[j][1] == trits[i + 1] && TRITS_MAPPING[j][2] == trits[i + 2] {
                tryte = format!("{}{}", tryte, TRYTE_ALPHABET[j]);
            }
        }
    }
    return tryte;
}
