use std::io::Read;

fn compare_versions(version1: &str, version2: &str) -> bool {
    let v_1 = version1.split(".").map(|e|e.parse::<u8>().unwrap()).collect::<Vec<_>>();
    let v_2 = version2.split(".").map(|e|e.parse::<u8>().unwrap()).collect::<Vec<_>>();
    v_1 >= v_2
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::compare_versions;

    fn dotest(v1: &str, v2: &str, expected: bool) {
        let actual = compare_versions(v1, v2);
        assert!(actual == expected, "With version1 = \"{v1}\", version2 = \"{v2}\"\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest("11", "10", true);
        dotest("11", "11", true);
        dotest("10.4.6", "10.4", true);
        dotest("10.4", "10.4.8", false);
        dotest("10.4", "11", false);
        dotest("10.4.9", "10.5", false);
        dotest("4.3.3", "4.3.3.1", false);
        dotest("10.4.9", "10.5", false);
        dotest("10.4.9", "104.9", false);
        dotest("10.15", "10.12", true);
    }
}