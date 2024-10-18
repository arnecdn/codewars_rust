// optimize function
// keep original order

fn first_n_smallest(arr: &[i32], n: usize) -> Vec<i32> {
    let mut sorted_arr = arr.iter().collect::<Vec<_>>();
    sorted_arr.sort();
    let mut result = vec![];
    let mut sorted = sorted_arr[..n].to_vec();

    for i in arr {
        if sorted.contains(&i) {
            result.push(*i);
            sorted.remove(sorted.iter().position(|&x| x == i).unwrap());
            if result.len() == n { return result; }
        }
    }
    return vec![];
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    //        [2, 7, -10, 9, -4, -10, 7, 5, -9, -1, 8, 5, -9, -2, 5, -3, 9, 7, -10, -3, 3, -7, 4, -8, 1, -8, -6, 8, -8, 9, -2, -6]
// Left:  [2, -10, -4, -10, 5, -9, -1, 5, -9, -2, 5, -3, -10, -3, 3, -7, 4, -8, 1, -8, -6, -8, -2]
// Right: [2, -10, -4, -10, 5, -9, -1, 5, -9, -2, -3, -10, -3, 3, -7, 4, -8, 1, -8, -6, -8, -2, -6]
    #[test]
    fn test_basic() {
        assert_eq!(first_n_smallest(&[2, 7, -10, 9, -4, -10, 7, 5, -9, -1, 8, 5, -9, -2, 5, -3, 9, 7, -10, -3, 3, -7, 4, -8, 1, -8, -6, 8, -8, 9, -2, -6], 23),
                   [2, -10, -4, -10, 5, -9, -1, 5, -9, -2, -3, -10, -3, 3, -7, 4, -8, 1, -8, -6, -8, -2, -6]);
        // assert_eq!(first_n_smallest(&[1, 2, 3, 4, 5], 3), [1, 2, 3]);
        // assert_eq!(first_n_smallest(&[5, 4, 3, 2, 1], 3), [3, 2, 1]);
        // assert_eq!(first_n_smallest(&[1, 2, 3, 1, 2], 3), [1, 2, 1]);
        // assert_eq!(first_n_smallest(&[1, 2, 3, -4, 0], 3), [1, -4, 0]);
        // assert_eq!(first_n_smallest(&[1, 2, 3, 4, 5], 0), []);
        // assert_eq!(first_n_smallest(&[1, 2, 3, 4, 5], 5), [1, 2, 3, 4, 5]);
        // assert_eq!(first_n_smallest(&[1, 2, 3, 4, 2], 4), [1, 2, 3, 2]);
        // assert_eq!(first_n_smallest(&[2, 1, 2, 3, 4, 2], 2), [2, 1]);
        // assert_eq!(first_n_smallest(&[2, 1, 2, 3, 4, 2], 3), [2, 1, 2]);
        // assert_eq!(first_n_smallest(&[2, 1, 2, 3, 4, 2], 4), [2, 1, 2, 2]);
    }
}
