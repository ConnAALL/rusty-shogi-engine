
use crate::eval;
use crate::view;
use crate::sfen;
use crate::tree::Tree;
use shogi_core::Move;
use shogi_legality_lite::all_legal_moves_partial;
use std::collections::HashSet;
use std::collections::VecDeque;


#[derive(Debug)]
pub struct GameTree {
    
    pub sfen: String,
/*    sfen: String - This field contains a string representation of a game state in the 
 *    Shogi Forsyth-Edwards Notation (SFEN). The root of the tree represents the initial 
 *    game state, and each child represents a game state that can be reached by making a 
 *    legal move from the parent state.   */

    pub game_move: Option<Move>, // Optional field to store the move that led to this state
/*    game_move: Option<Move> - This field represents the move that was made to reach the 
 *    game state represented by this GameTree node from the parent node's game state. It 
 *    is stored as an Option<Move> because the root of the tree does not have a parent 
 *    and thus does not correspond to a move. In this case, game_move would be None. For all 
 *    other nodes, it should be Some(Move), where Move is the move made to reach this game state.   */
   

    pub children: Vec<GameTree>,
/*    children: Vec<GameTree> - This field is a vector containing all child nodes of this node 
 *    in the game tree. Each child represents a game state that can be reached by making one 
 *    legal move from the current game state.   */

}


// A constructor method for GameTree
impl GameTree {
    // This function takes an SFEN string and an optional Move and returns a new 
    // GameTree object with those values and an empty Vec for its children.
    pub fn new(sfen: String, game_move: Option<Move>) -> Self {
        GameTree {
            sfen,
            game_move,
            children: vec![],
        }
    }
}

// DepthFirstIter is a struct that allows for depth-first traversal of the GameTree.
pub struct DepthFirstIter<'a> {
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


// ##########################################################################################
//                                  MINIMAX W/NO EVAL

// essentially just a random eval function
use random_number::random;
pub fn randomize() -> (f32, f32) {
    let mut white_fitness: f32 = random!(1.0..20.0);
    let mut black_fitness: f32 = random!(1.0..20.0);
    return(white_fitness, black_fitness);
}


pub fn just_mini(tree: &GameTree, depth: u32, is_maximizing_player: bool) -> ((f32, f32), Option<Move>) {

    println!("\nENTERED JUST_MINI-----------------------------------------------------------------------------------");
    println!("depth as passed in: {:?}", depth);
    println!("is maximizing? : {:?}", is_maximizing_player);
    println!("sfen as passed in{:?}" ,tree.sfen);
    view::display_sfen(&tree.sfen);

    if depth == 0 || tree.children.is_empty() {
        println!("depth was zero and children vec was empty");

        return (randomize(), tree.game_move.clone());
    }

    if is_maximizing_player { // Assuming this is the white player
        println!("assuming white");
        let mut max_eval = (f32::MIN, f32::MIN);
        println!("max_eval: {:?}", max_eval);
        let mut best_move = None;
        println!("Then loops through children nodes and calls just_mini() again");
        for child in &tree.children {
            let (eval, move_) = just_mini(child, depth - 1, false);
            //let (eval, move_) = minimax(child, depth - 1, false);
            if eval.0 > max_eval.0 {
                max_eval = eval;
                best_move = move_;
            }
        }
        println!("returning...");
        return (max_eval, best_move);
    
    } else { // Assuming this is the black player
        println!("assuming black");
        let mut min_eval = (f32::MAX, f32::MAX);
        println!("min_eval: {:?}", min_eval);
        let mut best_move = None;
        println!("Then loops through children nodes and calls just_mini() again");
        for child in &tree.children {
            let (eval, move_) = just_mini(child, depth - 1, true);
            //let (eval, move_) = minimax(child, depth - 1, true);
            if eval.1 < min_eval.1 {
                min_eval = eval;
                best_move = move_;
            }
        }
        println!("returning...");
        return (min_eval, best_move);
    }
}


// ##########################################################################################
//                          `           MINIMAX

// SKELETON EXAMPLE: def doesnt work don evn try 
pub fn minimax(tree: &GameTree, depth: u32, is_maximizing_player: bool) -> ((f32, f32), Option<Move>) {
    if depth == 0 || tree.children.is_empty() {
        return (eval::evaluate(&tree.sfen), tree.game_move.clone());
    }

    if is_maximizing_player { // Assuming this is the white player
        let mut max_eval = (f32::MIN, f32::MIN);
        let mut best_move = None;
        for child in &tree.children {
            let (eval, move_) = minimax(child, depth - 1, false);
            if eval.0 > max_eval.0 {
                max_eval = eval;
                best_move = move_;
            }
        }
        return (max_eval, best_move);
    } else { // Assuming this is the black player
        let mut min_eval = (f32::MAX, f32::MAX);
        let mut best_move = None;
        for child in &tree.children {
            let (eval, move_) = minimax(child, depth - 1, true);
            if eval.1 < min_eval.1 {
                min_eval = eval;
                best_move = move_;
            }
        }
        return (min_eval, best_move);
    }
}


// ##########################################################################################
//                                  OLD SEARCH STUFF


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




