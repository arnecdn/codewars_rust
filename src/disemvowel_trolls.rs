const VOWELS :&[char]=&['a', 'e', 'i', 'o', 'u'];
fn disemvowel( s: &str) -> String {
    s.chars().filter(|c| !VOWELS.contains(&c.to_lowercase().next().unwrap())).collect()
}

// https://www.codewars.com/kata/52fba66badcd10859f00097e/train/rust
#[cfg(test)]
mod tests {
    use super::disemvowel;

    #[test]
    fn example_test() {
        assert_eq!(disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!");
    }
}