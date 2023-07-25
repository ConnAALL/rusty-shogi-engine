

use crate::eval;
use crate::view;
use crate::sfen;
use crate::search;
use shogi_legality_lite::{normal_from_candidates, is_legal_partial_lite, all_legal_moves_partial};
use shogi_core::{PartialPosition, Square, Piece, Color, Move, PieceKind};


pub struct UsiHandler {
    
    game: PartialPosition, // Your game state

}


impl UsiHandler {
    pub fn position(&mut self, sfen: &str) {
        let positions = sfen::sfen_parse(sfen);
        let mut pos = sfen::generate_pos(positions.clone());
        pos.side_to_move_set(sfen::get_color(sfen));
    }
}






