/* Russell Kosovsky, Jim O'Conner
 * Module containing the 6 eval functions
    * 1. Piece Square Tables
    * 2. Promoted Pieces
    * 3. King Vulnerability
    * 4. Rook Mobility
    * 5. Lance Mobility
    * 6. Biship Mobility
 */

use crate::view;
use crate::sfen as SFEN;
use std::collections::HashMap;

// ############################################################################################
// ################################## 1. PIECE SQUARE TABLES ##################################
// ############################################################################################

/*
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


pub fn evaluate_piece_table(mut sfen: &str, color: &str) {
    
    let mut white_score = 0;
    let mut black_score = 0;

    let pst_map = pst();
    if color == "black" {
       let sfen = SFEN::flip(sfen);
    }
    
    println!("SFEN: {:?}", sfen);
    view::display_sfen(sfen);

    let sfen_vec = pst_parse(sfen);
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
        }

        /*
        else if i == 'P' {
            let value = pst_map["P"][index];
            black_score += value;

        } else if i == 'L' {
            let value = pst_map["L"][index];
            black_score += value;

        } else if i == 'N' {
            let value = pst_map["N"][index];
            black_score += value;

        } else if i == 'S' {
            let value = pst_map["S"][index];
            black_score += value;

        } else if i == 'G' {
            let value = pst_map["G"][index];
            black_score += value;

        } else if i == 'R' {
            let value = pst_map["R"][index];
            black_score += value;

        } else if i == 'B' {
            let value = pst_map["B"][index];
            black_score += value;

        } else if i == 'E' {
            let value = pst_map["E"][index];
            black_score += value;

        } else if i == 'W' {
            let value = pst_map["W"][index];
            black_score += value;

        } else if i == 'Z' {
            let value = pst_map["G"][index];
            black_score += value;

        } else if i == 'X' {
            let value = pst_map["G"][index];
            black_score += value;

        } else if i == 'Y' {
            let value = pst_map["G"][index];
            black_score += value;

        } else if i == 'Q' {
            let value = pst_map["G"][index];
            black_score += value;
        
        }  */ 

        else if i == 'k' {

        } else if i == 'K' {
        
        }

        index += 1;
    }

    if color == "white" {
        println!("{:?}", white_score);
    } else if color == "black"{
        println!("{:?}", black_score);
    }
}


// ############################################################################################



// ############################################################################################
// #################################### 2. PROMOTED PIECES ####################################
// ############################################################################################

// ############################################################################################



// ###########################################################################################
// ################################## 3. KING VULNERABILITY ##################################
// ###########################################################################################

// ###########################################################################################



// ##########################################################################################
// #################################### 4. ROOK MOBILITY ####################################
// ##########################################################################################

// ##########################################################################################



// ###########################################################################################
// #################################### 5. LANCE MOBILITY ####################################
// ###########################################################################################

// ###########################################################################################



// ############################################################################################
// #################################### 6. BISHOP MOBILITY ####################################
// ############################################################################################

// ############################################################################################




