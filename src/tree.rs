// Russell Kosovsky 7/17/23


use shogi_core::PartialPosition;

pub enum Tree<T: Ord + std::fmt::Display + Default> {
    Empty,
    Node {
        score: T,
        board: PartialPosition,
        sfen: String,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>
    },
}

impl<T: Ord + std::fmt::Display + Default> Tree<T> {
    
    pub fn new() -> Self {
        Tree::Empty
    }

    pub fn insert(&mut self, board: PartialPosition, sfen: String, score: T) {
        match self {
            Tree::Node { score: v, left, right, .. } => {
                if score < *v {
                    left.insert(board, sfen, score);
                } else {
                    right.insert(board, sfen, score);
                }
            },
            Tree::Empty => *self = Tree::Node {
                score: score,
                board: board,
                sfen: sfen,
                left: Box::new(Tree::Empty),
                right: Box::new(Tree::Empty),
            },
        }
    }

    pub fn search(&self, score: T) -> bool {
        match self {
            Tree::Node { score: v, left, right, .. } => {
                if *v == score {
                    true
                } else if score < *v {
                    left.search(score)
                } else {
                    right.search(score)
                }
            },
            Tree::Empty => false,
        }
    }

    pub fn print_in_order(&self) {
        match self {
            Tree::Node { score, left, right, .. } => {
                left.print_in_order();
                println!("{}", score);
                right.print_in_order();
            },
            Tree::Empty => (),
        }
    }
}





