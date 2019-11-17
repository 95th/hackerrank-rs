use std::collections::HashMap;

// TODO: Can we do better?
pub fn lonely_integer(nums: &[isize]) -> isize {
    let mut map = HashMap::new();
    for &n in nums {
        *map.entry(n).or_insert(0_usize) += 1;
    }
    map.into_iter()
        .find(|(_, v)| *v == 1)
        .map(|(k, _)| k)
        .unwrap()
}

#[cfg(test)]
#[test]
fn test_lonely_integer() {
    assert_eq!(4, lonely_integer(&[1, 2, 3, 4, 3, 2, 1]));
    assert_eq!(2, lonely_integer(&[1, 1, 2]));
    assert_eq!(2, lonely_integer(&[0, 0, 1, 2, 1]));
}
