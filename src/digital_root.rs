
fn digital_root(n: i64) -> i64 {
    if n < 10 {
        return n;
    }

    digital_root(
        n.to_string()
            .chars()
            .map(|e| e.to_digit(10).unwrap() as i64)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(digital_root(16), 7);
    }
}
