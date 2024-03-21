// Russell Kosovsky

use crate::eval;
use crate::sfen;
use crate::book;
use shogi_core::{Move, Color};
use shogi_legality_lite::all_legal_moves_partial;
use std::collections::HashSet;
use std::collections::VecDeque;


#[derive(Debug)]
pub struct GameTree {
    
    pub sfen: String,
/*    sfen: String - This field contains a string representation of 
 *    a game state in the Shogi Forsyth-Edwards Notation (SFEN). 
 *
 *    The root of the tree represents the initial game state
 *
 *    child represents a game state that can be reached by making 
 *    a legal move from the parent state.    */

    pub game_move: Option<Move>, // Optional field to store the move that led to this states
/*    game_move: Option<Move> - represents the move that was made to reach the 
 *    game state represented by this GameTree node from the parent node's game state. 
 *
 *  - It is stored as an Option<Move> because the root of the tree does not have a parent 
 *    and thus does not correspond to a move. In this case, game_move would be None. 
 *  - For all other nodes, it should be Some(Move), where Move is the move made to 
 *    reach this game state.   */

    pub children: Vec<GameTree>,
/*    children: Vec<GameTree> - This field is a vector containing all child 
 *    nodes of this node in the game tree. 
 *
 *    Each child represents a game state that can be reached by making one 
 *    legal move from the current game state.   */
}


impl GameTree { // GameTree struct implementation
    pub fn new(sfen: String, game_move: Option<Move>) -> Self {
        GameTree {
            sfen,
            game_move,
            children: vec![],
        }
    }
}

pub struct DepthFirstIter<'a> { // struct that allows for depth-first traversal of the GameTree.

    // uses a VecDeque to store references to GameTree nodes in the order they are to be visited.
    stack: VecDeque<&'a GameTree>,
}


impl<'a> DepthFirstIter<'a> {

    // The constructor function takes a reference to the root of the GameTree 
    // and returns a DepthFirstIter with the root as the first node to visit.
    pub fn new(root: &'a GameTree) -> Self {
        let mut stack = VecDeque::new();
        stack.push_back(root);
        DepthFirstIter { stack }
    }
}


impl<'a> Iterator for DepthFirstIter<'a> {
    type Item = &'a GameTree;
    // This function is the core of the depth-first traversal. It takes the last node 
    // added to the stack, adds all its children to the stack, and then returns the node.
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

// This implementation allows a GameTree to be iterated over directly with a for loop.
impl<'a> IntoIterator for &'a GameTree {
    type Item = &'a GameTree;
    type IntoIter = DepthFirstIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DepthFirstIter::new(self)
    }
}

// The treesearch function builds a GameTree by recursively exploring 
// all possible game states up to a specified depth.
pub fn treesearch(sfen: &str, depth: u32, current_depth: u32, game_move: Option<Move>) -> GameTree {

    // Parse the SFEN string and generate a PartialPosition from it
    let positions = sfen::sfen_parse(sfen);
    let mut pos = sfen::generate_pos(positions); 
    pos.side_to_move_set(sfen::get_color(sfen));

    // Create a new GameTree node with the current SFEN and move
    let mut game_tree = GameTree::new(sfen.to_string(), game_move);

    // If we haven't reached the maximum depth, generate all legal moves from the current position
    if current_depth < depth {
        let next_moves = all_legal_moves_partial(&pos);

        // For each legal move, create a new game state and add it as a child to the current node
        for move_item in next_moves {
            let mut temp_pos = pos.clone();
            temp_pos.make_move(move_item.clone());
            let sfen = temp_pos.to_sfen_owned();

            let child_tree = treesearch(&sfen, depth, current_depth + 1, Some(move_item));
            game_tree.children.push(child_tree);
        }
    }

    // Return the root of the constructed GameTree
    game_tree
}


pub fn minimax(tree: &GameTree, depth: u32, maximizing_player: Color) -> ((f32, f32), Option<Move>, Vec<(u32, u32)>, &str) {
    
    let curr_color = sfen::get_color(&tree.sfen);
    let mut best_sfen: &str = &"";
    let mut best_move = None;
    let mut best_features: Vec<(u32, u32)> = Vec::new();

    // BASE CASE if reaches max depth or if current node in game tree has no children (it's a terminal node) 
    // then evaluate current game state and return white/black scores
    if depth == 1 || tree.children.is_empty() {
        let eval = eval::evaluate3(&tree.sfen);
        return (eval.0, best_move, eval.1, eval.2);
    }

    // initialize best_eval, to store best evaluation value found so far. 
    // if current player is maximizing, start with the smallest possible value so any larger values will replace it. 
    // If current player is minimizing, start with the largest possible value so any smaller value will replace it.
    let mut best_eval = if curr_color == maximizing_player { (f32::MIN, f32::MIN) } else { (f32::MAX, f32::MAX) };

    // explore each child of the current node (each possible next state of the game)
    for child in &tree.children {

        // for each child, recursively call minimax3 to evaluate that child node. 
        // returns the pair of evaluation values, best move, and best features vector for the child node 
        // but here we are only interested in the evaluation and features, so we ignore the best move with _
        let (eval, _, features, sfen) = minimax(child, depth - 1, maximizing_player);

        // apply the maximization/minimization logic depending on whose turn it is. 
        // if current player is maximizing and current child's eval score is higher than the current best
        // or if current player is minimizing and current child's eval score is lower than the current best 
        // then update best evaluation, best move, and best features with the child's eval score, move, and features.
        if (maximizing_player == Color::Black && curr_color != maximizing_player && eval.0 < best_eval.0) ||
           (maximizing_player == Color::White && curr_color == maximizing_player && eval.0 > best_eval.0) ||
           (maximizing_player == Color::Black && curr_color == maximizing_player && eval.1 > best_eval.1) ||
           (maximizing_player == Color::White && curr_color != maximizing_player && eval.1 < best_eval.1) {
            best_sfen = sfen;
            best_eval = eval;
            best_move = child.game_move.clone();
            best_features.clear();
            best_features.extend(features);
        }
    }

    // after considering all the child nodes, return best evaluation, move, and features that were found
    return (best_eval, best_move, best_features, best_sfen);

}


pub fn get_book_move(tree: &GameTree, prev_moves: Vec<Move>) -> (Move, Vec<(u32, u32)>, &str) {
    
    let curr_color = Color::Black;
    let mut book_sfen: &str = &"";
    let mut book_move = None;
    let mut book_features: Vec<(u32, u32)> = Vec::new();


    //return book_move;
    return (book_move.unwrap(), book_features, book_sfen);


}


// ##########################################################################################
//                                  OLD SEARCH STUFF


//CALC BEST MOVE
//pub fn get_best_move(tree: &GameTree, depth: u32, maximizing_player: Color) -> ((f32, f32), Option<Move>, Vec<(u32, u32)>) {

  //  println!("######################### ENTERED GET_BEST_MOVE #########################");

    //let mut best_eval = (f32::MIN, f32::MIN);
    //let mut best_move = None;
    //let mut best_features: Vec<(u32, u32)> = Vec::new();
    
    //println!("");
    
    //for child in &tree.children {

      //  println!("best eval: {:?}", best_eval);
        //println!("child: {:?}", child.sfen);
        //let (eval, feature_vec) = minimax(child, depth, maximizing_player);
        //println!("eval: {:?}", eval);

        //if maximizing_player == Color::White {
            //println!("maxing white");
            //if eval.0 > best_eval.0 {
                //best_eval = eval;
                //best_move = child.game_move.clone();
                //best_features.clear();
                //best_features.extend(feature_vec);
            //}
        
        //} else {
            //println!("maxing black");
            //if eval.1 > best_eval.1 {
                //best_eval = eval;
                //best_move = child.game_move.clone();
                //best_features.clear();
                //best_features.extend(feature_vec);
            //}
        //}
    //}

    //return (best_eval, best_move, best_features);
//}


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

