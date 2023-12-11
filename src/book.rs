
// module for processing the opening book from the .tbk and .pat files

use shogi::piece;
use shogi_legality_lite::{normal_from_candidates, is_legal_partial_lite, all_legal_moves_partial};
use shogi_core::{PartialPosition, Square, Piece, Color, Move, PieceKind};
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead, BufReader};


// A helper function to convert piece shorthand into the `Piece` enum.
fn shorthand_to_piece(color: char, shorthand: char) -> Option<Piece> {
    match (color, shorthand) {
        ('B', 'P') => Some(Piece::B_P),
        ('B', 'L') => Some(Piece::B_L),
        ('B', 'N') => Some(Piece::B_N),
        ('B', 'S') => Some(Piece::B_S),
        ('B', 'G') => Some(Piece::B_G),
        ('B', 'B') => Some(Piece::B_B),
        ('B', 'R') => Some(Piece::B_R),
        ('W', 'P') => Some(Piece::W_P),
        ('W', 'L') => Some(Piece::W_L),
        ('W', 'N') => Some(Piece::W_N),
        ('W', 'S') => Some(Piece::W_S),
        ('W', 'G') => Some(Piece::W_G),
        ('W', 'B') => Some(Piece::W_B),
        ('W', 'R') => Some(Piece::W_R),
        _ => None,
    }
}


// A helper function to convert string "false"/"true" to boolean.
fn parse_bool(value: &str) -> Option<bool> { 
    match value {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}


// This function converts a string representation of a move into a `Move` object.
fn parse_move(move_str: &str) -> Option<Move> { 
    let parts: Vec<&str> = move_str.split(',').collect();
    match parts.as_slice() {
        //[color, from, to, promote, "false"] => {
        [color, from, to, promote, "false"] => {
            let from_value = from.parse::<u8>().ok()? + 1;
            let to_value = to.parse::<u8>().ok()? + 1;
            let from_square = Square::from_u8(from_value.checked_sub(1)?);
            let to_square = Square::from_u8(to_value.checked_sub(1)?);
            let promote = parse_bool(promote)?;
            Some(Move::Normal {
                from: from_square?,
                to: to_square?,
                promote,
            })
        },
        [color, piece_char, to, _, "true"] => {
            //let mut piece = shorthand_to_piece(color.chars().next()?, piece_char.chars().next()?)?;
            //println!("{:?}", piece_char.chars().next()?);
            let piece = Piece::W_P;
            let to_value = to.parse::<u8>().ok()? + 1;
            let to_square = Square::from_u8(to_value.checked_sub(1)? );
            Some(Move::Drop {
                piece,
                to: to_square?,
            })
        },
        _ => None,
    }
}


// This function reads a file and converts each line into a vector of `Move` objects.
fn read_openings<P: AsRef<Path>>(filename: P) -> io::Result<Vec<Vec<Move>>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    reader.lines().map(|line| {
        let line = line?;
        let moves_str = line.split(' ');
        let moves = moves_str.filter_map(parse_move).collect();
        Ok(moves)
    }).collect()
}


// function that tests the type conversions for openings from a file 
pub fn get_book_vec() -> io::Result<Vec<Vec<Move>>> {
    let openings = read_openings("/Users/russell/research/rusty-shogi-engine/src/formatted_openings.txt")?;
    Ok(openings)
}

pub fn display_book(book_vec: io::Result<Vec<Vec<Move>>>) {
    // display openings
    for opening in book_vec.unwrap() {
        // Now you can use the opening which is a Vec<Move>
        println!("{:?}", opening);
    }
}









/*
 * GetOpenings()
 *
 * Create a linked list of opening lines of play, with entry->next pointing to the
 * next line and entry->move pointing to a chunk of memory containing the
 * moves. More Opening lines of up to 100 half moves may be added to
 * gnushogi.book. But now it's a hashed table by position which yields a move
 * or moves for each position. It no longer knows about openings per se only
 * positions and recommended moves in those positions.
 *
 */


// function that opens a file and prints it to console line by line 
pub fn read_file_test(filepath: &str) {
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




//not used but could be helpful elsewhere
//
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

