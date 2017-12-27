
#[cfg(test)]
mod tests {
    extern crate iota_lib_rust;

    use self::iota_lib_rust::utils::Converter::*;

    #[test]
    fn test_from_tryte_to_trits() {
        let seed = "YVYDXGHH9HD9KIFKNDSRVYFFXAQBPGPIGAFOVZNSOVWWWCCHKZAABKWU9ONWWNPVFNAQYMEJVQXQCTFJY".to_string();
        let trits = from_tryte_to_trits(seed);
        let expected = vec![1, -1, 0, 1, 1, -1, 1, -1, 0, 1, 1, 0, 0, -1, 0, 1, -1, 1, -1, 0, 1, -1, 0, 1, 0, 0, 0, -1, 0, 1, 1, 1, 0, 0, 0, 0, -1, 1, 1, 0, 0, 1, 0, -1, 1, -1, 1, 1, -1, -1, -1, 1, 1, 0, 1, 0, -1, 0, 0, -1, 1, 1, -1, 1, -1, 0, 0, -1, 1, 0, -1, 1, 0, -1, 0, 1, 0, 0, -1, 0, -1, -1, 1, 0, 1, -1, -1, 1, -1, 1, 1, -1, -1, 0, 0, 1, 1, -1, 1, 1, 0, 0, 0, -1, 1, 0, -1, -1, 1, 1, -1, -1, 0, 0, -1, -1, -1, 1, 0, -1, 0, -1, -1, 1, 1, -1, -1, -1, 0, -1, -1, 0, -1, -1, 0, 0, 1, 0, 0, 1, 0, -1, 0, 1, -1, 1, 1, -1, 0, 0, 1, 0, 0, 1, 0, 0, -1, 1, 0, -1, 1, 1, -1, -1, 0, 0, 1, -1, 0, 0, 0, 0, -1, -1, -1, -1, -1, -1, -1, 0, -1, -1, 0, -1, -1, -1, 1, -1, -1, 1, 1, -1, 0, -1, 1, -1, -1, -1, 1, 0, 0, -1, 0, -1, 1, -1, 0, 1, 1, 1, -1, -1, 1, 1, 0, 1, 1, 1, -1, -1, 0, -1, 0, -1, 0, -1, 0, -1, 0, 1, 0, -1, 1, -1, 0, -1, 1, 1, 0, 1, 1, -1, 0];
        assert_eq!(trits, expected);
    }

    #[test]
    fn test_from_trits_to_tryte() {
        let seed = "YVYDXGHH9HD9KIFKNDSRVYFFXAQBPGPIGAFOVZNSOVWWWCCHKZAABKWU9ONWWNPVFNAQYMEJVQXQCTFJY".to_string();
        let trits = from_tryte_to_trits(seed.clone());
        let tryte= from_trits_to_tryte(trits);
        assert_eq!(seed, tryte);
    }
}
