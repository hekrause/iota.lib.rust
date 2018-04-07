
pub const RADIX: u8 = 3;
pub const MAX_TRIT_VALUE: i8 = 1;
pub const MIN_TRIT_VALUE: i8 = -1;

const TRYTES_ALPHABET: [char; 27] = ['9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
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

pub fn trits_number(input: i64) -> Vec<i8> {
    let mut trits: Vec<i8> = vec![];
    let mut abs_value = if input < 0 {-input} else {input} as u64;

    while abs_value > 0 {
        let mut remainder = (abs_value % RADIX as u64) as i8;
        abs_value = f64::floor(abs_value as f64 / RADIX as f64) as u64;

        if remainder > MAX_TRIT_VALUE {
            remainder = MIN_TRIT_VALUE;
            abs_value = abs_value + 1;
        }
        trits.push(remainder);
    }

    if input < 0 {
        for i in 0..trits.len() {
            trits[i] = -trits[i];
        }
    }
    return trits;
}

pub fn trits_string(input: String) -> Result<Vec<i8>, Box<::std::option::NoneError>> {
    let mut trits: Vec<i8> = vec![0; input.len()*3];

    for i in 0..input.len() {
        let elem = input.chars().nth(i)?;
        let index = TRYTES_ALPHABET.iter().position(|&s| s == elem)?;
        trits[(i * 3) + 0] = TRITS_MAPPING[index][0];
        trits[(i * 3) + 1] = TRITS_MAPPING[index][1];
        trits[(i * 3) + 2] = TRITS_MAPPING[index][2];
    }
    println!("{:?}", trits);
    Ok(trits)
}

pub fn trytes(trits: Vec<i8>) -> Result<String, Box<::std::option::NoneError>> {
    let mut trytes = String::from("");
    let mut i = 0;
    while i < trits.len() {
        for j in 0..TRYTES_ALPHABET.len() {
            if TRITS_MAPPING[j][0] == trits[i + 0] && TRITS_MAPPING[j][1] == trits[i + 1] && TRITS_MAPPING[j][2] == trits[i + 2] {
                trytes = trytes + &TRYTES_ALPHABET.get(j)?.to_string();
                break;
            }
        }
        i = i + 3;
    }
    Ok(trytes)
}

pub fn value(trits: Vec<i8>) -> i64 {
    let mut sum = 0i64;
    for i in (0..trits.len()).rev() {
        sum = (sum * 3) + trits[i] as i64;
    }
    return sum;
}

pub fn from_value(value: i64) -> Vec<i8> {
    let mut destination: Vec<i8> = vec![];
    let mut abs_value = if value < 0 {-value} else {value} as u64;

    while abs_value > 0 {
        let mut remainder = (abs_value % RADIX as u64) as i8;
        abs_value = f64::floor(abs_value as f64 / RADIX as f64) as u64;

        if remainder > MAX_TRIT_VALUE {
            remainder = MIN_TRIT_VALUE;
            abs_value = abs_value + 1;
        }

        destination.push(remainder);
    }

    if value < 0 {
        for j in 0..destination.len() {
            destination[j] = if destination[j] == 0 {0} else {-destination[j]};
        }
    }
    return destination;
}
