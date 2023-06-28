/* Russell Kosovsky, Jim O'Conner
 
    * Module containing the 6 eval functions
        * 1. Piece Square Tables
        * 2. Promoted Pieces
        * 3. Mobility
        * 4. King Vulnerability
    * 

 */


use crate::view;
use crate::sfen as SFEN;
use std::collections::HashMap;
//use shogi_legality_lite::normal_from_candidates;
use shogi_core::{Square, Piece, Color, Move, PieceKind};
use shogi_legality_lite::{LegalityChecker, LiteLegalityChecker};


/*

    ############################################################################################
    ################################## 1. PIECE SQUARE TABLES ##################################
    ############################################################################################
    
    REFERENCE FOR PROMOTED PIECES
        +P / +p ==> Z / z
        +L / +l ==> X / x
        +N / +n ==> Y / y
        +S / +s ==> Q / q
        +B / +b ==> W / w
        +R / +r ==> E / e

 */


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

fn pst() -> HashMap<&'static str, [i32; 81]> {
    let pst: HashMap<&'static str, [i32; 81]> = {
        let mut map = HashMap::new();
        map.insert("P", [ 0, 0, 0, 0, 0, 0, 0, 0, 0,
                          4, 4, 4, 4, 4, 4, 4, 4, 4,
                          6, 6, 6, 6, 6, 6, 6, 6, 6,
                          8, 8, 8, 8, 8, 8, 8, 8, 8,
                          6, 6, 6, 6, 6, 6, 6, 6, 6,
                          4, 4, 4, 4, 4, 4, 4, 4, 4,
                          2, 2, 2, 2, 2, 2, 2, 2, 2,
                          0, 0, 0, 0, 0, 0, 0, 0, 0,
                          0, 0, 0, 0, 0, 0, 0, 0, 0]);

        map.insert("L", [ 0, 0, 0, 0, 0, 0, 0, 0, 0,
                          4, 4, 4, 4, 4, 4, 4, 4, 4,
                          6, 6, 6, 6, 6, 6, 6, 6, 6,
                          8, 8, 8, 8, 8, 8, 8, 8, 8,
                          6, 6, 6, 6, 6, 6, 6, 6, 6,
                          4, 4, 4, 4, 4, 4, 4, 4, 4,
                          2, 2, 2, 2, 2, 2, 2, 2, 2,
                          0, 0, 0, 0, 0, 0, 0, 0, 0,
                          0, 0, 0, 0, 0, 0, 0, 0, 0]);

        map.insert("N", [ 0, 0, 0, 0, 0, 0, 0, 0, 0,
                          4, 4, 4, 4, 4, 4, 4, 4, 4,
                          6, 6, 6, 6, 6, 6, 6, 6, 6,
                          8, 8, 8, 8, 8, 8, 8, 8, 8,
                          6, 6, 6, 6, 6, 6, 6, 6, 6,
                          4, 4, 4, 4, 4, 4, 4, 4, 4,
                          2, 2, 2, 2, 2, 2, 2, 2, 2,
                          0, 0, 0, 0, 0, 0, 0, 0, 0,
                          0, 0, 0, 0, 0, 0, 0, 0, 0]);

        map.insert("S", [ 0, 0, 0, 0, 0, 0, 0, 0, 0,
                          4, 4, 4, 4, 4, 4, 4, 4, 4,
                          6, 6, 6, 6, 6, 6, 6, 6, 6,
                          8, 8, 8, 8, 8, 8, 8, 8, 8,
                          6, 6, 6, 6, 6, 6, 6, 6, 6,
                          4, 4, 4, 4, 4, 4, 4, 4, 4,
                          2, 2, 2, 2, 2, 2, 2, 2, 2,
                          0, 0, 0, 0, 0, 0, 0, 0, 0,
                          0, 0, 0, 0, 0, 0, 0, 0, 0]);

        map.insert("G", [ 0, 0, 0, 0, 0, 0, 0, 0, 0,
                          4, 4, 4, 4, 4, 4, 4, 4, 4,
                          6, 6, 6, 6, 6, 6, 6, 6, 6,
                          8, 8, 8, 8, 8, 8, 8, 8, 8,
                          6, 6, 6, 6, 6, 6, 6, 6, 6,
                          4, 4, 4, 4, 4, 4, 4, 4, 4,
                          2, 2, 2, 2, 2, 2, 2, 2, 2,
                          0, 0, 0, 0, 0, 0, 0, 0, 0,
                          0, 0, 0, 0, 0, 0, 0, 0, 0]);

        map.insert("R", [ 0, 0, 0, 0, 0, 0, 0, 0, 0,
                          4, 4, 4, 4, 4, 4, 4, 4, 4,
                          6, 6, 6, 6, 6, 6, 6, 6, 6,
                          8, 8, 8, 8, 8, 8, 8, 8, 8,
                          6, 6, 6, 6, 6, 6, 6, 6, 6,
                          4, 4, 4, 4, 4, 4, 4, 4, 4,
                          2, 2, 2, 2, 2, 2, 2, 2, 2,
                          0, 0, 0, 0, 0, 0, 0, 0, 0,
                          0, 0, 0, 0, 0, 0, 0, 0, 0]);

        map.insert("B", [ 0, 0, 0, 0, 0, 0, 0, 0, 0,
                          4, 4, 4, 4, 4, 4, 4, 4, 4,
                          6, 6, 6, 6, 6, 6, 6, 6, 6,
                          8, 8, 8, 8, 8, 8, 8, 8, 8,
                          6, 6, 6, 6, 6, 6, 6, 6, 6,
                          4, 4, 4, 4, 4, 4, 4, 4, 4,
                          2, 2, 2, 2, 2, 2, 2, 2, 2,
                          0, 0, 0, 0, 0, 0, 0, 0, 0,
                          0, 0, 0, 0, 0, 0, 0, 0, 0]);

        map.insert("W", [ 0, 0, 0, 0, 0, 0, 0, 0, 0,
                          4, 4, 4, 4, 4, 4, 4, 4, 4,
                          6, 6, 6, 6, 6, 6, 6, 6, 6,
                          8, 8, 8, 8, 8, 8, 8, 8, 8,
                          6, 6, 6, 6, 6, 6, 6, 6, 6,
                          4, 4, 4, 4, 4, 4, 4, 4, 4,
                          2, 2, 2, 2, 2, 2, 2, 2, 2,
                          0, 0, 0, 0, 0, 0, 0, 0, 0,
                          0, 0, 0, 0, 0, 0, 0, 0, 0]);

        map.insert("E", [ 0, 0, 0, 0, 0, 0, 0, 0, 0,
                          4, 4, 4, 4, 4, 4, 4, 4, 4,
                          6, 6, 6, 6, 6, 6, 6, 6, 6,
                          8, 8, 8, 8, 8, 8, 8, 8, 8,
                          6, 6, 6, 6, 6, 6, 6, 6, 6,
                          4, 4, 4, 4, 4, 4, 4, 4, 4,
                          2, 2, 2, 2, 2, 2, 2, 2, 2,
                          0, 0, 0, 0, 0, 0, 0, 0, 0,
                          0, 0, 0, 0, 0, 0, 0, 0, 0]);
        map
    };
    
    pst // return the hashmap
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
        println!("SFEN: {:?}", flipped);
        view::display_sfen(&flipped);
        sfen_vec = pst_parse(&flipped);
    } else if color == "white" {
        println!("SFEN: {:?}", sfen);
        view::display_sfen(sfen);
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


/* 

   ############################################################################################
   #################################### 2. PROMOTED PIECES ####################################
   ############################################################################################

 */


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


/* 
   
   ###########################################################################################
   ####################################### 3. MOBILITY #######################################
   ###########################################################################################

 */

pub fn mobility(sfen: &str, coord: &str) -> (usize, Vec<(PieceKind, Color)>) {

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
    let file = coord.chars().nth(0).unwrap() as u8 - b'A' + 1;
    let rank = coord.chars().nth(1).unwrap() as u8 - b'1' + 1;
    println!("file: {:?}", file);
    println!("rank: {:?}", rank);

    // Parse the SFEN string into a position
    let positions = SFEN::sfen_parse(sfen);// creates list of board squares and the pieces on them (if there are any)
    let mut pos = SFEN::generate_pos(positions); // creates a "partial position" out of it
    pos.side_to_move_set(SFEN::get_color(sfen)); // finalize the partial position

    // Find the rook's square based on the given file and rank
    let rook_square = shogi_core::Square::new(file, rank).expect("Invalid coordinate");

    // Print square index
    println!("rook sqr: {:?}", rook_square);

    // Get the Bitboard of possible rook moves from the LiteLegalityChecker
    let possible_moves = normal_from_candidates(&pos, rook_square);

    // Count the number of squares the rook can move to
    let num_moves = possible_moves.count() as usize;

    // Iterate over the possible moves and find the captured pieces
    let mut captured_pieces = Vec::<(PieceKind, Color)>::new();
    for to_square in possible_moves {
        if let Some(captured_piece) = pos.piece_at(to_square) {
            captured_pieces.push(captured_piece.to_parts());
        }
    }

    // Return the count of possible moves and the captured pieces
    (num_moves, captured_pieces)
}
 

/*
   
   ###########################################################################################
   ################################## 4. KING VULNERABILITY ##################################
   ###########################################################################################

 */


// Function to check if a piece can attack a given square
fn can_attack(pos: PartialPosition, piece: Piece, src: Square, dst: Square) -> bool {

    let mv = Move::Normal {
            from: src,
            to: dst,
            promote: false,
        };
    
    let legality_checker = LegalityChecker;
    let legal_move = legality_checker.is_legal_partial_lite(pos, mv);
    
    legal_move
}


// Function to find the attackers of a given square
fn attackers(
    pos: &PartialPosition,
    color: Color,
    square: Square,
) -> Vec<Square> {
    let mut result = Vec::new();

    for src_square in Square::ALL_SQUARES {
        if let Some(piece) = pos.get_piece(src_square) {
            if piece.color() == color && can_attack(pos, piece, src_square, square) {
                result.push(src_square);
            }
        }
    }

    result
}


pub fn enemy_king_vuln(sfen: &str, coord: &str) -> u32 {

/* Evaluate the 8 squares surrounding the King. A square is contributing positively to vulnerabilityif
   if it is being covered by a friendly piece. If an enemy can move to a square without being captured 
   it is not safe. Additionally, we need to know the King's escape routes. */

    const ATK_WEIGHT: u32 = 1;
    const DEF_WEIGHT: u32 = 1;
    const K_ATK_WEIGHT: u32 = 1;
    const ESC_WEIGHT: u32 = 1;
    

    // Parse the SFEN string into a position
    let positions = SFEN::sfen_parse(sfen);// creates list of board squares and the pieces on them (if there are any)
    let mut pos = SFEN::generate_pos(positions); // creates a "partial position" out of it
    let player = SFEN::get_color(sfen);
    pos.side_to_move_set(player); // finalize the partial position

    // Find the kings's square based on the given file and rank
    let king_square = pos.king_position(player);
    
    // Print square index
    println!("king sqr: {:?}", king_square);

    // Determine the color and enemy color based on the piece's case
    let (color, enemy_color) = if player == Color::White {
        ("WHITE", "BLACK")
    } else {
        ("BLACK", "WHITE")
    };

    // Construct the list of 8 squares that surround the king
    let file = coord.chars().nth(0).unwrap() as u8 - b'A' + 1; 
    let rank = coord.chars().nth(1).unwrap() as u8 - b'1' + 1;
    println!("file: {:?}", file);
    println!("rank: {:?}", rank);
    
    let files = (file - 1..=file + 1).filter(|&f| b'A' <= f && f <= b'I');
    let ranks = (rank - 1..=rank + 1).filter(|&r| 1 <= r && r <= 9);
    let squares: Vec<Square> = files
        .flat_map(|f| ranks.clone().map(move |r| Square::new(f,r)))
        .filter(|&s| s != king_square)
        .collect();

    // Calculate the number of pieces that can attack the squares surrounding the king
    let legality_checker = LiteLegalityChecker;
    let num_attackers = squares.iter().filter(|&s| {
        let attackers = attackers(&pos, color, *s);
        !attackers.is_empty()
    }).count() as u32;

    // Calculate the number of pieces that can defend the squares surrounding the king
    let num_defenders = squares.iter().filter(|&s| {
        let defenders = attackers(&pos, enemy_color, *s);
        !defenders.is_empty()
    }).count() as u32;

    // Calculate the number of pieces that can attack the king directly
    let num_king_attackers = attackers(&pos, enemy_color, king_square).len() as u32;

    // Calculate the number of escape routes the King has
    let num_escapes = legality_checker.all_legal_moves_partial(&pos).len() as u32;

    // Modify the values with internal weightings
    let king_vulnerability = (num_attackers * ATK_WEIGHT
        - num_defenders * DEF_WEIGHT
        + num_king_attackers * K_ATK_WEIGHT
        - num_escapes * ESC_WEIGHT)
        .max(0);

    king_vulnerability

}


