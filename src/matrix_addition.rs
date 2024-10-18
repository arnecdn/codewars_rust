fn matrix_addition(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    a.iter().zip(b.iter())
        .map(|(z1, z2)|
            z1.iter().zip(z2.iter())
                .map(|(&a, &b)| a + b)
                .collect::<Vec<_>>()
        ).collect()
}

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
// https://www.codewars.com/kata/526233aefd4764272800036f/train/rust
#[cfg(test)]
mod tests {
    use super::matrix_addition;

    #[test]
    fn sample_test1() {
        let matrix_a = vec![vec![1, 2, 3], vec![3, 2, 1], vec![1, 1, 1]];
        let matrix_b = vec![vec![2, 2, 1], vec![3, 2, 3], vec![1, 1, 3]];
        let expected = vec![vec![3, 4, 4], vec![6, 4, 4], vec![2, 2, 4]];
        assert_eq!(matrix_addition(&matrix_a, &matrix_b), expected);
    }

    #[test]
    fn sample_test2() {
        let matrix_a = vec![vec![1]];
        let matrix_b = vec![vec![2]];
        let expected = vec![vec![3]];
        assert_eq!(matrix_addition(&matrix_a, &matrix_b), expected);
    }

    #[test]
    fn sample_test3() {
        let matrix_a = vec![vec![1, 2], vec![1, 2]];
        let matrix_b = vec![vec![2, 3], vec![2, 3]];
        let expected = vec![vec![3, 5], vec![3, 5]];
        assert_eq!(matrix_addition(&matrix_a, &matrix_b), expected);
    }
}


