#[cfg(test)]
mod tests {
    extern crate iota_lib_rust;

    use self::iota_lib_rust::utils::SeedRandomGenerator;

    #[test]
    #[ignore]
    fn test_new_seed_length() {
        let seed = SeedRandomGenerator::generateNewSeed();
        assert_eq!(seed.len(), 81);
    }

    #[test]
    #[ignore]
    fn test_generate_differente_seeds() {
        let seed1 = SeedRandomGenerator::generateNewSeed();
        let seed2 = SeedRandomGenerator::generateNewSeed();
        let seed3 = SeedRandomGenerator::generateNewSeed();
        let seed4 = SeedRandomGenerator::generateNewSeed();
        assert_ne!(seed1, seed2);
        assert_ne!(seed1, seed3);
        assert_ne!(seed1, seed4);
        assert_ne!(seed2, seed3);
        assert_ne!(seed2, seed4);
        assert_ne!(seed3, seed4);
    }
}
