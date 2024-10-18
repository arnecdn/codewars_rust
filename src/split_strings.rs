fn solution(s: &str) -> Vec<String> {
    s.chars().collect::<Vec<_>>().chunks(2).map(|t| {
        match t.len() {
            1 => format!("{}_", t[0]),
            _ => format!("{}{}", t[0], t[1])
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solution("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(solution(""), [] as [&str; 0]);
    }
}
