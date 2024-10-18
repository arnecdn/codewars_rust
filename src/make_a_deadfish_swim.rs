fn parse_imp(code: &str) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut b = 0;

    for c in code.chars() {
        b = match c {
            'i' => b + 1,
            'd' => b - 1,
            's' => b * b,
            'o' => {
                result.push(b);
                b
            }
            _ => b
        };
    }
    result
}

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