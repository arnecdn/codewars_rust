fn flip(direction: char, matrix: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    match direction {
        'R' => flip_right(matrix),
        'L' => flip_left(matrix),
        'U' => flip_up(matrix),
        'D' => flip_down(matrix),
        _=> vec![vec![]]
    }
}

fn flip_down(mut matrix: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let num_cols = matrix.len();
    let num_rows = matrix[0].len();
    let mut rotated_matrix = rotate_matrix(&mut matrix, num_cols, num_rows);

    for mut row in &mut rotated_matrix{
        &row.sort();
    }

    rotate_matrix(&mut rotated_matrix, num_rows, num_cols)
}

fn flip_up(mut matrix: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let num_cols = matrix.len();
    let num_rows = matrix[0].len();
    let mut rotated_matrix = rotate_matrix(&mut matrix, num_cols, num_rows);

    for mut row in &mut rotated_matrix{
        &row.sort();
        &row.reverse();
    }

    rotate_matrix(&mut rotated_matrix, num_rows, num_cols)
}

fn rotate_matrix(matrix: &mut Vec<Vec<u32>>, num_cols: usize, num_rows: usize) -> Vec<Vec<u32>> {
    let mut rotated_matrix = vec![vec![0u32; num_cols]; num_rows];

    for (i, row) in matrix.iter().enumerate() {
        for (j, &col) in row.iter().enumerate() {
            rotated_matrix[j][i] = col;
        }
    }
    rotated_matrix
}

fn flip_left(mut matrix: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    matrix.iter_mut().fold(vec![], |mut acc,  r|{
        r.sort();
        r.reverse();
        acc.push(r.to_vec());
        acc
    })
}

fn flip_right(mut matrix: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    matrix.iter_mut().fold(vec![], |mut acc,  r|{
        r.sort();
        acc.push(r.to_vec());
        acc
    })
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip() {
        let input = vec![
                            vec![1, 3, 2],
                            vec![4, 5, 1],
                            vec![6, 5, 3],
                            vec![7, 2, 9]];

        assert_eq!(flip('R', input.clone()),
                   vec![vec![1, 2, 3],
                        vec![1, 4, 5],
                        vec![3, 5, 6],
                        vec![2, 7, 9]]);

        assert_eq!(flip('L', input.clone()),
                   vec![vec![3, 2, 1],
                        vec![5, 4, 1],
                        vec![6, 5, 3],
                        vec![9, 7, 2]]);

        assert_eq!(flip('U', input.clone()),
                   vec![vec![7, 5, 9],
                        vec![6, 5, 3],
                        vec![4, 3, 2],
                        vec![1, 2, 1]]);

        assert_eq!(flip('D', input),
                   vec![vec![1, 2, 1],
                        vec![4, 3, 2],
                        vec![6, 5, 3],
                        vec![7, 5, 9]]);
    }
}
