fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn minimum_number(xs: &[u32]) -> u32 {
    let mut sum: u32 = xs.iter().sum();
    while is_prime(sum) == false {
        sum = sum + 1;
    }

    sum - xs.iter().sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime() {
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(5), true);
    }
    #[test]
    fn basic() {
        assert_eq!(minimum_number(&[3, 1, 2]), 1);
        assert_eq!(minimum_number(&[5, 2]), 0);
        assert_eq!(minimum_number(&[1, 1, 1]), 0);
        assert_eq!(minimum_number(&[2, 12, 8, 4, 6]), 5);
        assert_eq!(minimum_number(&[50, 39, 49, 6, 17, 28]), 2);
    }
}
