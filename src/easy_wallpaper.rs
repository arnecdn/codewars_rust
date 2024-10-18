fn wall_paper(l: f64, w: f64, h: f64) -> String {
    let rolls_as_text = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
        "twenty",
    ];
    let g = (w * h * l) as i8;
    match g {
        0 => rolls_as_text[0].to_string(),
        _ => rolls_as_text[((l * h * 2.0 + w * h * 2.0) / 0.52 / 10.0 * 1.15).ceil() as usize]
            .to_string(),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(l: f64, w: f64, h: f64, exp: &str) -> () {
        println!("l: {:?}", l);
        println!("w: {:?}", w);
        println!("h: {:?}", h);
        let ans = wall_paper(l, w, h);
        println!("actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }
    #[test]
    fn basic_tests() {
        dotest(6.3, 4.5, 3.29, "sixteen");
        dotest(6.3, 5.8, 3.13, "seventeen");
        dotest(6.1, 2.0, 3.15, "twelve");
    }
}
