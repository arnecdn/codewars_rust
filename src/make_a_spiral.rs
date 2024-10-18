const FRAME: i8 = 0;
const SPIRAL_PATTERN: i8 = 1;


fn spiralize(size: usize) -> Vec<Vec<i8>> {
    fn make_spiral(mut start: (usize, usize), mut spiral: Vec<Vec<i8>>, rotations: usize) -> Vec<Vec<i8>> {
        if rotations >= spiral.len() {
            return spiral;
        }

        for i in start.1..spiral.len() {
            spiral[start.0][i] = SPIRAL_PATTERN;
            if i < spiral[start.0].len() - 2 && spiral[start.0][i + 2] == 1 {
                if start.0 > start.1 {
                    start.1 = start.0;
                }
                start.0 = spiral.len() - i - 1;

                break;
            }
        }

        let rotated_spiral = rotate_90_left(&spiral);

        return rotate_90_right(&make_spiral(start, rotated_spiral, rotations + 1));
    }
    return make_spiral((0, 0), vec![vec![FRAME; size]; size], 0);
}

fn rotate_90_right(spiral: &Vec<Vec<i8>>) -> Vec<Vec<i8>> {
    let mut rotated_spiral: Vec<Vec<_>> = vec![vec![0; spiral.len()]; spiral.len()];

    for i in 0..spiral.len() {
        for j in 0..spiral[i].len() {
            rotated_spiral[i][j] = spiral[spiral.len() - j - 1][i];
        }
    }
    rotated_spiral
}

fn rotate_90_left(spiral: &Vec<Vec<i8>>) -> Vec<Vec<i8>> {
    let mut rotated_spiral: Vec<Vec<_>> = vec![vec![0; spiral.len()]; spiral.len()];

    for i in 0..spiral.len() {
        for j in 0..spiral[i].len() {
            rotated_spiral[i][j] = spiral[j][spiral.len() - i - 1];
        }
    }
    rotated_spiral
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test5() {
        assert_eq!(
            spiralize(5),
            [
                [1, 1, 1, 1, 1],
                [0, 0, 0, 0, 1],
                [1, 1, 1, 0, 1],
                [1, 0, 0, 0, 1],
                [1, 1, 1, 1, 1],
            ],
        );
    }

    #[test]
    fn test8() {
        assert_eq!(
            spiralize(8),
            [
                [1, 1, 1, 1, 1, 1, 1, 1],
                [0, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 1, 0, 1],
                [1, 0, 1, 0, 0, 1, 0, 1],
                [1, 0, 1, 1, 1, 1, 0, 1],
                [1, 0, 0, 0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1, 1, 1, 1],
            ],
        );
        //     [
        // [1, 1, 1, 1, 1, 1, 1, 1],
        // [1, 0, 0, 0, 0, 0, 0, 1],
        // [1, 0, 1, 1, 1, 1, 0, 1],
        // [1, 0, 1, 1, 0, 0, 0, 1],
        // [1, 0, 1, 1, 0, 0, 0, 1],
        // [1, 0, 1, 1, 0, 0, 0, 1],
        // [1, 0, 1, 0, 0, 0, 0, 1],
        // [1, 0, 1, 1, 1, 1, 1, 1]]
    }
}


