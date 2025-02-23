
fn parse(code: &str) -> Vec<i32> {
    code.chars().fold((vec![], 0), |mut a, c| -> (Vec<i32>, i32){
        match c {
            'i' => a.1 = a.1 + 1,
            'd' => a.1 = a.1 - 1,
            's' => a.1 = a.1 * a.1,
            'o' => a.0.push(a.1),
            _ => ()
        };
        a
    },
    ).0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_tests() {
        assert_eq!(parse("iiisdoso"),
                   vec![8, 64]);
        // assert_eq!(parse("iiisdoso"),
        //            vec![8, 64]);
        assert_eq!(parse("iiisdosodddddiso"),
                   vec![8, 64, 3600]);
    }
}