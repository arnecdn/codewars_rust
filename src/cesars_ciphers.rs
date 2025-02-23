use std::cmp::Ordering;

struct CaesarCipher {
    alphabet: Vec<char>,
    shift: u32,
}

impl CaesarCipher {
    fn new(shift: u32) -> CaesarCipher {
        let alphabet = ('A'..='Z').collect::<Vec<_>>();
        CaesarCipher { shift, alphabet }
    }

    fn encode(&self, message: &str) -> String {
        message
            .to_uppercase()
            .chars()
            .map(|c| self.char_encoder(c))
            .collect::<String>()
    }

    fn char_encoder(&self, c: char) -> char {
        self.alphabet.iter().position(|&a| a == c).map_or(c, |index|{
                let temp_caesar_index = index + self.shift as usize;

                let caesar_index = match temp_caesar_index.cmp(&self.alphabet.len()) {
                    Ordering::Less => temp_caesar_index,
                    _ => temp_caesar_index - self.alphabet.len(),
                };

                self.alphabet[caesar_index]
        })
    }

    fn decode(&self, message: &str) -> String {
        message
            .to_uppercase()
            .chars()
            .map(|c| self.char_decoder(c))
            .collect::<String>()
    }

    fn char_decoder(&self, c: char) -> char {
        self.alphabet.iter().position(|&a| a == c).map_or(c, |index|{
            let temp_caesar_index: i32 = (index as i32 - self.shift as i32);

            let caesar_index = match temp_caesar_index.cmp(&0) {
                Ordering::Less => self.alphabet.len() - temp_caesar_index.abs() as usize,
                _ => temp_caesar_index as usize,
            };

            self.alphabet[caesar_index]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(shift: u32, message: &str, expected_encoded: &str, expected_decoded: &str) {
        let cipher = CaesarCipher::new(shift);
        assert_eq!(
            cipher.encode(&message),
            expected_encoded,
            "Encoded message does not match expected message for input: \"{message}\""
        );
        assert_eq!(
            cipher.decode(&expected_encoded),
            expected_decoded,
            "Decoded message does not match expected message for input: \"{expected_encoded}\""
        );
    }

    #[test]
    fn shift_of_5() {
        dotest(5, "Codewars", "HTIJBFWX", "CODEWARS");
        dotest(5, "WAFFLES", "BFKKQJX", "WAFFLES");
        dotest(
            5,
            "IT'S A SHIFT CIPHER!!",
            "NY'X F XMNKY HNUMJW!!",
            "IT'S A SHIFT CIPHER!!",
        );
        dotest(
            5,
            "IT\'S A SHIFT CIPHER!!",
            "NY\'X F XMNKY HNUMJW!!",
            "IT\'S A SHIFT CIPHER!!",
        );
    }

    #[test]
    fn shift_of_13() {
        dotest(13, "CNAPNXRF", "PANCAKES", "CNAPNXRF");
        dotest(13, "JAVASCRIPT", "WNINFPEVCG", "JAVASCRIPT");
    }

    #[test]
    fn simple_test() {
        dotest(0, "Codewars", "CODEWARS", "CODEWARS");
        dotest(1, "", "", "");
        dotest(2, " ", " ", " ");
    }
}
