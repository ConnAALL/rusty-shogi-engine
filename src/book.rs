
// module for processing the opening book from the .tbk and .pat files
// 

use shogi_legality_lite::{normal_from_candidates, is_legal_partial_lite, all_legal_moves_partial};
use shogi_core::{PartialPosition, Square, Piece, Color, Move, PieceKind};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn square_index(coord: &str) -> Option<Square> {
    match coord {
        "1a" => Square::from_u8(1),
        "2a" => Square::from_u8(10),
        "3a" => Square::from_u8(19),
        "4a" => Square::from_u8(28),
        "5a" => Square::from_u8(37),
        "6a" => Square::from_u8(46),
        "7a" => Square::from_u8(55),
        "8a" => Square::from_u8(64),
        "9a" => Square::from_u8(73),

        "1b" => Square::from_u8(2),
        "2b" => Square::from_u8(11),
        "3b" => Square::from_u8(20),
        "4b" => Square::from_u8(29),
        "5b" => Square::from_u8(38),
        "6b" => Square::from_u8(47),
        "7b" => Square::from_u8(56),
        "8b" => Square::from_u8(65),
        "9b" => Square::from_u8(74),

        "1c" => Square::from_u8(3),
        "2c" => Square::from_u8(12),
        "3c" => Square::from_u8(21),
        "4c" => Square::from_u8(30),
        "5c" => Square::from_u8(39),
        "6c" => Square::from_u8(48),
        "7c" => Square::from_u8(57),
        "8c" => Square::from_u8(66),
        "9c" => Square::from_u8(75),

        "1d" => Square::from_u8(4),
        "2d" => Square::from_u8(13),
        "3d" => Square::from_u8(22),
        "4d" => Square::from_u8(31),
        "5d" => Square::from_u8(40),
        "6d" => Square::from_u8(49),
        "7d" => Square::from_u8(58),
        "8d" => Square::from_u8(67),
        "9d" => Square::from_u8(76),

        "1e" => Square::from_u8(5),
        "2e" => Square::from_u8(14),
        "3e" => Square::from_u8(23),
        "4e" => Square::from_u8(32),
        "5e" => Square::from_u8(41),
        "6e" => Square::from_u8(50),
        "7e" => Square::from_u8(59),
        "8e" => Square::from_u8(68),
        "9e" => Square::from_u8(77),

        "1f" => Square::from_u8(6),
        "2f" => Square::from_u8(15),
        "3f" => Square::from_u8(24),
        "4f" => Square::from_u8(33),
        "5f" => Square::from_u8(42),
        "6f" => Square::from_u8(51),
        "7f" => Square::from_u8(60),
        "8f" => Square::from_u8(69),
        "9f" => Square::from_u8(78),

        "1g" => Square::from_u8(7),
        "2g" => Square::from_u8(16),    
        "3g" => Square::from_u8(25),
        "4g" => Square::from_u8(34),
        "5g" => Square::from_u8(43),
        "6g" => Square::from_u8(52),
        "7g" => Square::from_u8(61),
        "8g" => Square::from_u8(70),
        "9g" => Square::from_u8(79),

        "1h" => Square::from_u8(8),
        "2h" => Square::from_u8(17),
        "3g" => Square::from_u8(26),
        "4g" => Square::from_u8(35),
        "5g" => Square::from_u8(44),
        "6g" => Square::from_u8(53),
        "7g" => Square::from_u8(62),
        "8g" => Square::from_u8(71),
        "9g" => Square::from_u8(80),

        "1i" => Square::from_u8(9),
        "2i" => Square::from_u8(18),
        "3i" => Square::from_u8(27),
        "4i" => Square::from_u8(36),
        "5i" => Square::from_u8(45),
        "6i" => Square::from_u8(54),
        "7i" => Square::from_u8(63),
        "8i" => Square::from_u8(72),
        "9i" => Square::from_u8(81),

        _ => None, // or you can provide a more meaningful default or error message
    }
}

//pub fn sqr_test() {
//    let coordinate = "5A";
//    if let Some(index) = square_index(coordinate) {
//        println!("The square index for {} is {}", coordinate, index);
//    } else {
//        println!("Invalid coordinate: {}", coordinate);
//    }
//}

//fn parse_move(s: &str) -> Move {
    // If it contains '*', it's a drop
//    if s.contains('*') {
//        let parts: Vec<&str> = s.split('*').collect();
//        let piece = Piece::from_str(parts[0]).unwrap(); // Assuming a function to convert str to Piece
//        let to = Square::from_str(parts[1]).unwrap(); // Assuming a function to convert str to Square
//        Move::Drop { piece, to }
//    } else {
//        let piece_initial = &s[0..1];
//        let from = Square::from_str(&s[1..3]).unwrap();
//        let to = Square::from_str(&s[3..5]).unwrap();
//        let promote = s.ends_with('+');
//        Move::Normal { from, to, promote }
//    }
//}

//pub fn parse_move_test() {
    
//    let input = "P7f P8d S6h P3d S7g S6b"; // truncated for brevity
//    let moves: Vec<&str> = input.split_whitespace().collect();
//    let parsed_moves: Vec<Move> = moves.iter().map(|&m| parse_move(m)).collect();
    // Now, parsed_moves contains the list of shogi_core::Move objects.
//}



/*
 * GetOpenings()
 *
 * Read in the Opening Book file and parse the algebraic notation for a move
 * into an unsigned integer format indicating the from and to square. Create
 * a linked list of opening lines of play, with entry->next pointing to the
 * next line and entry->move pointing to a chunk of memory containing the
 * moves. More Opening lines of up to 100 half moves may be added to
 * gnushogi.book. But now it's a hashed table by position which yields a move
 * or moves for each position. It no longer knows about openings per se only
 * positions and recommended moves in those positions.
 *
 */

pub fn GetOpenings() {

}


/*
 * OpeningBook
 *
 * Go through each of the opening lines of play and check for a match with
 * the current game listing. If a match occurs, generate a random
 * number. If this number is the largest generated so far then the next
 * move in this line becomes the current "candidate".  After all lines are
 * checked, the candidate move is put at the top of the Tree[] array and
 * will be played by the program.  Note that the program does not handle
 * book transpositions.
 */

pub fn OpeningBook() {

}



// function that opens a file and prints it to console line by line 
pub fn test(filepath: &str) {
    let file = match File::open(filepath) {
        Ok(f) => f,
        Err(e) => {
            println!("Error opening file {}: {}", filepath, e);
            return;
        }
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(content) => println!("{}", content),
            Err(e) => println!("Error reading line: {}", e),
        }
    }
}

