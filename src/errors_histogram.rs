fn hist(s: &str) -> String {
    "uwxz".chars().fold(vec![], |mut acc, e| {
        let error_count = s.chars().filter(|&c| c == e).count();
        if error_count > 0 {
            acc.push(format!("{:3}{:<6}{}", e, error_count, "*".repeat(error_count)))
        }
        acc
    },
    ).join("\r")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(s: &str, exp: &str) -> () {
        println!("s:{:?}", s);
        let ans = hist(s);
        println!("actual: {:?}", ans);
        println!("expect: {:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        dotest("tpwaemuqxdmwqbqrjbeosjnejqorxdozsxnrgpgqkeihqwybzyymqeazfkyiucesxwutgszbenzvgxibxrlvmzihcb",
               "u  3     ***\rw  4     ****\rx  6     ******\rz  6     ******");
        dotest("aaifzlnderpeurcuqjqeywdq", "u  2     **\rw  1     *\rz  1     *");
    }
}
