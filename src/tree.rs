// Russell Kosovsky 7/17/23

use shogi_core::PartialPosition;

// Define a generic Tree enum that can hold a value of any type that implements
// the Ord, Display, and Default traits.
pub enum Tree<T: Ord + std::fmt::Display + Default> {
    Empty,  // Represents an empty tree
    Node {  // Represents a node in the tree
        score: T,  // The score value of the node
        board: PartialPosition,  // The partial position associated with the node
        sfen: String,  // The SFEN string representation of the game state
        children: Vec<Box<Tree<T>>>,  // The node's children, stored in a vector
    },
}

// Implement methods for the Tree enum
impl<T: Ord + std::fmt::Display + Default + Clone + PartialEq> Tree<T> {

    // Create a new, empty tree
    pub fn new() -> Self {
        Tree::Empty
    }

    // Insert a new node into the tree
    pub fn insert(&mut self, board: PartialPosition, sfen: String, score: T) {
        match self {
            // If the tree is not empty, add a new child node
            Tree::Node { score: _, children, .. } => {
                children.push(Box::new(Tree::Node {
                    score: score,
                    board: board,
                    sfen: sfen,
                    children: Vec::new(),
                }));
            },
            // If the tree is empty, make it a node
            Tree::Empty => {
                *self = Tree::Node {
                    score: score,
                    board: board,
                    sfen: sfen,
                    children: Vec::new(),
                }
            },
        }
    }

    // Check if a node with a given score exists in the tree
    pub fn search(&self, score: &T) -> bool {
        match self {
            // If the node's score is equal to the target score, return true
            Tree::Node { score: v, children, .. } => {
                if *v == score.clone() {
                    true
                } else {
                    // Otherwise, recursively search the children
                    children.iter().any(|child| child.search(score))
                }
            },
            // If the tree is empty, return false
            Tree::Empty => false,
        }
    }

    // Print the scores of all nodes in the tree
    pub fn print_tree(&self) {
        match self {
            Tree::Node { score, children, .. } => {
                for child in children {
                    child.print_tree();
                }
                println!("{}", score);
            },
            Tree::Empty => (),
        }
    }

    // Find the SFEN string of a node with a given score
    pub fn find_sfen(&self, score: &T) -> Option<String> {
        match self {
            Tree::Node { score: v, sfen, children, .. } => {
                if v == score {
                    // If the node's score is equal to the target score, return its SFEN string
                    Some(sfen.clone())
                } else {
                    // Otherwise, recursively search the children
                    for child in children {
                        if let Some(found_sfen) = child.find_sfen(score) {
                            return Some(found_sfen);
                        }
                    }
                    None
                }
            },
            // If the tree is empty, return None
            Tree::Empty => None,
        }
    }

    // Find the partial position of a node with a given score
    pub fn find_position(&self, score: &T) -> Option<PartialPosition> {
        match self {
            Tree::Node { score: v, board, children, .. } => {
                if v == score {
                    // If the node's score is equal to the target score, return its position
                    Some(board.clone())
                } else {
                    // Otherwise, recursively search the children
                    for child in children {
                        if let Some(found_pos) = child.find_position(score) {
                            return Some(found_pos);
                        }
                    }
                    None
                }
            },
            // If the tree is empty, return None
            Tree::Empty => None,
        }
    }
}



