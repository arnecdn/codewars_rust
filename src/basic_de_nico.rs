// https://www.codewars.com/kata/596f610441372ee0de00006e/train/rust

use std::collections::HashMap;

fn de_nico(key: &str, msg: &str) -> String {
    let key_codes = generate_key_codes(key);

    let chunked_message = chunk_message(msg.to_string(), &key_codes);

    let encoded_message = map_chunked_to_key_codes(chunked_message);

    let decoded_message = decode_message_by_key_codes(msg, key_codes, encoded_message);

    parse_decoded_message(decoded_message)
}

fn generate_key_codes(key: &str) -> Vec<usize> {
    let mut sorted_keys = key.clone().chars().collect::<Vec<_>>();
    sorted_keys.sort();

    key.chars().fold(vec! {}, |mut acc, c| {
        acc.push(sorted_keys.iter().position(|&p| p == c).unwrap() + 1);
        acc
    })
}


fn chunk_message<'a>(msg: String, key_codes: &Vec<usize>) -> Vec< Vec<char>> {
    msg.chars().collect::<Vec<_>>()
        .chunks(key_codes.len()).collect::<Vec<_>>()
        .iter().map(|&f| f.to_vec()).collect()
}

fn map_chunked_to_key_codes<'a>(chunked_message: Vec<Vec<char>>) -> HashMap<usize, Vec<char>> {
    let mut encoded_message: HashMap<usize, Vec<char>> = HashMap::new();

    for m in chunked_message {
        for (j, &c) in m.iter().enumerate() {
            encoded_message.entry(j + 1).or_insert(vec![]).push(c);
        }
    }
    encoded_message
}

fn decode_message_by_key_codes(msg: &str, key_codes: Vec<usize>, encoded_message: HashMap<usize, Vec<char>>) -> Vec<char> {
    let res = key_codes.iter().fold(vec![], |mut acc, k| {
        acc.push(encoded_message.get(k).unwrap());
        acc
    });

    let mut decoded_message = vec![];
    let mut i = 0;
    let mut c = 0;

    while i < msg.len() {
        for j in 0..res.len() {
            if c < res[j].len() {
                decoded_message.push(res[j][c]);
                i += 1;
            }
        }
        c += 1;
    }
    decoded_message
}


fn parse_decoded_message(decoded_message: Vec<char>) -> String {
    let res = decoded_message.iter().fold(String::new(), |mut acc, &c| {
        acc.push(c);
        acc
    });
    res.trim().to_string()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::de_nico;

    fn dotest(k: &str, s: &str, expected: &str) {
        let actual = de_nico(k, s);
        assert!(actual == expected,
                "With key = \"{k}\", msg = \"{s}\"\nExpected \"{expected}\" but got \"{actual}\"")
    }

    #[test]
    fn fixed_tests() {
        dotest("crazy", "cseerntiofarmit on  ", "secretinformation");
        dotest("crazy","cseerntiofarmit on","secretinformation");
        dotest("abc","abcd","abcd");
        dotest("ba","2143658709", "1234567890");
        dotest("a","message","message");
        dotest("key","eky","key");
    }
}
