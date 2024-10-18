use num::Integer;

fn dashatize(n: i64) -> String {
     n.abs().to_string().chars().fold((String::new(), false), |mut a: (String, bool), c: char| -> (String, bool){
        let current: i32 = c.to_digit(10).unwrap() as i32;

        if a.0.is_empty() {
            a.0.push(c);
        } else if a.1 && !current.is_odd() {
            a.0.push_str(&format!("{}{}",'-', c));
        } else if a.1 && current.is_odd() {
            a.0.push_str(&format!("{}{}",'-', c));
        } else if !a.1 && current.is_odd() {
            a.0.push_str(&format!("{}{}",'-', c));
        } else {
            a.0.push(c);
        }

        a.1 = current.is_odd();
        a
    }).0
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(dashatize(274), "2-7-4");
        assert_eq!(dashatize(5311), "5-3-1-1");
        assert_eq!(dashatize(86320), "86-3-20");
        assert_eq!(dashatize(974302), "9-7-4-3-02");
    }

    #[test]
    fn weird() {
        assert_eq!(dashatize(0), "0");
        assert_eq!(dashatize(-1), "1");
        assert_eq!(dashatize(-28369), "28-3-6-9");
    }
}
