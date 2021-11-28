/*
    a           1       1
    aa          3       2
    aaa         6       3
    aaaa        10      4
    aaaaa       15      5
    aaaaaa      21      6
*/
fn pyramid_(balls: u16) -> u16 {
    let mut levels = 1;
    let mut balls_in_pyramid = 1;

    while (balls_in_pyramid + levels) < balls {
        levels += 1;
        balls_in_pyramid += levels;
    }
    return levels;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(pyramid(1), 1);
        assert_eq!(pyramid(4), 2);
        assert_eq!(pyramid(20), 5);
        assert_eq!(pyramid(100), 13);
        assert_eq!(pyramid(2211), 66);
        assert_eq!(pyramid(9999), 140);
    }
}
