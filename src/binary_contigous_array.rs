use std::cmp::max;
use std::collections::HashMap;

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::binarray;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(a: &[u8], expected: u32) {
        assert_eq!(binarray(a), expected, "{ERR_MSG} with a= {a:?}")
    }

    #[test]
    fn fixed_tests() {
        for (input, expected) in [
            // (vec![0, 1], 2),
            // (vec![0], 0),
            (vec![1, 1, 0, 1, 1, 0, 1, 1], 4),
            (vec![0, 1, 1, 0, 1, 1, 1, 0, 0, 0], 10),
            (vec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0], 6),
            (vec![0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1], 12)
            // (vec![0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 1], 12)
        ] {
            dotest(&input, expected);
        }
    }
}

fn binarray(a: &[u8]) -> u32 {
    let mut max_subarray = 0;
    let mut index_by_sum = HashMap::new();
    let mut current_sum: i32 = 0;

    for (i, &v) in a.iter().enumerate() {
        current_sum += match v {
            0 => -1,
            _ => 1
        };

        if let Some(previous_index) = index_by_sum.get(&current_sum) {
            max_subarray = max(i - previous_index, max_subarray);
        } else if current_sum == 0 {
            max_subarray = i + 1;
        } else {
            index_by_sum.insert(current_sum, i);
        }
    }

    max_subarray as u32
}

/*
[0] = {(i32, usize)}
 0 = {i32} 2
 1 = {usize} 1
[1] = {(i32, usize)}
 0 = {i32} 1
 1 = {usize} 0
[2] = {(i32, usize)}
 0 = {i32} 4
 1 = {usize} 7
[3] = {(i32, usize)}
 0 = {i32} 3
 1 = {usize} 4


 min
 [0] = {(i8, usize)}
 0 = {i8} 1
 1 = {usize} 2
[1] = {(i8, usize)}
 0 = {i8} 2
 1 = {usize} 5
[2] = {(i8, usize)}
 0 = {i8} 4
 1 = {usize} 7
[3] = {(i8, usize)}
 0 = {i8} 3
 1 = {usize} 6
*/
