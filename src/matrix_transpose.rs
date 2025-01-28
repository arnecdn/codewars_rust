fn transpose(matrix: &[Vec<u8>]) -> Vec<Vec<u8>> {
    matrix.iter().fold(vec![], |mut acc, row| -> Vec<Vec<u8>> {
        for (j, &col) in row.iter().enumerate() {
            match acc.get(j) {
                Some(_) => acc.get_mut(j).unwrap().push(col),
                None => acc.push(vec![col])
            }
        }
        acc
    })
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::transpose;

    #[test]
    fn sample_tests() {
        // assert_eq!(transpose(&[vec![1]]), vec![vec![1]]);
        // assert_eq!(transpose(&[vec![1, 2, 3]]), vec![vec![1], vec![2], vec![3]]);
        assert_eq!(transpose(&[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]), vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]);
        assert_eq!(transpose(&[vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1], vec![0, 1, 0], vec![1, 0, 0]]), vec![vec![1, 0, 0, 0, 1], vec![0, 1, 0, 1, 0], vec![0, 0, 1, 0, 0]]);
    }
}
