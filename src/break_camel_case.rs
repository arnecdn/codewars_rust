fn solution(s: &str) -> String {
    s.chars().fold(String::new(), |acc, c| -> String {
        if c.is_uppercase() { format!("{} {}", acc, c) } else { format!("{}{}", acc, c) }
    })
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("camelCasing"), "camel Casing");
        assert_eq!(solution("camelCasingTest"), "camel Casing Test");
    }
}
