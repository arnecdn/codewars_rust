fn valid_isbn10(isbn: &str) -> bool {
    if isbn.len() != 10 {
        return false;
    }

    if isbn[0..9].contains(|x: char| !x.is_ascii_digit()) {
        return false;
    }

    if isbn[9..10].contains(|x: char| !x.is_ascii_digit() && x != 'X') {
        return false;
    }

    isbn.chars().enumerate().fold(0, |acc, f| {
        let value = if f.1 == 'X' { 10 } else { f.1.to_digit(10).unwrap() };
        acc + value * (f.0 + 1) as u32
    },
    ).checked_rem(11).unwrap() == 0
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
// https://www.codewars.com/kata/51fc12de24a9d8cb0e000001/train/rust
#[cfg(test)]
mod tests {
    use super::valid_isbn10;

    fn dotest(isbn: &str, expected: bool) {
        let actual = valid_isbn10(isbn);
        assert!(actual == expected, "Test failed with isbn = {}\nExpected {} but got {}", isbn,expected, actual)
    }

    #[test]
    fn sample_tests() {
        dotest("1112223339", true);
        dotest("048665088X", true);
        dotest("1293000000", true);
        dotest("1234554321", true);
        dotest("1234512345", false);
        dotest("1293", false);
        dotest("ABCDEFGHIJ", false);
        dotest("X123456788", false);
        dotest("XXXXXXXXXX", false);
        dotest("123456789T", false);
    }
}
