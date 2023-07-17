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
                children.push(Box::new(Tree::Node {
                    score: score,
                    board: board,
                    sfen: sfen,
                    children: Vec::new(),
                }));
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


    pub fn find_sfen(&self, score: &T) -> Option<String> {
        match self {
            Tree::Node { score: v, sfen, children, .. } => {
                if v == score {
                    Some(sfen.clone())
                } else {
                    for child in children {
                        if let Some(found_sfen) = child.find_sfen(score) {
                            return Some(found_sfen);
                        }
                    }
                    None
                }
            },
            Tree::Empty => None,
        }
    }


    pub fn find_position(&self, score: &T) -> Option<PartialPosition> {
        match self {
            Tree::Node { score: v, board, children, .. } => {
                if v == score {
                    Some(board.clone())
                } else {
                    for child in children {
                        if let Some(found_pos) = child.find_position(score) {
                            return Some(found_pos);
                        }
                    }
                    None
                }
            },
            Tree::Empty => None,
        }
    }
}





