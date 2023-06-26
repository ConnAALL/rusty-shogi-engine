/* Russell Kosovsky, Jim O'Conner
 * Module containing the 6 eval functions
    * 1. Piece Square Tables
    * 2. Promoted Pieces
    * 3. King Vulnerability
    * 4. Rook Mobility
    * 5. Lance Mobility
    * 6. Biship Mobility
 */

use crate::sfen;


// #################################### 1. PST ####################################

/*
REFERENCE FOR PROMOTED PIECES
    +P => Z
    +p => z
    
    +L => X
    +l => x
    
    +N => Y
    +n => y

    +S => Q
    +s => q

    +B => W
    +b => w

    +R => E
    +r => e
*/


pub fn pst_parse(sfen: &str) -> Vec<char> {

    let split: Vec<&str> = sfen.split_whitespace().collect();
    let pieces = split[0];
    // println!("{:?}", pieces);
    let replaced = pieces.replace("/", "");
    println!("{:?}", replaced);
    
    let clean = sfen::convert_promoted_pieces(&replaced);
    println!("{:?}", clean);

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





// ################################################################################



// #################################### 2. PROMOTED PIECES ####################################

// ############################################################################################



// #################################### 3. KING VULNERABILITY ####################################

// ###############################################################################################



// #################################### 4. ROOK MOBILITY ####################################

// ##########################################################################################



// #################################### 5. LANCE MOBILITY ####################################

// ###########################################################################################



// #################################### 6. BISHOP MOBILITY ####################################

// ############################################################################################




