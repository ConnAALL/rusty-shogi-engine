// Russell Kosovsky

mod view;
use shogi_core::PartialPosition;
use shogi_core::Square;
use shogi_legality_lite::all_legal_moves_partial;


fn print_board(sfen: &str) {
    let pieces: Vec<char> = vec![
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

    for rank in (0..9).rev() {
        for file in 0..9 {
            let square = format!("SQ_{}{}", rank + 1, ranks[file]).to_uppercase();
            let piece = board_state[rank][file].to_uppercase();
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
}
