// Russell Kosovsky 7/17/23




use shogi_core::PartialPosition;

pub enum Tree<T: Ord + std::fmt::Display + Default> {
    Empty,
    Node {
        score: T,
        board: PartialPosition,
        sfen: String,
        children: Vec<Box<Tree<T>>>,
    },
}

impl<T: Ord + std::fmt::Display + Default + Clone + PartialEq> Tree<T> {

    pub fn new() -> Self {
        Tree::Empty
    }

    pub fn insert(&mut self, board: PartialPosition, sfen: String, score: T) {
        match self {
            Tree::Node { score: _, children, .. } => {
                for child in children {
                    child.insert(board.clone(), sfen.clone(), score.clone());
                }
            },
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

    pub fn search(&self, score: &T) -> bool {
        match self {
            Tree::Node { score: v, children, .. } => {
                if *v == score.clone() {
                    true
                } else {
                    children.iter().any(|child| child.search(score))
                }
            },
            Tree::Empty => false,
        }
    }

    pub fn print_in_order(&self) {
        match self {
            Tree::Node { score, children, .. } => {
                for child in children {
                    child.print_in_order();
                }
                println!("{}", score);
            },
            Tree::Empty => (),
        }
    }
}





