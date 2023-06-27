// Russell Kosovsky

mod search;
mod view;
mod sfen;
mod eval;


fn search_test() {
    
    let start = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";
    let depth = 3;
    let current_depth = 0;

    let nodes = search::search(&start, depth, current_depth);
    
    //for node in &nodes {
    //    println!("{:?}", node);
    //    println!("{:?}", view::display_sfen(node));
    //}
    
    if search::has_duplicates(&nodes) {
        println!("Duplicates found in the vector");
    } else {
        println!("No duplicates found in the vector");
    }

    println!("Number of moves: {:?}", nodes.len());

    println!("PERFT: ");
    let max_depth = 3;
    
    for dep in 1..=max_depth {
        let node_count = search::perft(&start, dep);
        println!("Depth: {:<2} Nodes: {}", dep, node_count);
    }
}

fn test_rook_mobility() {
    let sfen = "lnsgkgsnl/1r5b1/p1ppppppp/p8/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL w - 1";
    let coord = "C1";
    
    println!("SFEN: {:?}", sfen);
    view::display_sfen(sfen);
    println!("------------------------------------------------------------------------------------");
    println!("AT: {:?}", coord);
    println!("------------------------------------------------------------------------------------");

    let (num_moves, captured_pieces) = eval::mobility(sfen, coord);

    println!("Number of possible moves: {}", num_moves);
    println!("Captured pieces: {:?}", captured_pieces);
}


fn main() {
    
    let sfen = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5RL/LNSGKGSN1 w - 1";
    let prom_sfen = "lnsgkgs+nl/1+r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R+L/L+N+SGKGSN1 w - 1";
    
    //search_test();
    
    //----------------------PIECE_SQR_TBL_TEST---------------------------
    //println!("SFEN: {:?}", sfen);
    //view::display_sfen(sfen);
    //let eval_pst = eval::evaluate_piece_table(sfen, "black");
    //println!("{:?}", eval_pst);

    //-----------------------PROM_PIECES_TEST-----------------------------
    //let (black_pieces, white_pieces) = eval::promoted_pieces(prom_sfen);
    //println!("Number blacks promoted pieces: {:?}", black_pieces);
    //println!("Number whites promoted pieces: {:?}", white_pieces);

    //-----------------------ROOK_MOBIL_TEST-----------------------------
    test_rook_mobility();
}
