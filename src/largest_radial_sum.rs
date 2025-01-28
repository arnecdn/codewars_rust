use std::cmp::max;

fn largest_radial_sum(arr: &[i32], d: u32) -> i32 {
    if arr.len() % d as usize != 0 || d > 31 || arr.len() == 0 {
        return 0;
    }

    fn rec_radual_sum<'a>(arr: &[i32], mut largest: i32, day_of_month: u32,  current: u32) -> i32 {
        let mut honored_sum = 0;
        let mut i = current as usize;
        let mut honoured_seat = 0;
        let seat_distance = arr.len() / day_of_month as usize;

        while honoured_seat < day_of_month && i < arr.len() {
            honored_sum += arr[i];
            i += seat_distance;
            honoured_seat += 1;
        }

        if honoured_seat < day_of_month {
            return largest;
        }

        largest = max(honored_sum, largest);

        rec_radual_sum(arr, largest, day_of_month, current + 1)
    }

    rec_radual_sum(arr, -1000, d, 0)
}


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::largest_radial_sum;

    fn dotest(arr: &[i32], d: u32, expected: i32) {
        let actual = largest_radial_sum(arr, d);
        assert!(actual == expected,
                "With arr = {arr:?}, d = {d}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn basic_tests() {
        for (arr, d, expected) in
        [
            (&vec![1, 2, 3, 4], 2, 6),
            (&vec![1, 5, 6, 3, 4, 2], 3, 11),
            (&vec![1, 1, 0], 1, 1),
            (&vec![9, 10, 2], 3, 21),
            (&vec![-2, -2, -1, -2], 2, -3),
            (&vec![6, -3, 1, 6, 8, 4, -4, 9, 8, 2, 5, 0, -9, -6, 6, -3, -9, -7, 9, 5, -9, 0, 4, -2, -7, 7], 13, 12)
        ] {
            dotest(arr, d, expected)
        }
    }
}