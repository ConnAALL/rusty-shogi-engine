
use crate::sfen;
use shogi_legality_lite::all_legal_moves_partial;
use std::collections::HashSet;


pub fn search(sfen: &str, depth: u32) -> Vec<String> {
    // creates list of board squares and the pieces on them (if there are any)
    let positions = sfen::sfen_parse(sfen);
    // creates a "partial position" out of it
    let pos = sfen::generate_pos(positions);
    
    if depth == 0 {
        return Vec::new();
    }
    
    // creates list of all possible moves
    let next_moves = all_legal_moves_partial(&pos);
    let mut sfen_list = Vec::new();
    
    for move_item in next_moves {
        let mut temp_pos = pos.clone();
        temp_pos.make_move(move_item);
        let sfen = temp_pos.to_sfen_owned();
        // flips sfen so that it becomes the next players turn
        let flipped_sfen = sfen::flip(&sfen);
        sfen_list.push(flipped_sfen.clone());
        
        let deeper_moves = search(&flipped_sfen, depth - 1);
        sfen_list.extend(deeper_moves);
    }

    sfen_list
}

pub fn has_duplicates<T: std::cmp::Eq + std::hash::Hash>(vec: &[T]) -> bool {
    let set: HashSet<_> = vec.iter().collect();
    set.len() != vec.len()
}

pub fn perft(sfen: &str, depth: u32) -> u64 {
    let positions = sfen::sfen_parse(sfen);
    let pos = sfen::generate_pos(positions);

    if depth == 0 {
        return 1;
    }

    let next_moves = all_legal_moves_partial(&pos);
    let mut node_count = 0;

    for move_item in next_moves {
        let mut temp_pos = pos.clone();
        temp_pos.make_move(move_item);
        let sfen = temp_pos.to_sfen_owned();
        let flipped_sfen = sfen::flip(&sfen);

        let child_count = perft(&flipped_sfen, depth - 1);
        node_count += child_count;
    }

    node_count
}

/*

pub fn old_search(sfen: &str) -> Vec<String> {
    
    let positions = sfen::sfen_parse(sfen);
    let pos = sfen::generate_pos(positions);
    let next_moves = all_legal_moves_partial(&pos); 
    let mut sfen_list = Vec::new();
    
    for move_item in next_moves {
        let mut temp_pos = pos.clone();
        temp_pos.make_move(move_item);
        let sfen = temp_pos.to_sfen_owned();
        sfen_list.push(sfen);
    }

    sfen_list
}


pub fn search_dep(depth: i32, sfen: &str) -> Vec<String> {
    
    let mut parent = Vec::new();
    parent.push(sfen.to_string());
    
    let mut final_result = Vec::new();
    for dep in 1..=depth {
        let mut next_sfen_list = Vec::new();
        for sfen in &parent {
            let moves = old_search(sfen);
            next_sfen_list.extend(moves);
        }
        final_result.extend(next_sfen_list.clone());
        parent = next_sfen_list;

    }

    final_result
}

*/
