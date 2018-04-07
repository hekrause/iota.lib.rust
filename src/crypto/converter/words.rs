
#![allow(exceeding_bitshifts)]

use std::mem::transmute;

pub const INT_LENGTH: usize = 12;
pub const BYTE_LENGTH: usize = 48;
pub const RADIX: u32 = 3;
const HALF_3: [u32;12] = [0xa5ce8964, 0x9f007669, 0x1484504f, 0x3ade00d9,
                        0x0c24486e, 0x50979d57, 0x79a4c702, 0x48bbae36,
                        0xa9f6808b, 0xaa06a805, 0xa87fabdf, 0x5e69ebef];

pub fn trits_to_words(trits: &[i8]) -> [u32; 12] {
    let mut base: [u32; INT_LENGTH] = [0; INT_LENGTH];

    let mut size = 1;

    for i in  (0..trits.len()).rev() {
        let trit = trits[i] + 1;
        {
            let sz = size;
            let mut carry = 0;

            for j in 0..sz {
                //println!("j: {:?}", j);
                let v: u64 = (base[j] as u64 * RADIX as u64) + carry as u64;
                carry = rshift(v, 32);
                base[j] = ((v & 0xFFFFFFFF) >> 0) as u32;
                //println!("base_len: {:?}", base.len());
            }

            if carry > 0 {
                println!("sz {:?}", sz);
                base[sz] = carry;
                size += 1;
            }
        }

        {
            let sz = bigint_add_small(&mut base, trit as u32);
            if sz > size {
                size = sz;
            }
        }
    }

    if !is_null(base) {
        if bigint_cmp(&HALF_3, &base) <= 0 {
            bigint_sub(&mut base, &mut HALF_3.clone());
        } else {
            let mut tmp = HALF_3;
            bigint_sub(&mut tmp, &mut base);
            bigint_not(&mut tmp);
            bigint_add_small(&mut tmp, 1);
            base = tmp;
        }
    }

    ta_reverse(&mut base);

    for i in 0..base.len() {
        base[i] = swap32(base[i]);
    }

    return base;
}

pub fn words_to_trits(words: &mut [u32;INT_LENGTH]) -> [i8;243] {
    let mut trits: [i8;243] = [0;243];
    let mut base: [u32; INT_LENGTH] = words.clone();

    ta_reverse(&mut base);

    let mut flip_trits = false;
    if base[INT_LENGTH - 1] >> 31 == 0 {
        bigint_add(&mut base, HALF_3);
    } else {
        bigint_not(&mut base);
        if bigint_cmp(&mut base, &mut HALF_3.clone()) > 0 {
            bigint_sub(&mut base, &mut HALF_3.clone());
            flip_trits = true;
        } else {
            bigint_add_small(&mut base, 1);
            let mut tmp = HALF_3;
            bigint_sub(&mut tmp, &mut base);
            base = tmp;
        }
    }

    for i in 0..243 {
        let mut rem: i64 = 0;
        for j in (0..INT_LENGTH).rev() {
            let calc = rem * 0xFFFFFFFF + rem + base[j] as i64;

            let mut lhs = if rem != 0 { calc } else { 0 };
            lhs = lhs + base[j] as i64;
            let rhs = RADIX as i64;

            let q = f64::ceil(lhs as f64 / rhs as f64) as i64 >> 0;
            let r = (lhs % rhs) >> 0;

            base[j] = q as u32;
            rem = r;
        }
        trits[i] = (rem - 1) as i8;
    }

    if flip_trits {
        for i in 0..trits.len() {
            trits[i] = !trits[i];
        }
    }

    return trits;
}

fn ta_reverse(array: &mut [u32; INT_LENGTH]) {
    let n = array.len();
    let middle = f32::floor(n as f32/2.0) as usize;

    for i in 0..middle {
        let tmp = array[i];
        array[i] = array[n - 1 - i];
        array[n - 1 - i] = tmp;
    }
}

fn bigint_not(arr: &mut [u32; INT_LENGTH]) {
    for i in 0.. arr.len() {
        arr[i] = !arr[i] >> 0;
    }
}

fn rshift(number: u64, shift: u32) -> u32 {
    return ((number / 2u64.pow(shift)) >> 0) as u32;
}

fn swap32(val: u32) -> u32 {
    return ((val & 0xFF) << 24) | ((val & 0xFF00) << 8) | ((val >> 8) & 0xFF00) | ((val >> 24) & 0xFF)
}

fn full_add(lh: u32, rh: u32, carry: bool) -> (u32, bool){
    let mut v = (lh as u64 + rh as u64) as u32;
    let mut l = (rshift(v as u64, 32)) & 0xFFFFFFFF;
    let mut r = (v & 0xFFFFFFFF) >> 0;
    let carry1 = l != 0;

    if carry {
        v = r + 1;
    }

    l = (rshift(v as u64, 32)) & 0xFFFFFFFF;
    r = (v & 0xFFFFFFFF) >> 0;
    let carry2 = l != 0;

    return (r, (carry1 || carry2));
}

fn bigint_sub(base: &mut [u32; INT_LENGTH], rh: &mut [u32; INT_LENGTH]) {
    let mut noborrow = true;

    for i in 0..base.len() {
        let vc = full_add(base[i],!rh[i] >> 0,noborrow);
        base[i] = vc.0;
        noborrow = vc.1;
    }
}

fn bigint_cmp(lh: &[u32; INT_LENGTH], rh: &[u32; INT_LENGTH]) -> isize {
    for i in  (0..lh.len()).rev() {
        let a = lh[i] >> 0;
        let b = rh[i] >> 0;

        if a < b {
            return -1;
        } else if a > b {
            return 1;
        }
    }
    return 0;
}

fn bigint_add(base: &mut [u32;INT_LENGTH], rh: [u32;INT_LENGTH]) {
    let mut carry = false;
    for i in 0..base.len() {
        let vc = full_add(base[i], rh[i], carry);
        base[i] = vc.0;
        carry = vc.1;
    }
}

fn bigint_add_small(base: &mut [u32; INT_LENGTH], other: u32) -> usize{
    let vc = full_add(base[0], other, false);
    base[0] = vc.0;
    let mut carry = vc.1;

    let mut i = 1;
    while carry && i < base.len() {
        let vc = full_add(base[0], other, carry);
        base[i] = vc.0;
        carry = vc.1;
        i = i + 1;
    }

    return i;
}

fn is_null(arr: [u32; INT_LENGTH]) -> bool{
    for index in 0..arr.len() {
        if arr[index] != 0 {
            return false;
        }
    }

    return true;
}

pub fn word_array_to_byte_array(words: &[u32; INT_LENGTH]) -> [u8; BYTE_LENGTH] {
    let mut bytes_array: [u8; BYTE_LENGTH] = [0;BYTE_LENGTH];
    for index in 0..words.len() {
        let bytes: [u8; 4] = unsafe { transmute(words[index].to_be()) };
        bytes_array[(index*4)] = bytes[0];
        bytes_array[(index*4)+1] = bytes[1];
        bytes_array[(index*4)+2] = bytes[2];
        bytes_array[(index*4)+3] = bytes[3];
    }
    return bytes_array;
}

pub fn byte_array_to_word_array(bytes: &[u8; BYTE_LENGTH]) -> [u32; INT_LENGTH] {
    let mut words_array: [u32; INT_LENGTH] = [0;INT_LENGTH];
    for index in 0..INT_LENGTH {
        let one = (bytes[(index*4)+0] as u32) << 24;
        let two = (bytes[(index*4)+1] as u32) << 16;
        let three = (bytes[(index*4)+2] as u32) << 8;
        let four = bytes[(index*4)+3] as u32;
        words_array[index] = (one | two | three | four) as u32;
    }
    return words_array;
}