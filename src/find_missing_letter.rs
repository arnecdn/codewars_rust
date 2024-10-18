
fn find_missing_letter(chars: &[char]) -> char {
    return (chars[0]..=chars[chars.len() - 1]).find(|c| !chars.contains(&c)).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_tests() {
        assert_eq!(find_missing_letter(&['a', 'b', 'c', 'd', 'f']), 'e');
        assert_eq!(find_missing_letter(&['O', 'Q', 'R', 'S']), 'P');
    }
}