#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    King,
    Rook,
    Bishop,
    Gold,
    Silver,
    Knight,
    Lance,
    Pawn,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Black,
    White,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    piece_type: PieceType,
    color: Color,
}

pub type Board = [[Option<Piece>; 9]; 9];

pub fn parse_piece(c: char) -> Option<Piece> {
    let piece_type = match c.to_ascii_lowercase() {
        'k' => PieceType::King,
        'r' => PieceType::Rook,
        'b' => PieceType::Bishop,
        'g' => PieceType::Gold,
        's' => PieceType::Silver,
        'n' => PieceType::Knight,
        'l' => PieceType::Lance,
        'p' => PieceType::Pawn,
        _ => return None,
    };

    let color = if c.is_lowercase() {
        Color::Black
    } else {
        Color::White
    };

    Some(Piece {
        piece_type,
        color,
    })
}

pub fn parse_board(afen: &str) -> Board {
    let mut board = [[None; 9]; 9];

    let mut rank = 0;
    let mut file = 0;

    for c in afen.chars() {
        match c {
            '/' => {
                rank += 1;
                file = 0;
            }
            '1'..='9' => {
                let num_spaces = c.to_digit(10).unwrap() as usize;
                file += num_spaces;
            }
            _ => {
                if let Some(piece) = parse_piece(c) {
                    board[rank][file] = Some(piece);
                }
                file += 1;
            }
        }
    }

    board
}

pub fn count_attacking_pieces(board: &Board, king_color: Color) -> usize {
    let opponent_color = match king_color {
        Color::Black => Color::White,
        Color::White => Color::Black,
    };

    let dx = [-1,  0,  1, -1, 1, -1, 0, 1];
    let dy = [-1, -1, -1,  0, 0,  1, 1, 1];

    let mut count = 0;

    // Find the king's position
    let mut king_pos: Option<(usize, usize)> = None;

    for i in 0..9 {
        for j in 0..9 {
            if let Some(piece) = board[i][j] {
                if piece.piece_type == PieceType::King && piece.color == king_color {
                    king_pos = Some((i, j));
                    break;
                }
            }
        }

        if king_pos.is_some() {
            break;
        }
    }

    if let Some((king_i, king_j)) = king_pos {
        for i in 0..9 {
            for j in 0..9 {
                if let Some(piece) = board[i][j] {
                    if piece.color == opponent_color {
                        let piece_type = piece.piece_type;

                        match piece_type {
                            PieceType::King => {
                                for k in 0..8 {
                                    let ni = i as isize + dx[k];
                                    let nj = j as isize + dy[k];

                                    if ni >= 0 && ni < 9 && nj >= 0 && nj < 9 {
                                        if (ni as usize, nj as usize) == (king_i, king_j) {
                                            count += 1;
                                        }
                                    }
                                }
                            }
                            PieceType::Rook => {
                                for k in 0..4 {
                                    let mut ni = i as isize + dx[k];
                                    let mut nj = j as isize + dy[k];

                                    while ni >= 0 && ni < 9 && nj >= 0 && nj < 9 {
                                        if (ni as usize, nj as usize) == (king_i, king_j) {
                                            count += 1;
                                            break;
                                        } else if board[ni as usize][nj as usize].is_some() {
                                            break;
                                        }

                                        ni += dx[k];
                                        nj += dy[k];
                                    }
                                }
                            }
                            PieceType::Bishop => {
                                for k in 4..8 {
                                    let mut ni = i as isize + dx[k];
                                    let mut nj = j as isize + dy[k];

                                    while ni >= 0 && ni < 9 && nj >= 0 && nj < 9 {
                                        if (ni as usize, nj as usize) == (king_i, king_j) {
                                            count += 1;
                                            break;
                                        } else if board[ni as usize][nj as usize].is_some() {
                                            break;
                                        }

                                        ni += dx[k];
                                        nj += dy[k];
                                    }
                                }
                            }
                            PieceType::Gold => {
                                for k in 0..8 {
                                    let ni = i as isize + dx[k];
                                    let nj = j as isize + dy[k];

                                    if ni >= 0 && ni < 9 && nj >= 0 && nj < 9 {
                                        if (ni as usize, nj as usize) == (king_i, king_j) {
                                            count += 1;
                                        }
                                    }
                                }
                            }
                            PieceType::Silver => {
                                let ni = i as isize - 1;
                                let nj = j as isize - 1;

                                if ni >= 0 && nj >= 0 && (ni as usize, nj as usize) == (king_i, king_j) {
                                    count += 1;
                                }

                                let ni = i as isize - 1;
                                let nj = j as isize + 1;

                                if ni >= 0 && nj < 9 && (ni as usize, nj as usize) == (king_i, king_j) {
                                    count += 1;
                                }

                                let ni = i as isize;
                                let nj = j as isize - 1;

                                if nj >= 0 && (ni as usize, nj as usize) == (king_i, king_j) {
                                    count += 1;
                                }

                                let ni = i as isize;
                                let nj = j as isize + 1;

                                if nj < 9 && (ni as usize, nj as usize) == (king_i, king_j) {
                                    count += 1;
                                }
                            }
                            PieceType::Knight => {
                                let ni = i as isize - 2;
                                let nj = j as isize - 1;

                                if ni >= 0 && nj >= 0 && (ni as usize, nj as usize) == (king_i, king_j) {
                                    count += 1;
                                }

                                let ni = i as isize - 2;
                                let nj = j as isize + 1;

                                if ni >= 0 && nj < 9 && (ni as usize, nj as usize) == (king_i, king_j) {
                                    count += 1;
                                }
                            }
                            PieceType::Lance => {
                                let ni = i as isize - 1;
                                let nj = j as isize;

                                if ni >= 0 && (ni as usize, nj as usize) == (king_i, king_j) {
                                    count += 1;
                                }
                            }
                            PieceType::Pawn => {
                                let ni = i as isize - 1;
                                let nj = j as isize;

                                if ni >= 0 && (ni as usize, nj as usize) == (king_i, king_j) {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    count
}

