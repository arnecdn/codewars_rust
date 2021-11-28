fn change(string: &str) -> String {
    return "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .map(|c| {
            if string.to_lowercase().contains(c) {
                '1'
            } else {
                '0'
            }
        })
        .collect::<String>();

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(change("a **&  bZ"), "11000000000000000000000001");
    }
}
