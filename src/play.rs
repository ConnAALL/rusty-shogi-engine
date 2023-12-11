// Russell Kosovsky

use std::io;
use crate::book;
use crate::eval;
use crate::view;
use crate::sfen;
use crate::search;
use crate::tree::Tree;
use shogi_legality_lite::{normal_from_candidates, is_legal_partial_lite, all_legal_moves_partial, status_partial};
use shogi_core::{PartialPosition, PositionStatus, Square, Piece, Color, Move, PieceKind};


fn validate_user_move(user_input: &str) -> bool {
    // check that user input is the correct number of characters.
    if user_input.len() == 10 || user_input.len() == 15 {
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
    //println!(" | INPUT: {:?}", moves);
    let (mut from_sqr, mut to_sqr) = (moves[0], moves[1]);
    let mut promote = false;
    if moves.len() == 3 {
        if moves[2] == "P" {
            promote = true;
        }
    }

    println!(" | ");
    println!(" | moving piece from square {} to square {}", from_sqr, to_sqr);
    println!(" | promote? {:?}", promote);
    println!(" | ");

    let from_sqr: Vec<&str> = from_sqr.split(",").collect();
    let (mut from_rank, mut from_file) = (from_sqr[0], from_sqr[1]);
    let from_rank: char = from_rank.chars().next().unwrap();
    let from_rank: u8 = char_to_u8(from_rank).unwrap();
    let from_file = from_file.parse::<u8>().unwrap();
    let from_square: Square = Square::new(from_file, from_rank).unwrap();
    //println!(" | from_rank as u8: {:?}", from_rank);
    //println!(" | from_file (u8): {:?}", from_file);
    //println!(" | from_square: {:?}", from_square);
    //println!(" | ");

    let to_sqr: Vec<&str> = to_sqr.split(",").collect();
    let (mut to_rank, mut to_file) = (to_sqr[0], to_sqr[1]);
    let to_rank: char = to_rank.chars().next().unwrap();
    let to_rank: u8 = char_to_u8(to_rank).unwrap();
    let to_file = to_file.parse::<u8>().unwrap();
    let to_square: Square = Square::new(to_file, to_rank).unwrap();
    //println!(" | to_rank as u8: {:?}", to_rank);
    //println!(" | to_file (u8): {:?}", to_file);
    //println!(" | to_square: {:?}", to_square);
    //println!(" | ");

    let mut user_move: Move = Move::Normal {from: from_square, to: to_square, promote: false};
    if promote {
        user_move = Move::Normal {from: from_square, to: to_square, promote: true};
    } else {
        user_move = Move::Normal {from: from_square, to: to_square, promote: false};
    }

    //println!(" | move object: {:?}", user_move);
    
    user_move

}


fn computer_book_move(past_mvs: Vec<Move>, openings: Vec<Vec<Move>>) -> Move {
   
    let mut opening_match: Vec<Vec<Move>> = Vec::new();

    for mv in past_mvs.iter() {
        //println!("{:?}", mv);
        for opening in openings.iter() {
            for (i, opening_mv) in opening.iter().enumerate() {
                if mv == opening_mv {
                    opening_match.push(opening.clone());
                }
                break
            }
        }
    }
    
    //println!("opening match: {:?}", opening_match);
    //println!();
    //println!("{:?}", past_mvs.len());

    let book_move = opening_match[0][past_mvs.len()].clone();

    println!(" | book move: {:?}", book_move);

    book_move

}


pub fn play_book() {

    println!("");
    println!(" |---------------------------------WELCOME---------------------------------|");
    println!(" | ");
    println!(" | you are playing as white and you are playing against the minimax algorithm");
    println!(" | in this game, squares are represented by their rank and file (rank, file)");
    println!(" | this means that your king would be in square: 'I,5'");
    println!(" | ranks are always a capital letter from A-I and files an integer from 1-9 ");
    println!(" | please enter your moves in the exact format as follows: 'G,9 to F,9'");
    println!(" | to promote a piece, format your input like this -> 'D,4 to C,4 to P'");
    println!(" | ");

    println!(" |-------------------------------------------------------------------------|");
    println!(" | ");

    let mut board = PartialPosition::startpos();
    let mut sfen = board.to_sfen_owned();
    //println!("sfen: {:?}", sfen);
    view::display_sfen(&sfen);

    let mut past_moves: Vec<Move> = Vec::new();

    let book_vec = book::get_book_vec().unwrap();

    // main game loop
    loop {
        let human_mv = human_move();
        past_moves.push(human_mv);
        
        if shogi_legality_lite::is_legal_partial_lite(&board, human_mv) { // check if the human move is legal
            board.make_move(human_mv);
            
            sfen = board.to_sfen_owned();
            println!(" | ");
            view::display_sfen(&sfen);

            // game end condition
            if shogi_legality_lite::status_partial(&board) == PositionStatus::BlackWins { // check if it's checkmate
                println!("Congratulations! You won.");
                break;
            } else if shogi_legality_lite::status_partial(&board) == PositionStatus::Draw { // check if it's stalemate
                println!("Game is a draw.");
                break;
            }

            println!(" | ");
            println!(" |------------------------------COMPUTER MOVE------------------------------|");
            println!(" | ");
            println!(" | thinking...");
            println!(" | ");
            
            let computer_mv = computer_book_move(past_moves.clone(), book_vec.clone());

            board.make_move(computer_mv);
            sfen = board.to_sfen_owned(); 
            view::display_sfen(&sfen);
            println!("{:?}", sfen);

            past_moves.push(computer_mv);

            // game end condition
            if shogi_legality_lite::status_partial(&board) == PositionStatus::WhiteWins {
                println!("Congratulations! You won.");
                break;
            } else if shogi_legality_lite::status_partial(&board) == PositionStatus::Draw {
                println!("Game is a draw.");
                break;
            }

        } else {
            println!("Illegal move, please try again.");
        }
    }
}




/////////////////////////////////// OG PLAY FUNCTION /////////////////////////////////////////

fn computer_move_OG(root_sfen: &str) -> Move {

    let dep = 3;
    let color = sfen::get_color(&root_sfen);
    
    let root = search::treesearch(&root_sfen, dep, 1, None); // Create the root GameTree node

    //let ((white_score, black_score), best_move, best_features) = search::get_best_move(&root, dep, color);
    let ((white_score, black_score), best_move, best_features, best_sfen) = search::minimax(&root, dep, color);

    let (white_promoted_pieces, black_promoted_pieces) = best_features[0];
    let (white_pst, black_pst) = best_features[1];
    let (white_king_vln, black_king_vln) = best_features[2];
    let (white_rook_mobil, black_rook_mobil) = best_features[3];
    let (white_lance_mobil, black_lance_mobil) = best_features[4];
    let (white_bish_mobil, black_bish_mobil) = best_features[5];
    let (white_hand, black_hand) = best_features[6];

    println!(" | best move: {:?}", best_move);
    println!(" | ");
    println!(" | best sfen: {:?}", best_sfen);
    view::display_sfen(best_sfen);
    println!(" | ");
    println!(" | white_score: {:?}", white_score);
    println!(" | black_score: {:?}", black_score);
    println!(" | feature variate values: ");
    println!(" |    |WHITE|");
    println!(" | white_promoted_pieces: {:?}", white_promoted_pieces);
    println!(" | white_pst: {:?}", white_pst);
    println!(" | white_king_vln: {:?}", white_king_vln);
    println!(" | white_rook_mobil: {:?}", white_rook_mobil);
    println!(" | white_lance_mobil: {:?}", white_lance_mobil);
    println!(" | white_bish_mobil: {:?}", white_bish_mobil);
    println!(" | white_hand: {:?}", white_hand);
    println!(" | ");
    println!(" |    |BLACK|");
    println!(" | black_promoted_pieces: {:?}", black_promoted_pieces);
    println!(" | black_pst: {:?}", black_pst);
    println!(" | black_king_vln: {:?}", black_king_vln);
    println!(" | black_rook_mobil: {:?}", black_rook_mobil);
    println!(" | black_lance_mobil: {:?}", black_lance_mobil);
    println!(" | black_bish_mobil: {:?}", black_bish_mobil);
    println!(" | black_hand: {:?}", black_hand);
    println!(" | ");

    best_move.unwrap()

}


pub fn play_OG() {

    println!("");
    println!(" |---------------------------------WELCOME---------------------------------|");
    println!(" | ");
    println!(" | you are black and you are playing against the minimax algorithm");
    println!(" | in this game, squares are represented by their rank and file (rank, file)");
    println!(" | this means that your king would be in square: 'I,5'");
    println!(" | ranks are always a capital letter from A-I and files an integer from 1-9 ");
    println!(" | please enter your moves in the exact format as follows: 'G,9 to F,9'");
    println!(" | to promote a piece, format your input like this -> 'D,4 to C,4 to P'");
    println!(" | ");

    println!(" |-------------------------------------------------------------------------|");
    println!(" | ");

    let mut board = PartialPosition::startpos();
    let mut sfen = board.to_sfen_owned();
    //println!("sfen: {:?}", sfen);
    view::display_sfen(&sfen);

    // main game loop
    loop {
        let human_mv = human_move();
        
        if shogi_legality_lite::is_legal_partial_lite(&board, human_mv) { // check if the human move is legal
            board.make_move(human_mv);
            
            sfen = board.to_sfen_owned();
            println!(" | ");
            view::display_sfen(&sfen);

            // game end condition
            if shogi_legality_lite::status_partial(&board) == PositionStatus::BlackWins { // check if it's checkmate
                println!("Congratulations! You won.");
                break;
            } else if shogi_legality_lite::status_partial(&board) == PositionStatus::Draw { // check if it's stalemate
                println!("Game is a draw.");
                break;
            }

            println!(" | ");
            println!(" |------------------------------COMPUTER MOVE------------------------------|");
            println!(" | ");
            println!(" | thinking...");
            println!(" | ");
            
            let computer_mv = computer_move_OG(&sfen);
            board.make_move(computer_mv);
            sfen = board.to_sfen_owned(); 
            view::display_sfen(&sfen);
            println!("{:?}", sfen);

            // game end condition
            if shogi_legality_lite::status_partial(&board) == PositionStatus::WhiteWins {
                println!("Congratulations! You won.");
                break;
            } else if shogi_legality_lite::status_partial(&board) == PositionStatus::Draw {
                println!("Game is a draw.");
                break;
            }

        } else {
            println!("Illegal move, please try again.");
        }
    }
}








pub fn play_one_move() {

    println!("");
    println!(" |---------------------------------WELCOME---------------------------------|");
    println!(" | ");
    println!(" | you are black and you are playing against the minimax algorithm");
    println!(" | in this game, squares are represented by their rank and file (rank, file)");
    println!(" | this means that your king would be in square: 'I,5'");
    println!(" | ranks are always a capital letter from A-I and files an integer from 1-9 ");
    println!(" | please enter your moves in the exact format as follows: 'G,9 to F,9'");
    println!(" | to promote a piece, format your input like this -> 'D,4 to C,4 to P'");
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
    
    //let computer_mv = computer_move(&sfen);
    
    //println!(" | ");
    //println!(" |------------------------------COMPUTER MOVE------------------------------|");
    //println!(" | ");
    //println!(" | move: {:?}", computer_mv);
    
    //board.make_move(computer_mv);
    //sfen = board.to_sfen_owned(); 

    //println!(" | ");
    //println!(" |------------------------------CURRENT BOARD------------------------------|");
    //view::display_sfen(&sfen);

    println!(" |-------------------------------------------------------------------------|");
   
}

