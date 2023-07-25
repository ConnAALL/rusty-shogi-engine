use crate::eval;
use crate::view;
use crate::sfen;
use crate::tree::Tree;
use shogi_legality_lite::{normal_from_candidates, is_legal_partial_lite, all_legal_moves_partial};
use shogi_core::{PartialPosition, Square, Piece, Color, Move, PieceKind};


pub fn play() {


    println!(" |---------------------------------WELCOME---------------------------------|");
    println!(" | you are black and you are playing against the minimax algorithm");
    println!(" | in this game, squares are represented by their rank and file (rank, file)");
    println!(" | this means that your king would be in square: 'I,5'");
    println!(" | ranks are always a capital letter from A-I and files an integer from 1-9 ");
    println!(" |-------------------------------------------------------------------------|");
    println!(" |-----------------------------Enter Your Move-----------------------------|");

    let mut board = PartialPosition::startpos();
    let sfen = board.to_sfen_owned();
    //println!("sfen: {:?}", sfen);
    view::display_sfen(&sfen);



}

