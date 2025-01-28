fn find_missing(seq: &[i32]) -> i32 {
    let first = *seq.first().unwrap();
    let last = *seq.last().unwrap();

    let step = (last - first) as usize / seq.len();
    let complete_sum = (first..=last).step_by(step).sum::<i32>();
    let missing = complete_sum - seq.iter().sum::<i32>();
    missing
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::find_missing;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(a: &[i32], expected: i32) {
        assert_eq!(find_missing(a), expected, "{ERR_MSG} with seq = {a:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&[1, 3, 5, 9, 11], 7);
        dotest(&[1, 2, 3, 4, 6, 7, 8, 9], 5);
        dotest(&[1, 3, 4, 5, 6, 7, 8, 9], 2);
    }
}
