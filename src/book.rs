
// module for processing the opening book from the .tbk and .pat files
// 

use shogi_legality_lite::{normal_from_candidates, is_legal_partial_lite, all_legal_moves_partial};
use shogi_core::{PartialPosition, Square, Piece, Color, Move, PieceKind};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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

