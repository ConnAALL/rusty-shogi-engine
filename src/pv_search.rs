





// The game state structure: Partial Position
pub struct GameState

// The game move structure: Either shogi_core stuff or maybe USI stuff
pub struct GameMove


// Principal Variable Search Function
fn pvs(state: &GameState, depth: i32, alpha: i32, beta: i32) -> i32 {
    
    if depth == 0 {
        return state.evaluate(); // return the evaluation of the board state
    }

    let mut alpha = alpha;
    let mut moves = state.generate_moves(); // generate all possible moves

    // Sort the moves according to some heuristic.
    // For simplicity, assume that the moves are already sorted.

    // Search the first move
    let first_move = moves.remove(0);
    let new_state = state.make_move(&first_move); // apply the move to the game state
    let mut score = -pvs(&new_state, depth - 1, -beta, -alpha);

    // Search the remaining moves with a null window
    for mv in moves {
        let new_state = state.make_move(&mv); 
        let mut temp_score = -pvs(&new_state, depth - 1, -alpha-1, -alpha);
        
        // if the score is greater than alpha, do a full re-search
        if temp_score > alpha && temp_score < beta {
            temp_score = -pvs(&new_state, depth - 1, -beta, -alpha);
        }

        // update the score and alpha if necessary
        if temp_score > score {
            score = temp_score;
        }
        if score > alpha {
            alpha = score;
        }

        // beta cutoff
        if alpha >= beta {
            break;
        }
    }

    return score;
}

