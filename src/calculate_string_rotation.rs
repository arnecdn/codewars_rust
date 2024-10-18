fn shifted_diff(first: & str, second: &str) -> Option<usize> {
    let mut res = String::from(first);
    if first == second {
        return Some(0);
    }

    for i in 0..res.len() {
        res = format!("{}{}", &res.chars().nth(res.len() - 1).unwrap(), &res[..res.len() - 1]);

        if res == second {
            return Some(i + 1);
        }
    }

    None
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // assert_eq!(shifted_diff("eecoff", "coffee"), Some(4));
        // assert_eq!(shifted_diff("Moose", "moose"), None);
        // assert_eq!(shifted_diff("isn't", "'tisn"), Some(2));
        // assert_eq!(shifted_diff("Esham", "Esham"), Some(0));
        // assert_eq!(shifted_diff(" ", " "), Some(0));
        assert_eq!(shifted_diff("hoop", "pooh"), None);
        assert_eq!(shifted_diff("  ", " "), None);
    }
}
