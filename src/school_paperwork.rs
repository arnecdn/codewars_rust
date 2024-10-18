fn paperwork(n: i16, m: i16) -> u32 {
    match (n, m) {
        (0.., 0..) => (n * m) as u32,
        _ => 0
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::paperwork;

    fn dotest(n: i16, m: i16, expected: u32) {
        let actual = paperwork(n, m);
        assert!(actual == expected,
                "With n = {}, m = {}\nExpected {} but got {}",n,m,expected,actual)
    }

    #[test]
    fn test_paperwork() {
        dotest(5, 5, 25);
        dotest(5, -5, 0);
        dotest(-5, -5, 0);
        dotest(-5, 5, 0);
        dotest(5, 0, 0);
    }
}
