

use std::io;
use crate::eval;
use crate::view;
use crate::sfen;
use crate::tree::Tree;
use shogi_legality_lite::{normal_from_candidates, is_legal_partial_lite, all_legal_moves_partial};
use shogi_core::{PartialPosition, Square, Piece, Color, Move, PieceKind};


fn validate_user_move(user_move: &str) -> bool {
    // add validation logic
    // check that the input is formatted to represent a valid move in accordance with shogi_core.
    true
}


pub fn play() {

    println!(" | ");
    println!(" |---------------------------------WELCOME---------------------------------|");
    println!(" | ");
    println!(" | you are black and you are playing against the minimax algorithm");
    println!(" | in this game, squares are represented by their rank and file (rank, file)");
    println!(" | this means that your king would be in square: 'I,5'");
    println!(" | ranks are always a capital letter from A-I and files an integer from 1-9 ");
    println!(" | ");
    println!(" |-------------------------------------------------------------------------|");
    println!(" | ");
    
    let mut board = PartialPosition::startpos();
    let sfen = board.to_sfen_owned();
    //println!("sfen: {:?}", sfen);
    view::display_sfen(&sfen);

    println!(" | ");
    println!(" |-----------------------------Enter Your Move-----------------------------|");
    //println!(" | ");

    let mut user_move = String::new();
    match io::stdin().read_line(&mut user_move) {
        Ok(_) => {
            user_move = user_move.trim().to_string(); // Trim the input
            if validate_user_move(&user_move) { // Validate the input
                println!(" | Your Move: {:?}", user_move);
            } else {
                println!(" | Invalid move. Please enter a valid move in the format 'I,5'.");
            }
        },
        Err(e) => {
            println!(" | There was an error reading your input: {}. Please try again.", e);
        },
    
    }

    println!(" | {:?}", user_move);

}

