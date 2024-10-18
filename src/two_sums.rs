fn two_sum(numbers: &[i32], target: i32) -> (usize, usize) {
    for (i, iv) in numbers.iter().enumerate() {
        let from = i + 1;

        for (j, jv) in numbers[from..].iter().enumerate() {
            if (iv + jv) == target {
                return (i, j + from);
            }
        }
    }
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn sample() {
        do_test(&[1, 2, 3], 4);
        do_test(&[1234, 5678, 9012], 14690);
        do_test(&[2, 2, 3], 4);
    }

    fn do_test(nums: &[i32], sum: i32) {
        let len = nums.len();
        let user_tuple = two_sum(nums, sum);
        assert!(
            user_tuple.0 < len && user_tuple.1 < len,
            "\nnumbers: {:?}\ntarget: {}\nresult: {:?}\nresult tuple has an index out of bounds",
            nums, sum, user_tuple
        );
        assert!(
            user_tuple.0 != user_tuple.1,
            "\nnumbers: {:?}\ntarget: {}\nresult: {:?}\nresult tuple must have two different indices",
            nums, sum, user_tuple
        );
        let num1 = nums[user_tuple.0];
        let num2 = nums[user_tuple.1];
        let user_sum = num1 + num2;
        assert!(
            user_sum == sum,
            "\nnumbers: {:?}\ntarget: {}\nresult: {:?}\nnumber as index {}: {}\nnumber as index {}: {}\nsum of the two numbers: {}\nsum of the two numbers did not equal target",
            nums, sum, user_tuple, user_tuple.0, num1, user_tuple.1, num2, user_sum
        )
    }
}