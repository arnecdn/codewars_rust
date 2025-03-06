fn king_in_check(chessboard: &[[char; 8]; 8]) -> bool {
    let board_with_pieces = parse_board_with_pieces(chessboard);

    if let Some(king) = parse_king(&board_with_pieces).first() {
        let attakers = parse_attackers(&board_with_pieces);

        return attakers
            .iter()
            .any(|a1| a1.is_king_in_check(&king, &attakers));
    };

    false
}

fn parse_board_with_pieces(chessboard: &[[char; 8]; 8]) -> Vec<(&char, usize, usize)> {
    chessboard
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, c)| (c, j, i))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>()
}

fn parse_king(board_with_pieces: &Vec<(&char, usize, usize)>) -> Vec<WhiteKing> {
    board_with_pieces
        .iter()
        .filter_map(|&(c, j, i)| WhiteKing::king(*c, Position { x: j, y: i }))
        .collect::<Vec<WhiteKing>>()
}

fn parse_attackers(board_with_pieces: &Vec<(&char, usize, usize)>) -> Vec<MovablePiece> {
    board_with_pieces
        .iter()
        .filter_map(|&(c, j, i)| MovablePiece::attacker(*c, Position { x: j, y: i }))
        .collect::<Vec<_>>()
}

#[derive(Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}
struct WhiteKing(Position);

impl WhiteKing {
    fn king(c: char, p: Position) -> Option<WhiteKing> {
        if c == '♔' {
            Some(WhiteKing(p))
        } else {
            None
        }
    }
}
enum MovablePiece {
    Pawn(Position),
    Bishop(Position),
    Rook(Position),
    Knight(Position),
    Queen(Position),
}

impl MovablePiece {
    fn attacker(c: char, p: Position) -> Option<MovablePiece> {
        match c {
            '♟' => Some(MovablePiece::Pawn(p)),
            '♝' => Some(MovablePiece::Bishop(p)),
            '♜' => Some(MovablePiece::Rook(p)),
            '♞' => Some(MovablePiece::Knight(p)),
            '♛' => Some(MovablePiece::Queen(p)),
            _ => None,
        }
    }
    fn position(&self) -> Option<&Position> {
        match self {
            MovablePiece::Bishop(v) => Some(v),
            MovablePiece::Rook(v) => Some(v),
            MovablePiece::Queen(v) => Some(v),
            MovablePiece::Pawn(v) => Some(v),
            MovablePiece::Knight(v) => Some(v),
            _ => None,
        }
    }
    fn find_blockers(&self, attakers: &Vec<MovablePiece>) -> Vec<Position> {
        attakers
            .iter()
            .filter_map(|a| a.position())
            .filter(|&&r| r.x != self.position().unwrap().x || r.y != self.position().unwrap().y)
            .map(|r| *r)
            .collect::<Vec<_>>()
    }

    fn is_king_in_check(&self, target: &WhiteKing, attakers: &Vec<MovablePiece>) -> bool {
        let possible_blockers = self.find_blockers(attakers);

        match self {
            MovablePiece::Pawn(v) => pawn_capture(v, &target.0),
            MovablePiece::Bishop(v) => bishop_capture(v, &possible_blockers, &target.0),
            MovablePiece::Rook(v) => rook_capture(v, &possible_blockers, &target.0),
            MovablePiece::Knight(v) => knight_capture(v, &target.0),
            MovablePiece::Queen(v) => queen_capture(v, &possible_blockers, &target.0),
            _ => panic!("Check-move not implemented!!!"),
        }
    }
}

fn queen_capture(queen: &Position, blockers: &Vec<Position>, king: &Position) -> bool {
    bishop_capture(queen, blockers, king) || rook_capture(queen, blockers, king)
}

fn knight_capture(knight: &Position, king: &Position) -> bool {
    (knight.y).abs_diff(king.y) == 2 && (knight.x).abs_diff(king.x) == 1
        || (knight.y).abs_diff(king.y) == 1 && (knight.x).abs_diff(king.x) == 2
}

fn rook_capture(rook: &Position, blockers: &Vec<Position>, king: &Position) -> bool {
    let no_blocker_in_row = blockers
        .iter()
        .any(|b| b.x == king.x && b.x == rook.x && b.y > rook.y && b.y < king.y)
        == false;
    let no_blocker_in_col = blockers
        .iter()
        .any(|b| b.y == king.y && b.y == rook.y && b.x > rook.x && b.x < king.x)
        == false;
    (rook.x == king.x && no_blocker_in_row) || (rook.y == king.y && no_blocker_in_col)
}

fn bishop_capture(bishop: &Position, blockers: &Vec<Position>, king: &Position) -> bool {
    let diagonal_bishop = bishop.x + bishop.y;
    let diagonal_king = king.y + king.x;
    let diagonal = |a: &Position, b: &Position| a.y + b.x == a.x + b.y;

    let no_blocker_in_diagonal = blockers
        .iter()
        .any(|b| diagonal(bishop,b) && diagonal(king,b))
        == false;

    let anti_diagonal = |a: &Position, b: &Position| a.x + b.y == a.y + b.x;

    let no_blocker_in_anti_diagonal = blockers.iter().peekable().any(|b| {
        anti_diagonal(bishop, b)
            && anti_diagonal(king, b)
            && (bishop.x + bishop.y < b.x + b.y && b.x + b.y < king.x + king.y
                || bishop.x + bishop.y > b.x + b.y && b.x + b.y > king.x + king.y)
    }) == false;

    diagonal_bishop == diagonal_king && no_blocker_in_diagonal
        || anti_diagonal(bishop, king) && no_blocker_in_anti_diagonal
}

fn pawn_capture(pawn: &Position, king: &Position) -> bool {
    (king.y as i32 - pawn.y as i32) == 1 && (king.x.abs_diff(pawn.x) == 1)
}

// Example tests - feel free to play around and experiment with these
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::king_in_check;
    use std::iter;

    const BASE: &str = "|---|---|---|---|---|---|---|---|";

    fn stringify_board(board: &[[char; 8]; 8]) -> String {
        format!(
            "{}\n{}",
            BASE,
            board
                .iter()
                .map(|row| row
                    .iter()
                    .map(|square| format!("| {square} "))
                    .chain(iter::once(format!("|\n{BASE}")))
                    .collect::<String>())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    fn dotest(board: &[[char; 8]; 8], expected: bool) {
        if king_in_check(board) == expected {
            assert!(true)
        } else {
            assert!(
                false,
                "With chessboard\n{}\n\nExpected {} but got {}",
                stringify_board(board),
                expected,
                !expected
            )
        }
    }
    #[test]
    fn random_test() {
        dotest(
            &[
                ['♞', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', '♞', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', '♔', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', '♝', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', '♜', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ],
            false,
        );
    }

    #[test]
    #[ignore]
    fn example_test_check_by_pawn() {
        dotest(
            &[
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', '♔', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', '♟', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ],
            true,
        );
    }

    #[test]
    #[ignore]

    fn example_test_check_by_bishop() {
        dotest(
            &[
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', '♔', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', '♝', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ],
            true,
        );
    }
    #[test]
    #[ignore]
    fn example_test_check_by_rook() {
        dotest(
            &[
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', '♔', ' ', ' ', '♜', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ],
            true,
        );
    }
    #[test]
    #[ignore]
    fn example_test_check_by_knight() {
        dotest(
            &[
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', '♞', ' ', ' ', ' ', ' '],
                [' ', '♔', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ],
            true,
        );
    }
    #[test]
    #[ignore]
    fn example_test_check_by_queen() {
        dotest(
            &[
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', '♛', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', '♔', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ],
            true,
        );
    }
    #[test]
    #[ignore]
    fn example_test_king_alone() {
        dotest(
            &[
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', '♔', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ],
            false,
        );
    }
    #[test]
    #[ignore]
    fn example_test_no_check() {
        dotest(
            &[
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                ['♛', ' ', ' ', '♞', ' ', ' ', ' ', '♛'],
                [' ', ' ', ' ', '♔', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ],
            false,
        );
    }
    #[test]
    #[ignore]
    fn example_test_piece_blocking_another() {
        dotest(
            &[
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', '♝', ' ', ' '],
                [' ', ' ', ' ', ' ', '♜', ' ', ' ', ' '],
                [' ', ' ', ' ', '♔', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
                [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
            ],
            false,
        );
    }
}
