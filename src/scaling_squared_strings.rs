
fn scale(s: &str, k: u32, n: u32) -> String {

    s.lines().flat_map(|w|{
        let horisontal_extended_word = w.chars().map(|c|String::from(c).repeat(k as usize)).collect::<String>();
        vec![horisontal_extended_word; n as usize]
    }).collect::<Vec<_>>().join("\n")
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::scale;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest<S>(s: &str, k: u32, n: u32, expected: S)
    where
        S: AsRef<str> + std::cmp::Ord + std::fmt::Debug,
        std::string::String: std::cmp::PartialEq<S>,
    {
        assert_eq!(
            scale(s, k, n),
            expected,
            "{ERR_MSG} with s = \n\"{s}\",\nk = {k}, n = {n}"
        )
    }

    #[test]
    fn sample_tests() {
        dotest("abcd\nefgh\nijkl\nmnop", 2, 3, "aabbccdd\naabbccdd\naabbccdd\neeffgghh\neeffgghh\neeffgghh\niijjkkll\niijjkkll\niijjkkll\nmmnnoopp\nmmnnoopp\nmmnnoopp");
        dotest("", 5, 5, "");
        dotest("Kj\nSH", 1, 2, "Kj\nKj\nSH\nSH");
    }
}
