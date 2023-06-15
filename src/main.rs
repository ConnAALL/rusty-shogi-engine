// Russell Kosovsky

mod view;
use shogi_core::PartialPosition;
use shogi_core::Square;
use shogi_core::Color;
use shogi_core::Piece;
use shogi_core::PieceKind;
use shogi_legality_lite::all_legal_moves_partial;


const SQUARES: [&str; 82] = ["init",
"SQ_1A",
"SQ_1B",
"SQ_1C",
"SQ_1D",
"SQ_1E",
"SQ_1F",
"SQ_1G",
"SQ_1H",
"SQ_1I",
"SQ_2A",
"SQ_2B",
"SQ_2C",
"SQ_2D",
"SQ_2E",
"SQ_2F",
"SQ_2G",
"SQ_2H",
"SQ_2I",
"SQ_3A",
"SQ_3B",
"SQ_3C",
"SQ_3D",
"SQ_3E",
"SQ_3F",
"SQ_3G",
"SQ_3H",
"SQ_3I",
"SQ_4A",
"SQ_4B",
"SQ_4C",
"SQ_4D",
"SQ_4E",
"SQ_4F",
"SQ_4G",
"SQ_4H",
"SQ_4I",
"SQ_5A",
"SQ_5B",
"SQ_5C",
"SQ_5D",
"SQ_5E",
"SQ_5F",
"SQ_5G",
"SQ_5H",
"SQ_5I",
"SQ_6A",
"SQ_6B",
"SQ_6C",
"SQ_6D",
"SQ_6E",
"SQ_6F",
"SQ_6G",
"SQ_6H",
"SQ_6I",
"SQ_7A",
"SQ_7B",
"SQ_7C",
"SQ_7D",
"SQ_7E",
"SQ_7F",
"SQ_7G",
"SQ_7H",
"SQ_7I",
"SQ_8A",
"SQ_8B",
"SQ_8C",
"SQ_8D",
"SQ_8E",
"SQ_8F",
"SQ_8G",
"SQ_8H",
"SQ_8I",
"SQ_9A",
"SQ_9B",
"SQ_9C",
"SQ_9D",
"SQ_9E",
"SQ_9F",
"SQ_9G",
"SQ_9H",
"SQ_9I"];


fn sfen_parse(sfen: &str) -> Vec<(String, String)> {

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



fn generate_pos(board: Vec<(String, String)>) -> PartialPosition {

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



fn main() {


    let start = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";
    let positions = sfen_parse(start);
    println!("{:?}", positions);

    let pos = generate_pos(positions);
    let sfen = pos.to_sfen_owned();

    println!("{}", sfen);
    view::display_sfen(&sfen);

    let new_moves = all_legal_moves_partial(&pos);
    println!("All Possible Moves:");
    
    let mut sfen_list = Vec::new();
    for move_item in new_moves {
        // display move object
        println!("{:?}", move_item);
        
        // display from sqr
        let fromindex = move_item.from().unwrap().index();
        let fromsqr = SQUARES[fromindex as usize];
        println!("FROM: {:?}", fromsqr);
        
        // display to sqr
        let toindex = move_item.to().index();
        let tosqr = SQUARES[toindex as usize];
        println!("TO: {:?}", tosqr);
        
        // clone position and "make" the move so we can obtain the sfen
        let mut temp_pos = pos.clone();
        temp_pos.make_move(move_item);
        let sfen = temp_pos.to_sfen_owned();
        sfen_list.push(sfen.clone());
        println!("{}", sfen);
        //view::display_sfen(&sfen);
        
        println!("\n");
    }
    println!("{:?}", sfen_list);
}


fn test() {

    let startpos = PartialPosition::startpos();

    let stri = startpos.to_sfen_owned();
    println!("STARTING BOARD");
    println!("{}", stri);
    view::display_sfen(&stri); 

    let start = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";
    let positions = sfen_parse(start);
    println!("{:?}", positions);

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
    let mut sfen_list = Vec::new();
    for move_item in new_moves {
        // display move object
        println!("{:?}", move_item);
        
        // display from sqr
        let fromindex = move_item.from().unwrap().index();
        let fromsqr = SQUARES[fromindex as usize];
        println!("FROM: {:?}", fromsqr);
        
        // display to sqr
        let toindex = move_item.to().index();
        let tosqr = SQUARES[toindex as usize];
        println!("TO: {:?}", tosqr);
        
        // clone position and "make" the move so we can obtain the sfen
        let mut temp_pos = pos.clone();
        temp_pos.make_move(move_item);
        let sfen = temp_pos.to_sfen_owned();
        sfen_list.push(sfen.clone());
        println!("{}", sfen);
        //view::display_sfen(&sfen);
        
        println!("\n");
    }
    println!("{:?}", sfen_list);
}
