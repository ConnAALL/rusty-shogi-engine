// Russell Kosovsky

mod view;
use shogi_core::PartialPosition;
use shogi_core::Square;
use shogi_core::Piece;
use shogi_legality_lite::all_legal_moves_partial;

fn print_board(sfen: &str) {
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

    for file in 0..9 {
        for rank in (0..9).rev() {
            let square = format!("SQ_{}{}", rank + 1, ranks[file]).to_uppercase();
            let piece = board_state[file][rank].to_uppercase();
            println!("{:<7} {}", square, piece);
        }
        println!();
    }
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
        'p' => "p",
        'l' => "l",
        'n' => "n",
        's' => "s",
        'g' => "g",
        'b' => "b",
        'r' => "r",
        'k' => "k",
        _ => "",
    }
}


fn sfen_to_color(c: char) -> &'static str {
    match c {
        'P' | 'L' | 'N' | 'S' | 'G' | 'B' | 'R' | 'K' => "B_",
        'p' | 'l' | 'n' | 's' | 'g' | 'b' | 'r' | 'k' => "W_",
        _ => "",
    }
}


fn main() {

    let startpos = PartialPosition::startpos();

    let stri = startpos.to_sfen_owned();
    println!("STARTING BOARD");
    println!("{}", stri);
    view::display_sfen(&stri); 

    let moves = all_legal_moves_partial(&startpos);
    println!("All Possible Moves:");
    for move_item in moves {
        println!("{:?}", move_item);
    }    

    let sfen = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";
    print_board(sfen);

    let mut pos = PartialPosition::empty();
    
    // White pieces
    pos.piece_set(Square::SQ_9A, Some(Piece::W_L));
    pos.piece_set(Square::SQ_8A, Some(Piece::W_N));
    pos.piece_set(Square::SQ_7A, Some(Piece::W_S));
    pos.piece_set(Square::SQ_6A, Some(Piece::W_G));
    pos.piece_set(Square::SQ_5A, Some(Piece::W_K));
    pos.piece_set(Square::SQ_4A, Some(Piece::W_G));
    pos.piece_set(Square::SQ_3A, Some(Piece::W_S));
    pos.piece_set(Square::SQ_2A, Some(Piece::W_N));
    pos.piece_set(Square::SQ_1A, Some(Piece::W_L));
    
    pos.piece_set(Square::SQ_8B, Some(Piece::W_B));
    pos.piece_set(Square::SQ_2B, Some(Piece::W_R));
    
    pos.piece_set(Square::SQ_9C, Some(Piece::W_P));
    pos.piece_set(Square::SQ_8C, Some(Piece::W_P));
    pos.piece_set(Square::SQ_7C, Some(Piece::W_P));
    pos.piece_set(Square::SQ_6C, Some(Piece::W_P));
    pos.piece_set(Square::SQ_5C, Some(Piece::W_P));
    pos.piece_set(Square::SQ_4C, Some(Piece::W_P));
    pos.piece_set(Square::SQ_3C, Some(Piece::W_P));
    pos.piece_set(Square::SQ_2C, Some(Piece::W_P));
    pos.piece_set(Square::SQ_1C, Some(Piece::W_P));

    // Black Pieces
    pos.piece_set(Square::SQ_9G, Some(Piece::B_P));
    pos.piece_set(Square::SQ_8G, Some(Piece::B_P));
    pos.piece_set(Square::SQ_7G, Some(Piece::B_P));
    pos.piece_set(Square::SQ_6G, Some(Piece::B_P));
    pos.piece_set(Square::SQ_5G, Some(Piece::B_P));
    pos.piece_set(Square::SQ_4G, Some(Piece::B_P));
    pos.piece_set(Square::SQ_3G, Some(Piece::B_P));
    pos.piece_set(Square::SQ_2G, Some(Piece::B_P));
    pos.piece_set(Square::SQ_1G, Some(Piece::B_P));
 
    pos.piece_set(Square::SQ_8H, Some(Piece::B_B));
    pos.piece_set(Square::SQ_2H, Some(Piece::B_R));

    pos.piece_set(Square::SQ_9I, Some(Piece::B_L));
    pos.piece_set(Square::SQ_8I, Some(Piece::B_N));
    pos.piece_set(Square::SQ_7I, Some(Piece::B_S));
    pos.piece_set(Square::SQ_6I, Some(Piece::B_G));
    pos.piece_set(Square::SQ_5I, Some(Piece::B_K));
    pos.piece_set(Square::SQ_4I, Some(Piece::B_G));
    pos.piece_set(Square::SQ_3I, Some(Piece::B_S));
    pos.piece_set(Square::SQ_2I, Some(Piece::B_N));
    pos.piece_set(Square::SQ_1I, Some(Piece::B_L));
    
    let new_sfen = pos.to_sfen_owned();
    println!("{}", new_sfen);
    view::display_sfen(&new_sfen);

    let new_moves = all_legal_moves_partial(&pos);
    println!("All Possible Moves:");
    for move_item in new_moves {
        println!("{:?}", move_item);
    }    

}
