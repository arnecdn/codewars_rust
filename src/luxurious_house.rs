fn luxhouse(houses: &[u32]) -> Vec<u32> {
    (0..houses.len()).fold(vec![], |mut acc, i| {
        if let Some(max) = houses[i + 1..].iter().max_by(|a, b| a.partial_cmp(b).unwrap()).filter(|&&a| a >= houses[i]) {
            let floors = &max.abs_diff(houses[i]) + 1;
            acc.push(floors)
        } else {
            acc.push(0);
        }
        acc
    },
    )
}



// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::luxhouse;

    fn dotest(a: &[u32], expected: &[u32]) {
        let actual = luxhouse(a);
        assert!(actual == expected,
                "With houses = {a:?}\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&[1, 2, 3, 1, 2], &[3, 2, 0, 2, 0]);
        dotest(&[3, 2, 1, 4], &[2, 3, 4, 0]);
        dotest(&[1, 2, 3], &[3, 2, 0]);
        dotest(&[3, 2, 1], &[0, 0, 0]);
        dotest(&[1, 1, 1], &[1, 1, 0]);
    }
}
