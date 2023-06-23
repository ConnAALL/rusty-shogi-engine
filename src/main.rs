// Russell Kosovsky
//
mod search;
mod view;
mod sfen;


fn test() {
    
    let start = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";

    let initial_moves = search::single_search(start);
    
    for bs in &initial_moves {
        println!("\nSFEN: {:?}", bs);
        //view::display_sfen(bs);
    }
    println!("\nNumber of moves: {:?}", initial_moves.len());

    println!("\n############################################################################################\n");

    let one = &initial_moves[0];
    println!("ROOT SFEN: {:?}", one);
    view::display_sfen(one);


    println!("#############################################################");

    let one_moves = search::single_search(one);
    for bs in &one_moves {
        println!("\nSFEN: {:?}", bs);
        view::display_sfen(bs);
    }
}

fn partial_pos_test() {

}


fn search_test() {

    let start = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";
    let depth = 2;

    let nodes = search::search(&start, depth);
    
    for node in &nodes {
        println!("{:?}", node);
        println!("{:?}", view::display_sfen(node));
    }
    if search::has_duplicates(&nodes) {
        println!("Duplicates found in the vector");
    } else {
        println!("No duplicates found in the vector");
    }

    println!("Number of nodes: {:?}", nodes.len());

    let max_depth = 3;

    for dep in 1..=max_depth {
        let node_count = search::perft(&start, dep);
        println!("Depth: {:<2} Nodes: {}", dep, node_count);
    }
}

fn main() {
    //search_test();
    //test();
    partial_pos_test();
}
