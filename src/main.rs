// Russell Kosovsky
//
mod search;
mod view;
mod sfen;


fn manual_test() {
    
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

    let sfen = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL w - 1";
    let side = sfen::get_color(sfen);    
    println!("{:?}", side);

    let positions = sfen::sfen_parse(sfen);
    let mut pos = sfen::generate_pos(positions);
    pos.side_to_move_set(side);

    let to_sfen = pos.to_sfen_owned();
    println!("{:?}", to_sfen);

}


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

fn main() {
    search_test();
    //manual_test();
    //partial_pos_test();
}
