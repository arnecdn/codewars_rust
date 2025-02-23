use regex::Regex;

fn is_valid_message(msg: &str) -> bool {
    let msg_tokenizer_re = Regex::new(r"\d*[a-zA-Z]*").unwrap();
    let msg_part_re = Regex::new(r"(?P<msg_length>\d*)(?P<msg_letters>[a-zA-Z]*)").unwrap();

    for msg_token in msg_tokenizer_re.find_iter(msg) {
        if let Some(token_oarts) = msg_part_re.captures(msg_token.as_str()) {
            let msg_length = token_oarts
                .name("msg_length")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap_or(0);
            let msg_letters = token_oarts.name("msg_letters").unwrap().as_str();

            if msg_length != msg_letters.len() {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(is_valid_message("3hey5hello2hi"), true);
        assert_eq!(is_valid_message("4code13hellocodewars"), true);
        assert_eq!(is_valid_message("3hey5hello2hi5"), false);
        assert_eq!(is_valid_message("code4hello5"), false);
        assert_eq!(is_valid_message("1a2bb3ccc4dddd5eeeee"), true);
        assert_eq!(is_valid_message(""), true);
    }
}
