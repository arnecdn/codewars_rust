fn duplicate_encode(word: &str) -> String {
    word.to_lowercase().chars().fold(String::new(), |acc, c| -> String{
        match word.matches(c).count() {
            ..=1 => format!("{}(", acc),
            _   =>   format!("{})", acc),
        }
    })
}

#[test]
fn run_tests() {
    assert_eq!(duplicate_encode("din"), "(((");
    assert_eq!(duplicate_encode("recede"), "()()()");
    assert_eq!(duplicate_encode("Success"), ")())())", "should ignore case");
    assert_eq!(duplicate_encode("(( @"), "))((");
}