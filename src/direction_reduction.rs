#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    West,
    South,
}

fn is_opposite(left: &Direction, right: &Direction) -> bool {
    match (left, right) {
        (Direction::South, Direction::North) => true,
        (Direction::North, Direction::South) => true,
        (Direction::West, Direction::East) => true,
        (Direction::East, Direction::West) => true,
        _ => false
    }
}

fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    arr.iter().fold(vec![], |mut acc, c| {
        if !acc.is_empty() && is_opposite(&acc.last().unwrap(), c) {
            acc.pop();
        } else {
            acc.push(*c);
        }
        acc
    },
    )
}


#[cfg(test)]
mod tests {
    use super::{dir_reduc, Direction::{*}};

    #[test]
    fn basic() {
        let a = [North, South, South, East, West, North, West];
        assert_eq!(dir_reduc(&a), [West]);

        let a = [North, West, South, East];
        assert_eq!(dir_reduc(&a), [North, West, South, East]);
    }
}