
// module for processing the opening book from the .tbk and .pat files
// 

use shogi_legality_lite::{normal_from_candidates, is_legal_partial_lite, all_legal_moves_partial};
use shogi_core::{PartialPosition, Square, Piece, Color, Move, PieceKind};



/*
 * BVerifyMove(s, mv, moveno)
 *
 * Compare the string 's' to the list of legal moves available for the
 * opponent. If a match is found, make the move on the board.
 */

pub fn BVerifyMove(s: &str, mv: Move, moveno: Move) {

}


/*
 * RESET()
 *
 * Reset the board and other variables to start a new game.
 *
 */

pub fn RESET() {

}


/*
 * GetOpenings()
 *
 * CHECKME: is this still valid esp. wrt gnushogi.book?
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
 * OpeningBook(hint, side)
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

