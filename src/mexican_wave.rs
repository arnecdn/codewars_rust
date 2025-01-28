fn wave(s: &str) -> Vec<String> {
    s.chars().enumerate().fold((s, vec![]), |mut acc, c| -> (&str, Vec<_>){
        if c.1.is_alphabetic() {
            acc.1.push(format!("{}{}{}", &acc.0[..c.0], c.1.to_uppercase(), &acc.0[c.0 + 1..]));
        }
        acc
    }).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let expect = ["Hello", "hEllo", "heLlo", "helLo", "hellO"];
        assert_eq!(wave("hello"), expect);

        let expect = ["Codewars", "cOdewars", "coDewars", "codEwars", "codeWars", "codewArs", "codewaRs", "codewarS"];
        assert_eq!(wave("codewars"), expect);

        let expect: [&str; 0] = [];
        assert_eq!(wave(""), expect);

        let expect = ["Two words", "tWo words", "twO words", "two Words", "two wOrds", "two woRds", "two worDs", "two wordS"];
        assert_eq!(wave("two words"), expect);

        let expect = [" Gap ", " gAp ", " gaP "];
        assert_eq!(wave(" gap "), expect);
    }
}
