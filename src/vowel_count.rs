fn get_count(string: &str) -> usize {
    let vowels = ['a','e','i','o','u'];
    string.matches(|c| vowels.contains(&c)).count()
}

#[test]
fn my_tests() {
    assert_eq!(get_count("abracadabra"), 5);
}