

use std::io;
use crate::eval;
use crate::view;
use crate::sfen;
use crate::search;
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


fn char_to_u8(input: char) -> Option<u8> {
    match input {
        'A' => Some(1),
        'B' => Some(2),
        'C' => Some(3),
        'D' => Some(4),
        'E' => Some(5),
        'F' => Some(6),
        'G' => Some(7),
        'H' => Some(8),
        'I' => Some(9),
        _ => None,
    }
}


fn human_move() -> Move {

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
    let from_rank: char = from_rank.chars().next().unwrap();
    let from_rank: u8 = char_to_u8(from_rank).unwrap();
    let from_file = from_file.parse::<u8>().unwrap();
    let from_square: Square = Square::new(from_file, from_rank).unwrap();
    println!(" | from_rank as u8: {:?}", from_rank);
    println!(" | from_file (u8): {:?}", from_file);
    println!(" | from_square: {:?}", from_square);
    println!(" | ");

    let to_sqr: Vec<&str> = to_sqr.split(",").collect();
    let (mut to_rank, mut to_file) = (to_sqr[0], to_sqr[1]);
    let to_rank: char = to_rank.chars().next().unwrap();
    let to_rank: u8 = char_to_u8(to_rank).unwrap();
    let to_file = to_file.parse::<u8>().unwrap();
    let to_square: Square = Square::new(to_file, to_rank).unwrap();
    println!(" | to_rank as u8: {:?}", to_rank);
    println!(" | to_file (u8): {:?}", to_file);
    println!(" | to_square: {:?}", to_square);
    println!(" | ");

    let user_move = Move::Normal {from: from_square, to: to_square, promote: false};
    println!(" | move object: {:?}", user_move);

    user_move

    
}


fn computer_move(root_sfen: &str) -> Move {


    let dep = 2;
    let color = sfen::get_color(&root_sfen);
    
    let root = search::treesearch(&root_sfen, dep, 0, None); // Create the root GameTree node

    let ((white_score, black_score), best_move) = search::get_best_move(&root, dep, color); 

    best_move.unwrap()

}


pub fn play() {

    println!("");
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
    let mut sfen = board.to_sfen_owned();
    //println!("sfen: {:?}", sfen);
    view::display_sfen(&sfen);

    let human_mv = human_move();
    
    board.make_move(human_mv);
    sfen = board.to_sfen_owned();
    
    println!(" | ");
    println!(" |------------------------------CURRENT BOARD------------------------------|");
    view::display_sfen(&sfen);
    
    let computer_mv = computer_move(&sfen);
    
    println!(" | ");
    println!(" |------------------------------COMPUTER MOVE------------------------------|");
    println!(" | ");
    println!(" | move: {:?}", computer_mv);
    
    board.make_move(computer_mv);
    sfen = board.to_sfen_owned(); 

    println!(" | ");
    println!(" |------------------------------CURRENT BOARD------------------------------|");
    view::display_sfen(&sfen);
    

    println!(" |-------------------------------------------------------------------------|");
   
}

