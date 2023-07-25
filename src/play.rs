

use std::io;
use crate::eval;
use crate::view;
use crate::sfen;
use crate::tree::Tree;
use shogi_legality_lite::{normal_from_candidates, is_legal_partial_lite, all_legal_moves_partial};
use shogi_core::{PartialPosition, Square, Piece, Color, Move, PieceKind};


fn validate_user_move(user_input: &str) -> bool {
    // check that user input is the correct number of characters.
    if user_input.len() == 10 {
        true
    } else {
        false
    }
}


pub fn play() {

    println!(" | ");
    println!(" |---------------------------------WELCOME---------------------------------|");
    println!(" | ");
    println!(" | you are black and you are playing against the minimax algorithm");
    println!(" | in this game, squares are represented by their rank and file (rank, file)");
    println!(" | this means that your king would be in square: 'I,5'");
    println!(" | ranks are always a capital letter from A-I and files an integer from 1-9 ");
    println!(" | please enter your moves in the exact format as follows: 'G,9 to F,9'");
    println!(" | ");
    println!(" |-------------------------------------------------------------------------|");
    println!(" | ");
    
    let mut board = PartialPosition::startpos();
    let sfen = board.to_sfen_owned();
    //println!("sfen: {:?}", sfen);
    view::display_sfen(&sfen);

    let mut input = String::new();
    loop {
        println!(" | ");
        println!(" |-----------------------------Enter Your Move-----------------------------|");
        println!(" | ");
        
        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input = input.trim().to_string(); // Trim the input
                if validate_user_move(&input) { // Validate the input
                    break;
                } else {
                    println!(" | ");
                    println!(" | Please enter a valid move in the format: 'I,5 to H,5'.");
                }
            },
            Err(e) => {
                println!(" | ");
                println!(" | There was an error reading your input: {}. Please try again.", e);
                println!(" | ");
            },
    
        }
    }

    let moves: Vec<&str> = input.split(" to ").collect();
    let (mut from_sqr, mut to_sqr) = (moves[0], moves[1]);
    println!(" | ");
    println!(" | moving piece from square {} to square {}", from_sqr, to_sqr);
    println!(" | ");

    let from_sqr: Vec<&str> = from_sqr.split(",").collect();
    let (mut from_rank, mut from_file) = (from_sqr[0], from_sqr[1]);
    println!(" | from_rank: {}", from_rank);
    println!(" | from_file: {}", from_file);
    println!(" | ");
    
    let to_sqr: Vec<&str> = to_sqr.split(",").collect();
    let (mut to_rank, mut to_file) = (to_sqr[0], to_sqr[1]);
    println!(" | to_rank: {}", to_rank);
    println!(" | to_file: {}", to_file);
    println!(" | ");


   }

