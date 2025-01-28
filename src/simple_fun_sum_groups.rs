use num::Integer;


fn sum_groups(arr: &[u32]) -> usize {
    let mut res: Vec<u32> = vec![];

    for (i, e) in arr.iter().enumerate() {
        if res.len() > 0 && e.is_even() == arr[i - 1].is_even() {
            *res.last_mut().unwrap() += e;
        } else {
            res.push(*e);
        }
    }

    if arr.len() != res.len() {
        return sum_groups(&res);
    }
    res.len()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::sum_groups;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(a: &[u32], expected: usize) {
        assert_eq!(sum_groups(a), expected, "{ERR_MSG} with arr = {a:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest(&[2, 1, 2, 2, 6, 5, 0, 2, 0, 5, 5, 7, 7, 4, 3, 3, 9], 6);
        dotest(&[2, 1, 2, 2, 6, 5, 0, 2, 0, 3, 3, 3, 9, 2], 5);
        dotest(&[2], 1);
        dotest(&[1, 2], 2);
        dotest(&[1, 1, 2, 2], 1);
    }
}
