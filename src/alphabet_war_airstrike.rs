use std::cmp::Ordering;
use regex::Regex;
use std::collections::HashMap;

fn alphabet_war(fight: &str) -> &'static str {
    let left_power = HashMap::from([('w', 4), ('p', 3), ('b', 2), ('s', 1)]);
    let right_power = HashMap::from([('m', 4), ('q', 3), ('d', 2), ('z', 1)]);
    let bomb = Regex::new(r"(.?[*]+.?)").unwrap();
    let survivors = bomb.replace_all(fight, "").to_lowercase();

    let (left_sum, right_sum) = survivors.chars().fold((0, 0), |acc, c| {
        if let Some(v) = left_power.get(&c) {
            (acc.0 + v, acc.1)
        } else if let Some(v) = right_power.get(&c) {
            (acc.0, acc.1 + v)
        } else {
            acc
        }
    });
    match left_sum.cmp(&right_sum){
        Ordering::Greater => "Left side wins!",
        Ordering::Equal => "Let's fight again!",
        Ordering::Less => "Right side wins!"
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::alphabet_war;

    fn dotest(s: &str, expected: &'static str) {
        let actual = alphabet_war(s);
        assert!(
            actual == expected,
            "With fight = \"{s}\"\nExpected \"{expected}\" but got \"{actual}\""
        )
    }

    #[test]
    fn fixed_tests() {
        dotest("z", "Right side wins!");
        dotest("****", "Let's fight again!");
        dotest("z*dq*mw*pb*s", "Let's fight again!");
        dotest("zdqmwpbs", "Let's fight again!");
        dotest("zz*zzs", "Right side wins!");
        dotest("sz**z**zs", "Left side wins!");
        dotest("z*z*z*zs", "Left side wins!");
        dotest("*wwwwww*z*", "Left side wins!");
        dotest("w****z", "Let's fight again!");
        dotest("mb**qwwp**dm", "Let's fight again!");
        dotest("*wwzsg*dqdsmz***qzsa", "Right side wins!");
    }
}
