
fn decode(s: &str) -> String {
    let number = s.split(|c:char| c.is_alphabetic()).nth(0).unwrap();
    let mut result = vec![];

    for c in s[number.len()..].chars() {
        let index = c as u32- 97;

        if let Some(res) = find_x(index as i128, number.parse::<i128>().unwrap()) {
            let char = (res as u8+ 97) as char;
            result.push(char);
        } else {
            return String::from("Impossible to decode");
        }
    }

    result.iter().collect()
}

fn find_x(f_x: i128, num: i128) -> Option<i128> {
    let modulus: i128 = 26;

    match mod_inverse(num , modulus ) {
        Some(inverse_num) => Some((inverse_num * f_x) % modulus),
        _ => None
    }
}

fn mod_inverse(n: i128, p: i128) -> Option<i128> {
    for x in 1..p {
        if (n * x) % p == 1 {
            return Some(x);
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(s: &str, exp: &str) -> () {
        let ans = decode(s);
        println!("{:?}", ans == exp);
        assert_eq!(ans, exp, "Testing: {}", s);
    }

    #[test]
    fn basic_tests() {
        let mut s = "10559625hbkeohysnztuuqdznnkkcgjndbujej";
        dotest(s, "dtiygdoenhxqqsfhnniimkpnftqpyp");

        s = "1273409kuqhkoynvvknsdwljantzkpnmfgf";
        dotest(s, "uogbucwnddunktsjfanzlurnyxmx");

        let s = "761328qockcouoqmoayqwmkkic";
        dotest(s, "Impossible to decode");
    }
}

