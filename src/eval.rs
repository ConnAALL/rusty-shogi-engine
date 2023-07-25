

/* Russell Kosovsky, Jim O'Conner
 
    * Module containing the 6 eval functions
        * 1. Piece Square Tables
        * 2. Promoted Pieces
        * 3. Mobility
        * 4. King Vulnerability
    * 

 */


use crate::sfen as SFEN;
use std::collections::HashMap;
use shogi_legality_lite::{normal_from_candidates, is_legal_partial_lite, all_legal_moves_partial};
use shogi_core::{Color, Square, PartialPosition, Piece, PieceKind, Move};


// CONST WEIGHTS FOR PIECES IN HAND 
const PAWN_HAND: u32 = 0;
const LANCE_HAND: u32 = 18;
const KNIGHT_HAND: u32 = 9;
const SILVER_HAND: u32 = 15;
const GOLD_HAND: u32 = 22;
const ROOK_HAND: u32 = 18;
const BISHOP_HAND: u32 = 2;

// CONST WEIGHTS FOR EVAL FEATURE VARIATES
const LANCE_MOBIL: u32 = 17;
const ROOK_MOBIL: u32 = 16;
const BISHOP_MOBIL: u32 = 17;
const PROMOTED_PIECES: u32 = 27;
const KING_VULN: u32 = 22;


/*
    ################################## 1. PIECE SQUARE TABLES ##################################
    
    REFERENCE FOR PROMOTED PIECES
        +P / +p ==> Z / z
        +L / +l ==> X / x
        +N / +n ==> Y / y
        +S / +s ==> Q / q
        +B / +b ==> W / w
        +R / +r ==> E / e
 */


// CONST PST WEIGHTS
fn pst() -> HashMap<&'static str, [i32; 81]> {
    let pst: HashMap<&'static str, [i32; 81]> = {
        let mut map = HashMap::new();
        map.insert("P", [ 3,  19, 9,  4,  28, 0,  3,  31, 6,
                          8,  1,  26, 0,  16, 0,  10, 20, 22,
                          24, 16, 3,  3,  29, 8,  9,  30, 4,
                          28, 3,  5,  17, 15, 3,  1,  0,  1,
                          13, 3,  21, 29, 3,  24, 29, 1,  17,
                          27, 14, 23, 12, 16, 18, 4,  27, 28,
                          14, 14, 24, 17, 5,  22, 16, 26, 10,
                          29, 28, 0,  29, 11, 6,  8,  31, 9,
                          9,  19, 23, 6,  11, 7,  3,  30, 26]);

        map.insert("L", [ 12, 3,  24, 25, 5,  13, 30, 29, 23,
                          25, 14, 18, 26, 11, 18, 6,  18, 2,
                          8,  26, 0,  3,  7,  4,  0,  22, 4,
                          20, 27, 18, 16, 28, 6,  17, 1,  30,
                          6,  12, 14, 14, 12, 28, 13, 9,  18,
                          5,  25, 5,  18, 3,  18, 21, 0,  6,
                          6,  15, 26, 26, 27, 2,  10, 15, 4,
                          0,  7,  13, 21, 6,  2,  10, 19, 1,
                          2,  19, 15, 12, 12, 2,  5,  19, 2]);

        map.insert("N", [ 8,  3,  19, 27, 8,  0,  3,  29, 16,
                          17, 22, 17, 12, 4,  13, 27, 28, 28,
                          8,  9,  6,  15, 23, 21, 2,  14, 28,
                          16, 16, 28, 12, 26, 24, 0,  13, 18,
                          24, 24, 25, 15, 12, 29, 5,  7,  31,
                          20, 10, 20, 26, 24, 29, 16, 16, 1,
                          21, 23, 10, 27, 31, 2,  12, 12, 5,
                          8,  27, 27, 7,  19, 20, 19, 14, 19,
                          29, 4,  28, 8,  3,  7,  28, 0,  24]);

        map.insert("S", [ 3,  1,  24, 30, 21, 15, 11, 24, 8,
                          11, 0,  25, 29, 5,  11, 30, 29, 24,
                          6,  30, 29, 18, 7,  14, 24, 23, 30,
                          16, 4,  19, 12, 8,  29, 26, 0,  23,
                          6,  22, 31, 10, 22, 7,  19, 2,  9,
                          29, 6,  15, 21, 29, 13, 2,  1,  29,
                          20, 28, 13, 2,  5,  0,  5,  31, 23,
                          21, 26, 17, 6,  14, 1,  10, 22, 10,
                          8,  19, 7,  10, 15, 24, 22, 11, 10]);

        map.insert("G", [ 11, 22, 26, 18, 9,  14, 19, 16, 30,
                          13, 6,  23, 3,  29, 2,  8,  18, 5,
                          11, 26, 1,  5,  12, 1,  2,  13, 29,
                          15, 2,  13, 5,  22, 3,  8,  21, 3,
                          5,  0,  7,  1,  11, 2,  26, 1,  29,
                          13, 5,  17, 10, 17, 26, 13, 6,  13,
                          11, 23, 18, 21, 20, 19, 29, 0,  16,
                          12, 8,  30, 14, 14, 15, 28, 2,  9,
                          1,  21, 21, 3,  25, 20, 12, 0,  0]);

        map.insert("R", [ 2,  2,  29, 8,  18, 20, 2,  22, 26,
                          26, 6,  11, 28, 2,  13, 20, 29, 30,
                          28, 23, 30, 6,  20, 6,  10, 4,  5,
                          17, 10, 20, 20, 3,  20, 16, 11, 21,
                          25, 9,  9,  6,  9,  26, 24, 5,  27,
                          1,  10, 16, 4,  13, 14, 25, 26, 10,
                          25, 9,  29, 9,  6,  4,  1,  31, 5,
                          0,  30, 10, 28, 20, 28, 12, 22, 25,
                          7,  11, 14, 13, 4,  12, 7,  12, 0]);

        map.insert("B", [ 28, 2,  7,  2,  8,  7,  21, 25, 8,
                          13, 5,  0,  12, 12, 14, 15, 14, 2,
                          9,  25, 11, 18, 8,  29, 15, 21, 22,
                          4,  23, 28, 10, 31, 13, 2,  1,  18,
                          0,  25, 14, 22, 4,  1,  24, 18, 1,
                          25, 17, 26, 6,  21, 3,  12, 5,  24,
                          30, 3,  13, 16, 9,  23, 22, 3,  16,
                          25, 29, 25, 4,  31, 9,  15, 1,  30,
                          1,  13, 19, 8,  13, 6,  13, 24, 12]);

        map.insert("W", [ 31, 30, 22, 4,  0,  2,  24, 21, 31,
                          30, 22, 5,  2,  8,  4,  1,  18, 3,
    6,  30, 19, 2,  16, 28, 19, 24, 3,
                          23, 22, 22, 7,  3,  17, 7,  26, 1,
                          7,  23, 24, 29, 21, 30, 16, 2,  28,
                          21, 27, 8,  30, 17, 15, 30, 20, 5,
                          1,  11, 26, 28, 8,  8,  25, 26, 23,
                          6,  8,  15, 0,  4,  29, 12, 30, 12,
                          20, 12, 7,  24, 7,  27, 13, 10, 5]);

        map.insert("E", [ 21, 0,  6,  13, 31, 28, 6,  2,  31,
                          8,  19, 30, 2,  25, 0,  4,  9,  17,
                          10, 28, 18, 2,  31, 6,  20, 18, 6,
                          8,  24, 31, 6,  2,  25, 7,  20, 2,
                          22, 17, 25, 23, 15, 18, 19, 0,  27,
                          23, 20, 24, 31, 26, 11, 6,  19, 8,
                          11, 0,  2,  7,  7,  2,  29, 31, 5,
                          8,  10, 13, 6,  26, 19, 16, 25, 28,
                          15, 30, 1,  30, 20, 25, 11, 31, 23]);
        map
    };
    
    pst // return the hashmap

}


fn pst_parse(sfen: &str) -> Vec<char> {

    let split: Vec<&str> = sfen.split_whitespace().collect();
    let pieces = split[0];
    // println!("{:?}", pieces);
    let replaced = pieces.replace("/", "");
    let clean = SFEN::convert_promoted_pieces(&replaced);

    let mut result = Vec::new();
    for ch in clean.chars() {
        if ch.is_digit(10) {
            let count = ch.to_digit(10).unwrap();
            for _ in 0..count {
                result.push('*');
            }
        } else {
            result.push(ch);
        }
    }
    
    result

} 


pub fn evaluate_piece_table(mut sfen: &str, color: &str) -> i32 {
    
    let mut white_score = 0;
    let mut black_score = 0;

    let pst_map = pst();
    let mut sfen_vec = Vec::<char>::new();

    if color == "black" {
        let clean = SFEN::convert_promoted_pieces(&sfen);
        let case_flip = SFEN::flip_case(&clean); // swaps lowercase with uppercase
        let flipped = SFEN::flip(&case_flip); // actual board flip
        //println!("SFEN: {:?}", flipped);
        //view::display_sfen(&flipped);
        sfen_vec = pst_parse(&flipped);
    } else if color == "white" {
        //println!("SFEN: {:?}", sfen);
        //view::display_sfen(sfen);
        sfen_vec = pst_parse(sfen);
    }
    

    let mut index = 0;
    for i in sfen_vec {

        if i == 'p'{
            let value = pst_map["P"][index];
            if color == "black" {
                black_score += value
            } else if color == "white" {
                white_score += value;
            }
        
        } else if i == 'l' {
            let value = pst_map["L"][index];
            if color == "black" {
                black_score += value
            } else if color == "white" {
                white_score += value;
            }

        } else if i == 'n' {
            let value = pst_map["N"][index];
            if color == "black" {
                black_score += value
            } else if color == "white" {
                white_score += value;
            }

        } else if i == 's' {
            let value = pst_map["S"][index];
            if color == "black" {
                black_score += value
            } else if color == "white" {
                white_score += value;
            }

        } else if i == 'g' {
            let value = pst_map["G"][index];
            if color == "black" {
                black_score += value
            } else if color == "white" {
                white_score += value;
            }

        } else if i == 'r' {
            let value = pst_map["R"][index];
            if color == "black" {
                black_score += value
            } else if color == "white" {
                white_score += value;
            }

        } else if i == 'b' {
            let value = pst_map["B"][index];
            if color == "black" {
                black_score += value
            } else if color == "white" {
                white_score += value;
            }

        } else if i == 'e' /* PR rook */ {
            let value = pst_map["E"][index];
            if color == "black" {
                black_score += value
            } else if color == "white" {
                white_score += value;
            }

        } else if i == 'w' /* PR bishop */ {
            let value = pst_map["W"][index];
            if color == "black" {
                black_score += value
            } else if color == "white" {
                white_score += value;
            }

} else if i == 'z' /* PR pawn */ {
            let value = pst_map["G"][index];
            if color == "black" {
                black_score += value
            } else if color == "white" {
                white_score += value;
            }

        } else if i == 'x' /* PR lance */ {
            let value = pst_map["G"][index];
            if color == "black" {
                black_score += value
            } else if color == "white" {
                white_score += value;
            }

        } else if i == 'y' /* PR knight */ {
            let value = pst_map["G"][index];
            if color == "black" {
                black_score += value
            } else if color == "white" {
                white_score += value;
            }

        } else if i == 'q' /* PR siLver */ {
            let value = pst_map["G"][index];
            if color == "black" {
                black_score += value
            } else if color == "white" {
                white_score += value;
            }
    
        } else if i == 'k' {

        } else if i == 'K' {
        
        }

        index += 1;
    }

    if color == "white" {
        //println!("{:?}", white_score);
        white_score
    } else if color == "black"{
        //println!("{:?}", black_score);
        black_score
    } else {
        0
    }

}


//   #################################### 2. PROMOTED PIECES ####################################


pub fn promoted_pieces(sfen: &str) -> (u32, u32) {
    let sfen_chars: Vec<char> = sfen.chars().collect();
    let mut num_black_pieces = 0;
    let mut num_white_pieces = 0;
    let mut index = 0;

    for ch in sfen_chars.iter() {
        if *ch == '+' {
            let nxt = sfen_chars[index + 1];
            // black
            if nxt.is_ascii_uppercase() {
                num_black_pieces += 1;
            }
            // white
            if nxt.is_ascii_lowercase() {
                num_white_pieces += 1;
            }
        }
        index += 1;
    }

    (num_black_pieces, num_white_pieces)

}


//   ####################################### 3. MOBILITY #######################################


pub fn mobility(sfen: &str, coord: String) -> u32 {

/*   
 *              SQUARE INDEXES
 *    FILE >>>  
 *    I   H   G   F   E   D   C   B   A  
 *    #   #   #   #   #   #   #   #   #  #  
 *    73  64  55  46  37  28  19  10  1  #  1  R
 *    74  65  56  47  38  29  20  11  2  #  2  A
 *    75  66  57  48  39  30  21  12  3  #  3  N
 *    76  67  58  49  40  31  22  13  4  #  4  K
 *    77  68  59  50  41  32  23  14  5  #  5  
 *    78  69  60  51  42  33  24  15  6  #  6  
 *    79  70  61  52  43  34  25  16  7  #  7  
 *    80  71  62  53  44  35  26  17  8  #  8  
 *    81  72  63  54  45  36  27  18  9  #  9    
 */
    
    // Convert the coordinate to file and rank indices
    let file = coord.chars().nth(3).unwrap() as u8 - b'1' + 1;
    let rank = coord.chars().nth(4).unwrap() as u8 - b'A' + 1;
    //println!("file: {:?}", file);
    //println!("rank: {:?}", rank);

    // Parse the SFEN string into a position
    let positions = SFEN::sfen_parse(sfen);// creates list of board squares and the pieces on them (if there are any)
    let mut pos = SFEN::generate_pos(positions); // creates a "partial position" out of it
    pos.side_to_move_set(SFEN::get_color(sfen)); // finalize the partial position

    // Find the rook's square based on the given file and rank
    let square = shogi_core::Square::new(file, rank).expect("Invalid coordinate");

    // Print square index
    // println!("rook sqr: {:?}", rook_square);

    // Get the Bitboard of possible rook moves from the LiteLegalityChecker
    let possible_moves = normal_from_candidates(&pos, square);

    // Count the number of squares the rook can move to
    let num_moves = possible_moves.count() as u32;

    // Iterate over the possible moves and find the captured pieces
    let mut captured_pieces = Vec::<(PieceKind, Color)>::new();
    for to_square in possible_moves {
        if let Some(captured_piece) = pos.piece_at(to_square) {
            captured_pieces.push(captured_piece.to_parts());
        }
    }

    // Return the count of possible moves and the captured pieces
    //(num_moves, captured_pieces)
    num_moves

}


pub fn rook_mobility(sfen: &str) -> (u32, u32) {

    let positions = SFEN::sfen_parse(sfen);// creates list of board squares and the pieces on them (if there are any)
    let mut pos = SFEN::generate_pos(positions.clone()); // creates a "partial position" out of it
    pos.side_to_move_set(SFEN::get_color(sfen));
    
    //println!("Positions: {:?}", positions);

    let mut white_rook_mobil = 0;
    let mut black_rook_mobil = 0;
    
    for sqr in &positions {
       
        if sqr.1 == "W_R" {
            pos.side_to_move_set(Color::White);
            let coord = &sqr.0;
            //println!("COORD: {:?}", coord);
            let sfen = pos.to_sfen_owned();
            let mobil = mobility(&sfen, coord.to_string());
            white_rook_mobil += mobil;
            //println!("MOBILITY: {:?}", mobil);

        } else if sqr.1 == "B_R" {
            pos.side_to_move_set(Color::Black);
            let coord = &sqr.0;
            //println!("COORD: {:?}", coord);
            let sfen = pos.to_sfen_owned();
            let mobil = mobility(&sfen, coord.to_string());
            black_rook_mobil += mobil;
            //println!("MOBILITY: {:?}", mobil);
        }
    }

    //println!("Final White Mobility: {:?}", white_rook_mobil);
    //println!("Final Black Mobility: {:?}", black_rook_mobil);
    (white_rook_mobil, black_rook_mobil)

}


pub fn lance_mobility(sfen: &str) -> (u32, u32) {

    let positions = SFEN::sfen_parse(sfen);// creates list of board squares and the pieces on them (if there are any)
    let mut pos = SFEN::generate_pos(positions.clone()); // creates a "partial position" out of it
    pos.side_to_move_set(SFEN::get_color(sfen));
    
    //println!("Positions: {:?}", positions);

    let mut white_lance_mobil = 0;
    let mut black_lance_mobil = 0;
    
    for sqr in &positions {
       
        if sqr.1 == "W_L" {
            pos.side_to_move_set(Color::White);
            let coord = &sqr.0;
            //println!("COORD: {:?}", coord);
            let sfen = pos.to_sfen_owned();
            let mobil = mobility(&sfen, coord.to_string());
            white_lance_mobil += mobil;
            //println!("MOBILITY: {:?}", mobil);

        } else if sqr.1 == "B_L" {
            pos.side_to_move_set(Color::Black);
            let coord = &sqr.0;
            //println!("COORD: {:?}", coord);
            let sfen = pos.to_sfen_owned();
            let mobil = mobility(&sfen, coord.to_string());
            black_lance_mobil += mobil;
            //println!("MOBILITY: {:?}", mobil);
        }
    }

    (white_lance_mobil, black_lance_mobil)

}


pub fn bishop_mobility(sfen: &str) -> (u32, u32) {

    let positions = SFEN::sfen_parse(sfen);// creates list of board squares and the pieces on them (if there are any)
    let mut pos = SFEN::generate_pos(positions.clone()); // creates a "partial position" out of it
    pos.side_to_move_set(SFEN::get_color(sfen));
    
    //println!("Positions: {:?}", positions);

    let mut white_bishop_mobil = 0;
    let mut black_bishop_mobil = 0;
    
    for sqr in &positions {
       
        if sqr.1 == "W_B" {
            pos.side_to_move_set(Color::White);
            let coord = &sqr.0;
            //println!("COORD: {:?}", coord);
            let sfen = pos.to_sfen_owned();
            let mobil = mobility(&sfen, coord.to_string());
            white_bishop_mobil += mobil;
            //println!("MOBILITY: {:?}", mobil);

        } else if sqr.1 == "B_B" {
            pos.side_to_move_set(Color::Black);
            let coord = &sqr.0;
            //println!("COORD: {:?}", coord);
            let sfen = pos.to_sfen_owned();
            let mobil = mobility(&sfen, coord.to_string());
            black_bishop_mobil += mobil;
            //println!("MOBILITY: {:?}", mobil);
        }
    }

    (white_bishop_mobil, black_bishop_mobil)

}


//   ################################## 4. KING VULNERABILITY ##################################


// Function to check if a piece can attack a given square
fn can_attack(pos: &PartialPosition, piece: Piece, src: Option<Square>, dst: Option<Square>) -> bool {
    //println!("dst: {:?}", dst);
    if let (Some(from), Some(to)) = (src, dst) {
        let mv = Move::Normal {
            from,
            to,
            promote: false,
        };
        //println!("move: {:?}", mv);
        //if is_legal_partial_lite(pos, mv) {
        //    println!("{:?}", mv);
        //}
        
        return is_legal_partial_lite(&pos, mv);
    
    } else {
       return false;
    }

}


// Function to find the attackers of a given square
fn attackers(pos: &PartialPosition, color: Color, square: Option<Square>) -> Vec<Option<Square>> {
    //println!("Entered ATTACKERS!");
    //println!("CURR TURN: {:?}", pos.side_to_move());
    //println!("Color param: {:?}", color);
    //println!("square (king): {:?}", square);

    let mut result = Vec::new();

    for file in 1..=9 {
        for rank in 1..=9 {
            let src_square = Square::new(file, rank);
            //println!("square: {:?}", src_square);
            if let Some(piece) = pos.piece_at(src_square.unwrap()) {
                //println!("piece: {:?}", piece.color());
                if piece.color() == color && can_attack(pos, piece, src_square, square) {
                    result.push(src_square);
                }
            }
        }
    }

    result

}


pub fn enemy_king_vuln(sfen: &str, side: Color) -> i32 {

    const ATK_WEIGHT: i32 = 1;
    const DEF_WEIGHT: f32 = 0.5;
    const K_ATK_WEIGHT: i32 = 1;
    const ESC_WEIGHT: i32 = 1;


    // Parse the SFEN string into a position
    let positions = SFEN::sfen_parse(sfen);
    let mut pos = SFEN::generate_pos(positions.clone());
    pos.side_to_move_set(side);
    //println!("Side to Move: {:?}", pos.side_to_move());

    // Determine the color and enemy color based on the player's case
    let player = pos.side_to_move();
    let (color, enemy_color) = if player == Color::White {
        (Color::White, Color::Black)
    } else {
        (Color::Black, Color::White)
    };

    //println!("Color: {:?}", color);
    //println!("Enemy Color: {:?}", enemy_color);

    let king_square = pos.king_position(enemy_color);
    //println!("King's Square: {:?}", king_square);

    let file = king_square.unwrap().file();
    let rank = king_square.unwrap().rank();

    // --------------------------------------------------------------------------------------------------
    // Construct the list of 8 squares that surround the king
    //println!("\n-------------Constructing the list of 8 squares that surround the king");
    let files = (file.saturating_sub(1)..=file.saturating_add(1)).filter(|&f| 1 <= f && f <= 9);
    let ranks = (rank.saturating_sub(1)..=rank.saturating_add(1)).filter(|&r| 1 <= r && r <= 9);
    //println!("files: {:?}", files.clone().collect::<Vec<_>>());
    //println!("ranks: {:?}", ranks.clone().collect::<Vec<_>>());
    let squares: Vec<Option<Square>> = files
        .flat_map(|f| ranks.clone().map(move |r| Square::new(f, r)))
        .filter(|&s| s != king_square)
        .collect();
    //println!("SQUARES surrounding king: {:?}\n", squares);


    // --------------------------------------------------------------------------------------------------
    // Calculate the number of pieces that can attack the squares surrounding the king
    //println!("\n-------------Calculateing the number of pieces that can attack the squares surrounding the king");
    pos.side_to_move_set(color);
    let num_attackers = squares.iter().filter(|&&s| {
        //println!("S: {:?}", s);
        let attackers = attackers(&pos, color, s);
        !attackers.is_empty()
    }).count() as i32;
    //println!("num_attackers: {:?}\n", num_attackers);


    // --------------------------------------------------------------------------------------------------
    // Calculate the number of pieces that can defend the squares surrounding the king
    //println!("\n-------------Calculateing the number of pieces that can defend the squares surrounding the king");
    pos.side_to_move_set(enemy_color);
    let mut total_defenders = Vec::<Option<Square>>::new();
    for s in squares {
        //println!("S: {:?}", s);
        let defenders = attackers(&pos, enemy_color, s);
        total_defenders.extend(defenders);   
    };
    //println!("Total defenders: {:?}", total_defenders);
    let num_defenders = total_defenders.len() as i32; 
    //println!("num_defenders: {:?}\n", num_defenders);


    // --------------------------------------------------------------------------------------------------
    // Calculate the number of pieces attacking the king
    //println!("\n-------------Calculateing the number of pieces attacking the king");
    //println!("Side To Move: {:?}", color);
    pos.side_to_move_set(color);
    let enemy_king_sqr = pos.king_position(enemy_color);
    //println!("enemy king square: {:?}", enemy_king_sqr);
    
    // change king to a pawn
    //println!("changing king to pawn:");
    if enemy_color == Color::White {
        pos.piece_set(enemy_king_sqr.unwrap(), Some(Piece::W_P));
    } else {
        pos.piece_set(enemy_king_sqr.unwrap(), Some(Piece::B_P));
    }

    let next_moves = all_legal_moves_partial(&pos); 
    
    let mut attacks = Vec::new();

    for move_item in next_moves {
        
        if move_item.to() == enemy_king_sqr.unwrap() && move_item.is_promoting() == false {
            attacks.push(move_item);
            //println!("{:?}", move_item);
        }
    }

    // change back to king
    //println!("changing back to king:");
    if enemy_color == Color::White {
        pos.piece_set(enemy_king_sqr.unwrap(), Some(Piece::W_K));
    } else {
        pos.piece_set(enemy_king_sqr.unwrap(), Some(Piece::B_K));
    }

    let num_king_attackers = attacks.len() as i32;
    //println!("num_king_attackers: {:?}", num_king_attackers);


    // --------------------------------------------------------------------------------------------------
    // Calculate the number of escape routes the king has
    //println!("\n-------------Calculateing number of escape routes for the king");
    pos.side_to_move_set(enemy_color);
    let escapes = normal_from_candidates(&pos, king_square.clone().unwrap());
    //println!("escapes: {:?}", escapes);
    let num_escapes = escapes.count() as i32;
    //println!("num_escapes: {:?}\n", num_escapes);

    // Modify the values with internal weightings
    let king_vulnerability = (num_attackers * ATK_WEIGHT
                            - num_defenders * DEF_WEIGHT as i32
                            + num_king_attackers * K_ATK_WEIGHT
                            - num_escapes * ESC_WEIGHT)
                            .max(0);

    king_vulnerability

}


//   ################################## 5. PIECES IN HAND ##################################


pub fn eval_hand(sfen: &str) -> (u32, u32) {
    
    let mut white_pieces = Vec::new();
    let mut black_pieces = Vec::new();

    let parts: Vec<&str> = sfen.split_whitespace().collect();
    //if parts.len() < 3 {
    //    return (white_pieces, black_pieces);  
    //}

    let pieces_in_hand = parts[2];
    let mut current_count: Option<u32> = None;
    for c in pieces_in_hand.chars() {
        if let Some(n) = c.to_digit(10) {              
            current_count = Some(n);
        } else {
            let piece = c.to_string();
            let pieces = vec![piece.clone(); current_count.unwrap_or(1) as usize];
            if c.is_uppercase() {
                black_pieces.extend(pieces);
            } else {
                white_pieces.extend(pieces);
            }
            current_count = None;
        }
    }

    let mut white_hand_value = 0;
    let mut black_hand_value = 0;

    for piece in white_pieces {
        white_hand_value += match piece.as_str() {
            "p" => PAWN_HAND,
            "l" => LANCE_HAND,
            "n" => KNIGHT_HAND,
            "s" => SILVER_HAND,
            "g" => GOLD_HAND,
            "r" => ROOK_HAND,
            "b" => BISHOP_HAND,
            _ => 0,
        };
    }
    for piece in black_pieces {
        black_hand_value += match piece.as_str() {
            "P" => PAWN_HAND,
            "L" => LANCE_HAND,
            "N" => KNIGHT_HAND,
            "S" => SILVER_HAND,
            "G" => GOLD_HAND,
            "R" => ROOK_HAND,
            "B" => BISHOP_HAND,
            _ => 0,
        };
    }

    (white_hand_value, black_hand_value)

}


pub fn evaluate(sfen: &str) -> (f32, f32) {

    let mut white_fitness = 0;
    let mut black_fitness = 0;

// ---------------------------------PROMOTED PIECES---------------------------------

    let (mut white_pp, mut black_pp) = promoted_pieces(sfen);
    white_fitness += white_pp * PROMOTED_PIECES;
    black_fitness += black_pp * PROMOTED_PIECES;
    
// ---------------------------------PIECE SQUARE TABLES---------------------------------

    let white_pst = evaluate_piece_table(&sfen, "white");
    let black_pst = evaluate_piece_table(&sfen, "black");

    white_fitness += white_pst as u32;
    black_fitness += black_pst as u32;

// ---------------------------------KING VULN---------------------------------

    let white_king_vln = enemy_king_vuln(&sfen, Color::White);
    let black_king_vln = enemy_king_vuln(&sfen, Color::Black);

    //println!("white_king_vln: {:?}", white_king_vln);
    //println!("black_king_vln: {:?}", black_king_vln);
    
    white_fitness += white_king_vln as u32 * KING_VULN;
    black_fitness += black_king_vln as u32 * KING_VULN;

// ---------------------------------ROOK MOBIL---------------------------------

    let (white_rook_mobil, black_rook_mobil) = rook_mobility(&sfen);

    //println!("white_rook_mobil: {:?}", white_rook_mobil);
    //println!("black_rook_mobil: {:?}", black_rook_mobil);
    
    white_fitness += white_rook_mobil * ROOK_MOBIL;
    black_fitness += black_rook_mobil * ROOK_MOBIL;

// ---------------------------------LANCE MOBIL---------------------------------

    let (white_lance_mobil, black_lance_mobil) = lance_mobility(&sfen);
    
    //println!("white_lance_mobil: {:?}", white_lance_mobil);
    //println!("black_lance_mobil: {:?}", black_lance_mobil);
    
    white_fitness += white_lance_mobil * LANCE_MOBIL;
    black_fitness += black_lance_mobil * LANCE_MOBIL;

// ---------------------------------BISHOP MOBIL---------------------------------
    
    let (white_bish_mobil, black_bish_mobil) = bishop_mobility(&sfen);
    
    //println!("white_bish_mobil: {:?}", white_bish_mobil);
    //println!("black_bish_mobil: {:?}", black_bish_mobil);
    
    white_fitness += white_bish_mobil * BISHOP_MOBIL;
    black_fitness += black_bish_mobil * BISHOP_MOBIL;

// ---------------------------------PIECES IN HAND---------------------------------

    let (white_hand, black_hand) = eval_hand(&sfen);
    
    white_fitness += white_hand;
    black_fitness += black_hand;
    
    //println!("white hand: {:?}", white_hand);
    //println!("black hand: {:?}", black_hand);

// ---------------------------------RETURN BOTH FITNESSES
    
    return(white_fitness as f32, black_fitness as f32);

}

