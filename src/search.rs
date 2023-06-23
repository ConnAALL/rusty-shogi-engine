
use crate::sfen;
//use crate::view;
use shogi_legality_lite::all_legal_moves_partial;
use std::collections::HashSet;

pub fn search(sfen: &str, depth: u32) -> Vec<String> {
    let positions = sfen::sfen_parse(sfen); // creates list of board squares and the pieces on them (if there are any)
    let pos = sfen::generate_pos(positions); // creates a "partial position" out of it
    
    if depth == 0 {
        return Vec::new(); // End of search, return an empty vector
    }
    
    let next_moves = all_legal_moves_partial(&pos); // creates list of all possible moves
    let mut sfen_list = Vec::new(); // Vector to store SFEN strings
    
    for move_item in next_moves {
        let mut temp_pos = pos.clone(); // Create a temporary copy of the position
        temp_pos.make_move(move_item); // Make the move on the temporary position
        let sfen = temp_pos.to_sfen_owned(); // Convert the updated position to an SFEN string
        sfen_list.push(sfen.clone()); // Add the SFEN string to the vector
        let flipped_sfen = sfen::flip(&sfen); // Flip the SFEN string
        let deeper_moves = search(&flipped_sfen, depth - 1); // Recursively search deeper with decreased depth
        sfen_list.extend(deeper_moves); // Add the deeper moves to the vector
    }

    sfen_list // Return the vector of SFEN strings
}


pub fn has_duplicates<T: std::cmp::Eq + std::hash::Hash>(vec: &[T]) -> bool {
    let set: HashSet<_> = vec.iter().collect(); // Create a HashSet from the elements of the slice
    set.len() != vec.len() // Check if the lengths are different (indicating duplicates)
}


pub fn perft(sfen: &str, depth: u32) -> u64 {
    let positions = sfen::sfen_parse(sfen); // Parse the SFEN string into a list of positions
    let pos = sfen::generate_pos(positions); // Generate a "partial position" from the positions

    if depth == 0 {
        return 1; // End of search, return 1 as a leaf node
    }

    let next_moves = all_legal_moves_partial(&pos); // Get all possible moves for the current position
    let mut node_count = 0; // Counter for the number of nodes

    for move_item in next_moves {
        let mut temp_pos = pos.clone(); // Create a temporary copy of the position
        temp_pos.make_move(move_item); // Make the move on the temporary position
        let sfen = temp_pos.to_sfen_owned(); // Convert the updated position to an SFEN string
        let flipped_sfen = sfen::flip(&sfen); // Flip the SFEN string

        let child_count = perft(&flipped_sfen, depth - 1); // Recursively calculate child nodes count
        node_count += child_count; // Add child count to the node count
    }

    node_count // Return the total node count
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
