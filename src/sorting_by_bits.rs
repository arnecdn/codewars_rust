fn sort_by_bit(arr: &mut Vec<u32>) {
    arr.sort_by(|a, b| {
        if a.count_ones().cmp(&b.count_ones()).is_eq() { a.cmp(&b) } else { a.count_ones().cmp(&b.count_ones()) }
    });
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::sort_by_bit;

    fn dotest(a: &[u32], expected: &[u32]) {
        let mut v = a.to_vec();
        sort_by_bit(&mut v);
        assert!(v == expected, "With arr = {a:?}\nExpected {expected:?} but got {v:?}")
    }

    #[test]
    fn fixed_tests() {
        for (arr, expected) in [
            (vec![3, 8, 3, 6, 5, 7, 9, 1], vec![1, 8, 3, 3, 5, 6, 9, 7]),
            (vec![9, 4, 5, 3, 5, 7, 2, 56, 8, 2, 6, 8, 0], vec![0, 2, 2, 4, 8, 8, 3, 5, 5, 6, 9, 7, 56])
        ] {
            dotest(&arr, &expected);
        }
    }
}