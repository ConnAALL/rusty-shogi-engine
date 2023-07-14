

use crate::search;
use crate::eval;
use crate::view;
use crate::sfen as SFEN;
use std::collections::HashMap;
use shogi_legality_lite::{ normal_from_candidates, is_legal_partial_lite, 
                           all_legal_moves_partial, all_checks_partial};
use shogi_core::{ Bitboard, Color, IllegalMoveKind, Square, PartialPosition, 
                  Piece, PieceKind, PositionStatus, Move, LegalityChecker};


// Principal Variable Search Function
//pub fn pvs(mut state: PartialPosition, depth: i32, alpha: i32, beta: i32) -> (Option<Move>, f32) {
pub fn pvs(mut state: PartialPosition, depth: i32, alpha: i32, beta: i32) -> (String, f32) {
    let sfen = state.to_sfen_owned();
    
    if depth == 0 {
        let (white_fitness, black_fitness) = eval::evaluate(&sfen); 
        // return the evaluation of the board state
        if white_fitness > black_fitness {
            println!("Depth 0 reached, returning white_fitness: {}", white_fitness);
            return (state.to_sfen_owned(), white_fitness);
        } else {
            println!("Depth 0 reached, returning black_fitness: {}", black_fitness);
            return (state.to_sfen_owned(), black_fitness);
        }
    }

    let mut alpha = alpha;
    let (mut outcomes, mut moves) = search::single_search(&sfen); // generate all possible moves

    // Print out the moves returned by the single_search function.
    println!("Single search moves: {:?}", moves);

    let mut best_move = String::new();

    // Search the first move
    let first_move = moves.remove(0);
    println!("First move: {:?}", first_move);
    
    state.make_move(first_move.clone()); // apply the move to the game state
    let (first_move_result, mut score) = pvs(state.clone(), depth - 1, -beta, -alpha);
    println!("First move result: {:?}, Score: {}", first_move_result, score);

    // Search the remaining moves with a null window
    for mv in moves {
        state.make_move(mv.clone());
        
        let (move_result, mut temp_score) = pvs(state.clone(), depth - 1, -alpha-1, -alpha);
        println!("Move: {:?}, Result: {:?}, Score: {}", mv, move_result, temp_score);

        // if the score is greater than alpha, do a full re-search
        if temp_score > alpha as f32 && temp_score < beta as f32 {
            let (move_result, temp_score_inner) = pvs(state.clone(), depth - 1, -beta, -alpha);
            println!("Re-search Move: {:?}, Result: {:?}, Score: {}", mv, move_result, temp_score_inner);
            temp_score = temp_score_inner;
        }

        // update the score and alpha if necessary
        if temp_score > score {
            println!("New best score: {}, New best move: {:?}", temp_score, mv);
            score = temp_score;
            best_move = state.to_sfen_owned();
        }
        if score > alpha as f32 {
            println!("Alpha updated to: {}", score);
            alpha = score as i32;
        }

        // beta cutoff
        if alpha >= beta {
            println!("Alpha greater or equal to Beta, break");
            break;
        }
    }

    println!("Returning move: {:?}, score: {}", best_move, score);
    return (best_move.to_string(), score);
}




