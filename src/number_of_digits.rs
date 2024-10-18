fn digits(n: u64) -> usize {
    return n.to_string().len();
}

#[test]
fn sample_test() {
    assert_eq!(digits(5), 1);
    assert_eq!(digits(12345), 5);
    assert_eq!(digits(9876543210), 10);
}
