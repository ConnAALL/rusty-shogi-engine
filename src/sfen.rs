/* Russell Kosovsky
 * Converts an sfen string into a board position that can be used for the search function
 */

use shogi_core::PartialPosition;
use shogi_core::{Square, Piece, Color, PieceKind};


pub fn flip(sfen: &str) -> String {
    
    let mut split: Vec<&str> = sfen.split_whitespace().collect();
    let board = split[0];
    let flip: String = board.chars().rev().collect();
    split[0] = &flip;
    let joined = split.join(" ");
    
    joined
}


pub fn flip_case(sfen: &str) -> String {
    let flipped_chars: String = sfen.chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                c.to_ascii_lowercase()
            } else if c.is_ascii_lowercase() {
                c.to_ascii_uppercase()
            } else {
                c
            }
        })
        .collect();

    flipped_chars
}


pub fn convert_promoted_pieces(sfen: &str) -> String {
    let mut result = String::new();
    let mut skip_next = false;
    let mut index = 0;
    
    for c in sfen.chars() {
        if skip_next {
            skip_next = false;
            index += 1;
            continue;
        }
        
        match c {
            '+' => {
                let next_char = sfen.chars().nth(index + 1);
                if let Some(promoted_piece) = next_char {
                    let unique_char = match promoted_piece {
                        'P' => 'Z',
                        'L' => 'X',
                        'N' => 'Y',
                        'S' => 'Q',
                        'B' => 'W',
                        'R' => 'E',
                        'p' => 'z',
                        'l' => 'x',
                        'n' => 'y',
                        's' => 'q',
                        'b' => 'w',
                        'r' => 'e',
                        _ => promoted_piece,
                    };
                    result.push(unique_char);
                    skip_next = true;
                }
            },
            _ => result.push(c),
        }

        index+= 1;
    }
    
    result
}


pub fn sfen_parse(dirty_sfen: &str) -> Vec<(String, String)> {

    let sfen = convert_promoted_pieces(dirty_sfen);

    let _pieces: Vec<char> = vec![
        'P', 'L', 'N', 'S', 'G', 'B', 'R', 'K',
        'p', 'l', 'n', 's', 'g', 'b', 'r', 'k',
    ];

    let ranks: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'];
    let sfen_parts: Vec<&str> = sfen.split_whitespace().collect();

    let board = sfen_parts[0];

    let mut board_state: Vec<Vec<String>> = vec![vec!["  ".to_string(); 9]; 9];

    let mut file = 9;
    let mut rank = 0;

    for c in board.chars() {
        match c {
            '/' => {
                file = 9;
                rank += 1;
            }
            '1'..='9' => {
                let empty_squares: usize = c.to_digit(10).unwrap() as usize;
                for _ in 0..empty_squares {
                    file -= 1;
                    board_state[rank][file] = "   ".to_string();
                }
            }
            _ => {
                file -= 1;
                board_state[rank][file] = format!("{}{}", sfen_to_color(c), sfen_to_piece(c));
            }
        }
    }

    let mut result: Vec<(String, String)> = Vec::new();

    for file in 0..9 {
        for rank in (0..9).rev() {
            let square = format!("SQ_{}{}", rank + 1, ranks[file]).to_uppercase();
            let piece = board_state[file][rank].to_uppercase();
            result.push((square, piece));
        }
    }

    result
}


fn sfen_to_piece(c: char) -> &'static str {
    match c {
        'P' => "P",
        'L' => "L",
        'N' => "N",
        'S' => "S",
        'G' => "G",
        'B' => "B",
        'R' => "R",
        'K' => "K",
        
        'Z' => "PP",
        'X' => "PL",
        'Y' => "PN",
        'Q' => "PS",
        'W' => "PB",
        'E' => "PR",

        'p' => "p",
        'l' => "l",
        'n' => "n",
        's' => "s",
        'g' => "g",
        'b' => "b",
        'r' => "r",
        'k' => "k",

        'z' => "pp",
        'x' => "pl",
        'y' => "pn",
        'q' => "ps",
        'w' => "pb",
        'e' => "pr",
        _ => "",
    }
}


fn sfen_to_color(c: char) -> &'static str {
    match c {
        'P' | 'Z' | 'X' | 'Y' | 'Q' | 'W' | 'E' | 'L' | 'N' | 'S' | 'G' | 'B' | 'R' | 'K' => "B_",
        'p' | 'z' | 'x' | 'y' | 'q' | 'w' | 'e' | 'l' | 'n' | 's' | 'g' | 'b' | 'r' | 'k' => "W_",
        _ => "",
    }
}


pub fn generate_pos(board: Vec<(String, String)>) -> PartialPosition {

    let mut pos = PartialPosition::empty();

    for (square, piece) in board {
        if piece.trim().is_empty() {
            continue; 
        }

        let square_enum = match square.as_str() {
            "SQ_9A" => Square::SQ_9A,
            "SQ_8A" => Square::SQ_8A,
            "SQ_7A" => Square::SQ_7A,
            "SQ_6A" => Square::SQ_6A,
            "SQ_5A" => Square::SQ_5A,
            "SQ_4A" => Square::SQ_4A,
            "SQ_3A" => Square::SQ_3A,
            "SQ_2A" => Square::SQ_2A,
            "SQ_1A" => Square::SQ_1A,
            
            "SQ_9B" => Square::SQ_9B,
            "SQ_8B" => Square::SQ_8B,
            "SQ_7B" => Square::SQ_7B,
            "SQ_6B" => Square::SQ_6B,
            "SQ_5B" => Square::SQ_5B,
            "SQ_4B" => Square::SQ_4B,
            "SQ_3B" => Square::SQ_3B,
            "SQ_2B" => Square::SQ_2B,
            "SQ_1B" => Square::SQ_1B,
            
            "SQ_9C" => Square::SQ_9C,
            "SQ_8C" => Square::SQ_8C,
            "SQ_7C" => Square::SQ_7C,
            "SQ_6C" => Square::SQ_6C,
            "SQ_5C" => Square::SQ_5C,
            "SQ_4C" => Square::SQ_4C,
            "SQ_3C" => Square::SQ_3C,
            "SQ_2C" => Square::SQ_2C,
            "SQ_1C" => Square::SQ_1C,
            
            "SQ_9D" => Square::SQ_9D,
            "SQ_8D" => Square::SQ_8D,
            "SQ_7D" => Square::SQ_7D,
            "SQ_6D" => Square::SQ_6D,
            "SQ_5D" => Square::SQ_5D,
            "SQ_4D" => Square::SQ_4D,
            "SQ_3D" => Square::SQ_3D,
            "SQ_2D" => Square::SQ_2D,
            "SQ_1D" => Square::SQ_1D,

            "SQ_9E" => Square::SQ_9E,
            "SQ_8E" => Square::SQ_8E,
            "SQ_7E" => Square::SQ_7E,
            "SQ_6E" => Square::SQ_6E,
            "SQ_5E" => Square::SQ_5E,
            "SQ_4E" => Square::SQ_4E,
            "SQ_3E" => Square::SQ_3E,
            "SQ_2E" => Square::SQ_2E,
            "SQ_1E" => Square::SQ_1E,

            "SQ_9F" => Square::SQ_9F,
            "SQ_8F" => Square::SQ_8F,
            "SQ_7F" => Square::SQ_7F,
            "SQ_6F" => Square::SQ_6F,
            "SQ_5F" => Square::SQ_5F,
            "SQ_4F" => Square::SQ_4F,
            "SQ_3F" => Square::SQ_3F,
            "SQ_2F" => Square::SQ_2F,
            "SQ_1F" => Square::SQ_1F,

            "SQ_9G" => Square::SQ_9G,
            "SQ_8G" => Square::SQ_8G,
            "SQ_7G" => Square::SQ_7G,
            "SQ_6G" => Square::SQ_6G,
            "SQ_5G" => Square::SQ_5G,
            "SQ_4G" => Square::SQ_4G,
            "SQ_3G" => Square::SQ_3G,
            "SQ_2G" => Square::SQ_2G,
            "SQ_1G" => Square::SQ_1G,
            
            "SQ_9H" => Square::SQ_9H,
            "SQ_8H" => Square::SQ_8H,
            "SQ_7H" => Square::SQ_7H,
            "SQ_6H" => Square::SQ_6H,
            "SQ_5H" => Square::SQ_5H,
            "SQ_4H" => Square::SQ_4H,
            "SQ_3H" => Square::SQ_3H,
            "SQ_2H" => Square::SQ_2H,
            "SQ_1H" => Square::SQ_1H,
            
            "SQ_9I" => Square::SQ_9I,
            "SQ_8I" => Square::SQ_8I,
            "SQ_7I" => Square::SQ_7I,
            "SQ_6I" => Square::SQ_6I,
            "SQ_5I" => Square::SQ_5I,
            "SQ_4I" => Square::SQ_4I,
            "SQ_3I" => Square::SQ_3I,
            "SQ_2I" => Square::SQ_2I,
            "SQ_1I" => Square::SQ_1I,
            _ => panic!("Invalid square format: {}", square),
        };

        let piece_enum = match piece.as_str() {
            "W_P" => Piece::new(PieceKind::Pawn, Color::White),
            "W_L" => Piece::new(PieceKind::Lance, Color::White),
            "W_N" => Piece::new(PieceKind::Knight, Color::White),
            "W_S" => Piece::new(PieceKind::Silver, Color::White),
            "W_G" => Piece::new(PieceKind::Gold, Color::White),
            "W_B" => Piece::new(PieceKind::Bishop, Color::White),
            "W_R" => Piece::new(PieceKind::Rook, Color::White),
            "W_K" => Piece::new(PieceKind::King, Color::White), 
            "W_PB" => Piece::new(PieceKind::ProBishop, Color::White),
            "W_PL" => Piece::new(PieceKind::ProLance, Color::White),
            "W_PN" => Piece::new(PieceKind::ProKnight, Color::White),
            "W_PP" => Piece::new(PieceKind::ProPawn, Color::White),
            "W_PR" => Piece::new(PieceKind::ProRook, Color::White),
            "W_PS" => Piece::new(PieceKind::ProSilver, Color::White),
            
            "B_P" => Piece::new(PieceKind::Pawn, Color::Black),
            "B_L" => Piece::new(PieceKind::Lance, Color::Black),
            "B_N" => Piece::new(PieceKind::Knight, Color::Black),
            "B_S" => Piece::new(PieceKind::Silver, Color::Black),
            "B_G" => Piece::new(PieceKind::Gold, Color::Black),
            "B_B" => Piece::new(PieceKind::Bishop, Color::Black),
            "B_R" => Piece::new(PieceKind::Rook, Color::Black),
            "B_K" => Piece::new(PieceKind::King, Color::Black),
            "B_PB" => Piece::new(PieceKind::ProBishop, Color::Black),
            "B_PL" => Piece::new(PieceKind::ProLance, Color::Black),
            "B_PN" => Piece::new(PieceKind::ProKnight, Color::Black),
            "B_PP" => Piece::new(PieceKind::ProPawn, Color::Black),
            "B_PR" => Piece::new(PieceKind::ProRook, Color::Black),
            "B_PS" => Piece::new(PieceKind::ProSilver, Color::Black),
            _ => panic!("Invalid piece format: {}", piece),
        };

        pos.piece_set(square_enum, Some(piece_enum));
    }

    pos
}


pub fn get_color(sfen: &str) -> Color {

    let split: Vec<&str> = sfen.split_whitespace().collect();
    let curr_side = split[1];
    let mut side: Color = Color::White;
    if curr_side == "b" {
        side = Color::Black;
    } else {
        side = Color::White;
    }
    
    side
}


pub fn get_enemy_color(sfen: &str) -> Color {

    let split: Vec<&str> = sfen.split_whitespace().collect();
    let curr_side = split[1];
    let mut side: Color = Color::White;
    if curr_side == "w" {
        side = Color::Black;
    } else {
        side = Color::White;
    }
    
    side
}



