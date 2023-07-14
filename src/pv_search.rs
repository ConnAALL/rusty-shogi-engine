

use crate::search;
use crate::eval;
use crate::view;
use crate::sfen as SFEN;
use std::collections::HashMap;
use shogi_legality_lite::{ normal_from_candidates, is_legal_partial_lite, 
                           all_legal_moves_partial, all_checks_partial};
use shogi_core::{ Bitboard, Color, IllegalMoveKind, Square, PartialPosition, 
                  Piece, PieceKind, PositionStatus, Move, LegalityChecker};


// The game state structure: Partial Position
//pub struct GameState

// The game move structure: Either shogi_core stuff or maybe USI stuff
//pub struct GameMove


// Principal Variable Search Function
fn pvs(state: PartialPosition, depth: i32, alpha: i32, beta: i32) -> f32 {

    let sfen = state.to_sfen_owned();
    
    if depth == 0 {
        let (white_fitness, black_fitness) = eval::evaluate(&sfen); 
        // return the evaluation of the board state
        if white_fitness > black_fitness {
            return white_fitness;
        } else {
            return black_fitness;
        }
    }

    let mut alpha = alpha;
    
    // might need to modify so that this stores the resulting move object as well as the sfen
    let mut moves = search::single_search(&sfen).1; // generate all possible moves

    // todo: Sort the moves according to some heuristic.

    // Search the first move
    let first_move = moves.remove(0);
    let new_state = state.make_move(first_move); // apply the move to the game state
    let mut score = -pvs(&new_state, depth - 1, -beta, -alpha);

    // Search the remaining moves with a null window
    for mv in moves {
        let new_state = state.make_move(mv); 
        let mut temp_score = -pvs(&new_state, depth - 1, -alpha-1, -alpha);
        
        // if the score is greater than alpha, do a full re-search
        if temp_score > alpha as f32 && temp_score < beta as f32 {
            temp_score = -pvs(&new_state, depth - 1, -beta, -alpha);
        }

        // update the score and alpha if necessary
        if temp_score > score {
            score = temp_score;
        }
        if score > alpha as f32 {
            alpha = score as i32;
        }

        // beta cutoff
        if alpha >= beta {
            break;
        }
    }

    return score;
}

