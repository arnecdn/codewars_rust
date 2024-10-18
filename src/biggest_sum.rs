
fn find_sum(matrix: &[Vec<u32>]) -> u32 {
    fn find_sum_rec(matrix: &[Vec<u32>], current: (usize, usize), mut path: Vec<u32>) -> u32 {
        path.push(matrix[current.0][current.1]);

        let down_is_larger = || -> bool { matrix[current.0 + 1][current.1] > matrix[current.0][current.1 + 1] };
        let can_move_down = || -> bool{ current.0 < matrix.len() - 1 };
        let can_move_right = || -> bool{ current.1 < matrix[current.0].len() - 1 };

        let should_move_down = || -> bool { can_move_down() && !can_move_right() || can_move_down() && can_move_right() && down_is_larger() };
        let should_move_right = || -> bool { can_move_right() };

        if should_move_down() {
            return find_sum_rec(matrix, (current.0 + 1, current.1), path);
        } else if should_move_right() {
            return find_sum_rec(matrix, (current.0, current.1 + 1), path);
        }
        path.iter().for_each(|f| println!("{}", f));
        path.iter().sum()
    }
    find_sum_rec(matrix, (0, 0), vec![])
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::find_sum;

    fn dotest(matrix: &[Vec<u32>], expected: u32) {
        let actual = find_sum(matrix);
        assert!(actual == expected,
                "With matrix = {matrix:?}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn example_test() {
        dotest(
            &[vec![20, 20, 10, 10],
            vec![10, 20, 10, 10],
            vec![10, 20, 20, 20],
            vec![10, 10, 10, 20]], 140);
    }


}
