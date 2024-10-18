// Write a program that will calculate the number of trailing zeros in a factorial of a given number.
// example zeros(6) = 1
// reformat the code


fn zeros(n: u64) -> u64 {
    let mut num_of_zeros = 0;
    let mut num = n;
    while num > 0 {
        num /= 5;
        num_of_zeros += num;
    }
    return num_of_zeros;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(zeros(0), 0);
        assert_eq!(zeros(6), 1);
        assert_eq!(zeros(14), 2);
        assert_eq!(zeros(30), 7);
        assert_eq!(zeros(1000), 249);
        assert_eq!(zeros(100000), 24999);
        assert_eq!(zeros(1000000000), 249999998);
    }
}