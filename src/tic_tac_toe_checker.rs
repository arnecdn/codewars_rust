fn is_solved(board: &[&[u8; 3]; 3]) -> i8 {
    let mut flattened = flatten(board);
    let mut flattened_diagonal = flatten_diagonal(board);

    flattened.append(&mut flattened_diagonal);

    let possible_winners = flattened.iter().filter(|f| f[0] != 0 && f.iter().min() == f.iter().max()).collect::<Vec<_>>();

    match &possible_winners[..] {
        [[1, ..], [2, ..]] => return 0,
        [[2, ..], [1, ..]] => return 0,
        [[1, ..],..] => return 1,
        [[2, ..],..] => return 2,
        _ => if flattened.iter().any(|f| f.contains(&0)) { return -1; } else { return 0; }
    };
}

fn flatten(board: &[&[u8; 3]; 3]) -> Vec<[i32; 3]> {
    let mut flattened = vec![];

    for i in 0..3 {
        let mut rows: [i32; 3] = [0; 3];
        let mut cols: [i32; 3] = [0; 3];

        for j in 0..3 {
            rows[j] = board[i][j] as i32;
            cols[j] = board[j][i] as i32;
        }

        flattened.push(rows);
        flattened.push(cols);
    }
    flattened
}

fn flatten_diagonal(board: &[&[u8; 3]; 3]) -> Vec<[i32; 3]> {
    let mut diagonal_top_left_bottom_right: [i32; 3] = [0; 3];
    let mut diagonal_top_right_bottom_left: [i32; 3] = [0; 3];

    for i in 0..3 {
        diagonal_top_left_bottom_right[i] = board[i][i] as i32;
        diagonal_top_right_bottom_left[i] = board[i][2 - i] as i32;
    }

    let mut flattened_diagonal = vec![];
    flattened_diagonal.push(diagonal_top_left_bottom_right);
    flattened_diagonal.push(diagonal_top_right_bottom_left);
    flattened_diagonal
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::is_solved;

    fn dotest(board: &[&[u8; 3]; 3], expected: i8) {
        let actual = is_solved(board);
        assert!(actual == expected, "With board = {:?}\nExpected {expected} but got {}", board, actual)
    }

    #[test]
    fn fixed_tests() {
        for (board, expected) in [
            ([&[0, 0, 1], &[0, 1, 2], &[2, 1, 0]], -1),
            ([&[1, 0, 1], &[0, 1, 2], &[1, 0, 0]], 1),
            ([&[1, 1, 1], &[0, 2, 2], &[0, 0, 0]], 1),
            ([&[2, 1, 2], &[2, 1, 1], &[1, 1, 2]], 1),
            ([&[2, 1, 2], &[2, 1, 1], &[1, 2, 1]], 0),
            ([&[0, 0, 0], &[1, 1, 1],& [2, 2, 0]], 1),
            ([&[2, 2, 2], &[1, 1, 1], &[2, 2, 0]], 0),
            ([&[1, 2, 2], &[1, 1, 1], &[2, 2, 1]], 1),
        ] {
            dotest(&board, expected);
        }
    }
}