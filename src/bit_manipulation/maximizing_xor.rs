// TODO: Can we do better?
pub fn max_xor(l: usize, r: usize) -> usize {
    let mut max = 0;
    for i in l..=r {
        for j in i..=r {
            max = (i ^ j).max(max);
        }
    }
    max
}

#[cfg(test)]
#[test]
fn test_max_xor() {
    assert_eq!(7, max_xor(10, 15));
    assert_eq!(127, max_xor(11, 100));
}
