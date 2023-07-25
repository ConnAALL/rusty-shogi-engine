use crate::eval;
use crate::view;
use crate::sfen;
use crate::tree::Tree;
use shogi_legality_lite::{normal_from_candidates, is_legal_partial_lite, all_legal_moves_partial};
use shogi_core::{PartialPosition, Square, Piece, Color, Move, PieceKind};


