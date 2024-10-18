fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
    where
        T: std::iter::IntoIterator,
        T::Item: std::cmp::PartialEq + std::fmt::Debug,
{

    sequence.into_iter(). fold(vec![], |mut h, e| {
        if h.last() != Some(&e) {
            h.push(e)
        }
        h
    })


}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
// https://www.codewars.com/kata/54e6533c92449cc251001667/train/rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(unique_in_order("AAAABBBCCDAABBB".chars()), vec!['A', 'B', 'C', 'D', 'A', 'B']);
    }
}