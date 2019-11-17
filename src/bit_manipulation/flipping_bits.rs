pub fn flip_bits(n: u32) -> u32 {
    n ^ u32::max_value()
}

#[cfg(test)]
#[test]
fn test_flip_bits() {
    for i in 0..1_000_000 {
        assert_eq!(!i, flip_bits(i));
    }
}
