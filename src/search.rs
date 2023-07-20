
use crate::eval;
use crate::view;
use crate::sfen;
use crate::tree::Tree;
use shogi_core::Move;
use shogi_legality_lite::all_legal_moves_partial;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn search(sfen: &str, depth: u32, current_depth: u32) -> Vec<String> {
    let positions = sfen::sfen_parse(sfen);// creates list of board squares and the pieces on them (if there are any)
    let mut pos = sfen::generate_pos(positions); // creates a "partial position" out of it
    pos.side_to_move_set(sfen::get_color(sfen));

    if current_depth == depth {
        return vec![sfen.to_string()]; // Return the SFEN string as a single-element vector
    }

    let next_moves = all_legal_moves_partial(&pos);
    let mut sfen_list = Vec::new();

    for move_item in next_moves {
        let mut temp_pos = pos.clone();
        temp_pos.make_move(move_item);
        let sfen = temp_pos.to_sfen_owned();

        let deeper_moves = search(&sfen, depth, current_depth + 1);
        sfen_list.extend(deeper_moves);
    }

    sfen_list
}


pub fn has_duplicates<T: std::cmp::Eq + std::hash::Hash>(vec: &[T]) -> bool {
    let set: HashSet<_> = vec.iter().collect(); // Create a HashSet from the elements of the slice
    set.len() != vec.len() // Check if the lengths are different (indicating duplicates)
}


pub fn perft(sfen: &str, depth: u32) -> u64 {
    let positions = sfen::sfen_parse(sfen); // Parse the SFEN string into a list of positions
    let mut pos = sfen::generate_pos(positions); // Generate a "partial position" from the positions
    pos.side_to_move_set(sfen::get_color(sfen));

    if depth == 0 {
        return 1; // End of search, return 1 as a leaf node
    }

    let next_moves = all_legal_moves_partial(&pos); // Get all possible moves for the current position
    let mut node_count = 0; // Counter for the number of nodes

    for move_item in next_moves {
        let mut temp_pos = pos.clone(); // Create a temporary copy of the position
        temp_pos.make_move(move_item); // Make the move on the temporary position
        let sfen = temp_pos.to_sfen_owned(); // Convert the updated position to an SFEN string

        let child_count = perft(&sfen, depth - 1); // Recursively calculate child nodes count
        node_count += child_count; // Add child count to the node count
    }

    node_count // Return the total node count
}


pub fn single_search(sfen: &str) -> (Vec<String>, Vec<Move>) {
    
    let positions = sfen::sfen_parse(sfen);
    let mut pos = sfen::generate_pos(positions);
    pos.side_to_move_set(sfen::get_color(sfen));
    let next_moves = all_legal_moves_partial(&pos); 
    let mut sfen_list = Vec::new();
    let mut move_list = Vec::new();
    
    for move_item in next_moves {
        move_list.push(move_item);
        let mut temp_pos = pos.clone();
        temp_pos.make_move(move_item);
        let sfen = temp_pos.to_sfen_owned();
        sfen_list.push(sfen);
    }

    return (sfen_list, move_list);
}


pub struct DepthFirstIter<'a> {
    stack: VecDeque<&'a GameTree>,
}


impl<'a> DepthFirstIter<'a> {
    pub fn new(root: &'a GameTree) -> Self {
        let mut stack = VecDeque::new();
        stack.push_back(root);
        DepthFirstIter { stack }
    }
}


impl<'a> Iterator for DepthFirstIter<'a> {
    type Item = &'a GameTree;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop_back();
        if let Some(node) = node {
            for child in &node.children {
                self.stack.push_back(child);
            }
        }
        node
    }
}


impl<'a> IntoIterator for &'a GameTree {
    type Item = &'a GameTree;
    type IntoIter = DepthFirstIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DepthFirstIter::new(self)
    }
}


#[derive(Debug)]
pub struct GameTree {
    sfen: String,
    game_move: Option<String>, // Optional field to store the move that led to this state
    children: Vec<GameTree>,
}


impl GameTree {
    pub fn new(sfen: String, game_move: Option<String>) -> Self {
        GameTree {
            sfen,
            game_move,
            children: vec![],
        }
    }
}


pub fn treesearch(sfen: &str, depth: u32, current_depth: u32) -> GameTree {
    let positions = sfen::sfen_parse(sfen);
    let mut pos = sfen::generate_pos(positions); 
    pos.side_to_move_set(sfen::get_color(sfen));

    let mut game_tree = GameTree::new(sfen.to_string(), None);

    if current_depth < depth {
        let next_moves = all_legal_moves_partial(&pos);

        for move_item in next_moves {
            let mut temp_pos = pos.clone();
            temp_pos.make_move(move_item.clone());
            let sfen = temp_pos.to_sfen_owned();

            let child_tree = treesearch(&sfen, depth, current_depth + 1);
            game_tree.children.push(child_tree);
        }
    }

    game_tree
}
