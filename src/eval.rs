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
use shogi_legality_lite::{normal_from_candidates, is_legal_partial_lite, all_legal_moves_partial, all_checks_partial};
use shogi_core::{ Bitboard, Color, IllegalMoveKind, LegalityChecker, Move, PartialPosition, Piece, PieceKind, PositionStatus, Square};


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


pub fn enemy_king_vuln(sfen: &str, coord: &str) -> i32 {

    const ATK_WEIGHT: i32 = 1;
    const DEF_WEIGHT: i32 = 1;
    const K_ATK_WEIGHT: i32 = 1;
    const ESC_WEIGHT: i32 = 1;


    // Parse the SFEN string into a position
    let positions = SFEN::sfen_parse(sfen);
    let mut pos = SFEN::generate_pos(positions.clone());
    pos.side_to_move_set(SFEN::get_color(sfen));
    println!("Side to Move: {:?}", pos.side_to_move());


    // Find the king's square based on the given file and rank
    let file = coord.chars().next().unwrap() as u8 - b'A' + 1;
    let rank = coord.chars().nth(1).unwrap() as u8 - b'1' + 1;
    let king_square = Square::new(file, rank);
    //println!("file: {:?}", file);
    //println!("rank: {:?}", rank);
    println!("King's Square: {:?}", king_square);


    // Determine the color and enemy color based on the player's case
    let player = pos.side_to_move();
    let (color, enemy_color) = if player == Color::White {
        (Color::White, Color::Black)
    } else {
        (Color::Black, Color::White)
    };

    println!("Color: {:?}", color);
    println!("Enemy Color: {:?}", enemy_color);


    // Construct the list of 8 squares that surround the king
    println!("\n-------------Constructing the list of 8 squares that surround the king");
    let files = (file.saturating_sub(1)..=file.saturating_add(1)).filter(|&f| 1 <= f && f <= 9);
    let ranks = (rank.saturating_sub(1)..=rank.saturating_add(1)).filter(|&r| 1 <= r && r <= 9);
    //println!("files: {:?}", files.clone().collect::<Vec<_>>());
    //println!("ranks: {:?}", ranks.clone().collect::<Vec<_>>());
    let squares: Vec<Option<Square>> = files
        .flat_map(|f| ranks.clone().map(move |r| Square::new(f, r)))
        .filter(|&s| s != king_square)
        .collect();
    println!("SQUARES surrounding king: {:?}\n", squares);


    // Calculate the number of pieces that can attack the squares surrounding the king
    println!("\n-------------Calculateing the number of pieces that can attack the squares surrounding the king");
    pos.side_to_move_set(color);
    let num_attackers = squares.iter().filter(|&&s| {
        //println!("S: {:?}", s);
        let attackers = attackers(&pos, color, s);
        !attackers.is_empty()
    }).count() as i32;
    println!("num pieces that can attack the surrounding sqrs: {:?}\n", num_attackers);


    // Calculate the number of pieces that can defend the squares surrounding the king
    println!("\n-------------Calculateing the number of pieces that can defend the squares surrounding the king");
    pos.side_to_move_set(enemy_color);
    let mut total_defenders = Vec::<Option<Square>>::new();
    for s in squares {
        //println!("S: {:?}", s);
        let defenders = attackers(&pos, enemy_color, s);
        total_defenders.extend(defenders);   
    };
    //println!("Total defenders: {:?}", total_defenders);
    let num_defenders = total_defenders.len() as i32; 
    println!("num pieces that can defend the surrounding sqrs: {:?}\n", num_defenders);




    // Calculate the number of pieces attacking the king
    println!("\n-------------Calculateing the number of pieces attacking the king");
    println!("Side To Move: {:?}", color);
    pos.side_to_move_set(color);
    let enemy_king_sqr = pos.king_position(enemy_color);
    println!("enemy king square: {:?}", enemy_king_sqr);
    
    // change king to a pawn
    println!("changing king to pawn:");
    if enemy_color == Color::White {
        pos.piece_set(enemy_king_sqr.unwrap(), Some(Piece::W_P));
    } else {
        pos.piece_set(enemy_king_sqr.unwrap(), Some(Piece::B_P));
    }

    let next_moves = all_legal_moves_partial(&pos); 
    
    let mut attacks = Vec::new();

    for move_item in next_moves {
        
        if move_item.to() == enemy_king_sqr.unwrap() {
            attacks.push(move_item);
            println!("{:?}", move_item);
        }
    }

    // change back to king
    println!("changing back to king:");
    if enemy_color == Color::White {
        pos.piece_set(enemy_king_sqr.unwrap(), Some(Piece::W_K));
    } else {
        pos.piece_set(enemy_king_sqr.unwrap(), Some(Piece::B_K));
    }

    let num_king_attackers = attacks.len() as i32;
    println!("number of king attackers: {:?}", num_king_attackers);


    // Calculate the number of escape routes the king has
    println!("\n-------------Calculateing number of escape routes for the king");
    pos.side_to_move_set(enemy_color);
    let escapes = normal_from_candidates(&pos, king_square.clone().unwrap());
    //println!("escapes: {:?}", escapes);
    let num_escapes = escapes.count() as i32;
    println!("num escape routes for king: {:?}\n", num_escapes);

    println!(" ");


    // Modify the values with internal weightings
    let king_vulnerability = (num_attackers * ATK_WEIGHT
        - num_defenders * DEF_WEIGHT
        + num_king_attackers * K_ATK_WEIGHT
        - num_escapes * ESC_WEIGHT)
        .max(0);

    king_vulnerability

}



