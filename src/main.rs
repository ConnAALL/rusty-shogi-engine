// Russell Kosovsky

// TODO: - put tests where they belong and clean up main.rs
//       - oh also we need to make everything account for drops (search.rs and play.rs)
//       - 

mod view;
mod sfen;
mod eval;
mod search;
mod tree;
mod play;
mod book;

use shogi_legality_lite::{normal_from_candidates, is_legal_partial_lite, all_legal_moves_partial};
use shogi_core::{PartialPosition, Square, Piece, Color, Move, PieceKind};


fn search_test() {
    
    let start = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";
    let depth = 2;
    let current_depth = 0;

    let moves = search::search(&start, depth, current_depth);
    
    
    for outcome in &moves {
        println!("{:?}", outcome);
        println!("{:?}", view::display_sfen(outcome));
    }
    
    if search::has_duplicates(&moves) {
        println!("Duplicates found in the vector");
    } else {
        println!("No duplicates found in the vector");
    }

    println!("Number of moves: {:?}", moves.len());

    println!("PERFT: ");
    let max_depth = 3;
    
    for dep in 1..=max_depth {
        let node_count = search::perft(&start, dep);
        println!("Depth: {:<2} Nodes: {}", dep, node_count);
    }

}


fn king_vuln_test() {

    let sfen = "8l/1l+R2P3/p2pBG1pp/kps1p4/Nn1P2G2/P1P1P2PP/1pS6/1KSG3+r1/LN2+p3L w Sbgn3p 124";
    let coord = "H8";
    
    //let sfen = "lnsgkgsnl/4r2b1/pppp1pppp/9/9/9/PPPP1PPPP/1B5R1/LNSGKGSNL w - 1";
    //let coord = "E9";
    
    println!("SFEN: {:?}", sfen);
    view::display_sfen(sfen);

    let king_vuln = eval::enemy_king_vuln(sfen, Color::White);
    println!("KING VULN: {:?}", king_vuln);

}


fn king_attackers_test() {

    let sfen = "lnsgkgsnl/4r2b1/pppp1pppp/9/9/9/PPPP1PPPP/1B1P1l1R1/LNSGKGSNL w - 1";
    
    println!("SFEN: {:?}", sfen);
    view::display_sfen(sfen);

    let color = sfen::get_color(sfen);
    let enemy_color = sfen::get_enemy_color(sfen);
    
    // Parse the SFEN string into a position
    let positions = sfen::sfen_parse(sfen);// creates list of board squares and the pieces on them (if there are any)
    let mut pos = sfen::generate_pos(positions); // creates a "partial position" out of it
    pos.side_to_move_set(color); // finalize the partial position
    
    let enemy_king_sqr = pos.king_position(enemy_color);

    println!("Side To Move: {:?}", color);
    println!("enemy king square: {:?}", enemy_king_sqr);

    // change king to a pawn
    println!("changing king to pawn:");
    if enemy_color == Color::White {
        pos.piece_set(enemy_king_sqr.unwrap(), Some(Piece::W_P));
    } else {
        pos.piece_set(enemy_king_sqr.unwrap(), Some(Piece::B_P));
    }

    let res = pos.to_sfen_owned();
    view::display_sfen(&res);
   
    let next_moves = all_legal_moves_partial(&pos); 

    let mut attacks = Vec::new();

    for move_item in next_moves {
        
        if move_item.to() == enemy_king_sqr.unwrap() && move_item.is_promoting() == false {
            attacks.push(move_item);
        }
    }

    // change back to king
    println!("changing back to king:");
    if enemy_color == Color::White {
        pos.piece_set(enemy_king_sqr.unwrap(), Some(Piece::W_K));
    } else {
        pos.piece_set(enemy_king_sqr.unwrap(), Some(Piece::B_K));
    }

    let fin = pos.to_sfen_owned();
    view::display_sfen(&fin);

    println!("{:?}", attacks);
} 


fn coord_test() {
    
    let sfen = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";
    view::display_sfen(&sfen);

    let positions = sfen::sfen_parse(sfen);// creates list of board squares and the pieces on them (if there are any)
    let mut pos = sfen::generate_pos(positions.clone()); // creates a "partial position" out of it
    pos.side_to_move_set(sfen::get_color(sfen));
    
    println!("Positions: {:?}", positions);

    let mut white_rook_mobil = 0;
    let mut black_rook_mobil = 0;
    
    for sqr in &positions {
       
        if sqr.1 == "W_R" {
            pos.side_to_move_set(Color::White);
            let coord = &sqr.0;
            println!("COORD: {:?}", coord);
            let sfen = pos.to_sfen_owned();
            let mobil = eval::mobility(&sfen, coord.to_string());
            white_rook_mobil += mobil;
            println!("MOBILITY: {:?}", mobil);

        } else if sqr.1 == "B_R" {
            pos.side_to_move_set(Color::Black);
            let coord = &sqr.0;
            println!("COORD: {:?}", coord);
            let sfen = pos.to_sfen_owned();
            let mobil = eval::mobility(&sfen, coord.to_string());
            black_rook_mobil += mobil;
            println!("MOBILITY: {:?}", mobil);
        }
    }

    println!("Final White Mobility: {:?}", white_rook_mobil);
    println!("Final Black Mobility: {:?}", black_rook_mobil);

}


fn mobility_tests() {

    let sfen = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PP1PPPPPP/1B5R1/LNSGKGSNL b - 1";
    view::display_sfen(sfen);

    let (wr_mob, br_mob) = eval::rook_mobility(&sfen);
    println!("white rook mobil: {:?}", wr_mob);
    println!("black rook mobil: {:?}", br_mob);
    
    let (wl_mob, bl_mob) = eval::lance_mobility(&sfen);
    println!("white lance mobil: {:?}", wl_mob);
    println!("black lance mobil: {:?}", bl_mob);

    let (wb_mob, bb_mob) = eval::bishop_mobility(&sfen);
    println!("white bishop mobil: {:?}", wb_mob);
    println!("black bishop mobil: {:?}", bb_mob);

}


fn hand_test() {

    //let sfen = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";
    let sfen = "8l/1l+R2P3/p2pBG1pp/kps1p4/Nn1P2G2/P1P1P2PP/1pS6/1KSG3+r1/LN2+p3L w Sbgn3p 124";
    view::display_sfen(&sfen);
    
    let positions = sfen::sfen_parse(sfen);// creates list of board squares and the pieces on them (if there are any)
    let mut pos = sfen::generate_pos(positions.clone()); // creates a "partial position" out of it

    println!("SFEN: {:?}", sfen);

        
    let (white_hand, black_hand) = eval::eval_hand(&sfen);
    
    println!("white hand: {:?}", white_hand);
    println!("black hand: {:?}", black_hand);

}


fn eval_test() {
    
    let sfen = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";
    view::display_sfen(&sfen);
    let (white_fitness, black_fitness) = eval::evaluate(&sfen);
    println!("white fitness: {:?}", white_fitness);
    println!("black fitness: {:?}", black_fitness);

}


fn eval2_test() {

    let sfen = "lnsgkgsnl/1r5b1/pppppp1pp/6p2/9/2P6/PP1PPPPPP/1B5R1/LNSGKGSNL b - 3";
    //let sfen = "lnsgkgsnl/1r5b1/pppppp1pp/6p2/9/6P2/PPPPPP1PP/1B5R1/LNSGKGSNL b - 3";
    println!("sfen: {:?}", sfen);
    view::display_sfen(&sfen);
    
    let ((white_fitness, black_fitness), best_features) = eval::evaluate2(&sfen);
    
    println!("white fitness: {:?}", white_fitness);
    println!("black fitness: {:?}", black_fitness);
    
    let (white_promoted_pieces, black_promoted_pieces) = best_features[0];
    let (white_pst, black_pst) = best_features[1];
    let (white_king_vln, black_king_vln) = best_features[2];
    let (white_rook_mobil, black_rook_mobil) = best_features[3];
    let (white_lance_mobil, black_lance_mobil) = best_features[4];
    let (white_bish_mobil, black_bish_mobil) = best_features[5];
    let (white_hand, black_hand) = best_features[6];

    println!(" | feature variate values: ");
    println!(" |    |WHITE|");
    println!(" | white_promoted_pieces: {:?}", white_promoted_pieces);
    println!(" | white_pst: {:?}", white_pst);
    println!(" | white_king_vln: {:?}", white_king_vln);
    println!(" | white_rook_mobil: {:?}", white_rook_mobil);
    println!(" | white_lance_mobil: {:?}", white_lance_mobil);
    println!(" | white_bish_mobil: {:?}", white_bish_mobil);
    println!(" | white_hand: {:?}", white_hand);
    println!(" | ");
    println!(" |    |BLACK|");
    println!(" | black_promoted_pieces: {:?}", black_promoted_pieces);
    println!(" | black_pst: {:?}", black_pst);
    println!(" | black_king_vln: {:?}", black_king_vln);
    println!(" | black_rook_mobil: {:?}", black_rook_mobil);
    println!(" | black_lance_mobil: {:?}", black_lance_mobil);
    println!(" | black_bish_mobil: {:?}", black_bish_mobil);
    println!(" | black_hand: {:?}", black_hand);
    println!(" | ");

}


fn tree_test() {
    
    // Set up the initial game state
    let sfen = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";
    let positions = sfen::sfen_parse(sfen);// creates list of board squares and the pieces on them (if there are any)
    let mut pos = sfen::generate_pos(positions.clone()); // creates a "partial position" out of it
    pos.side_to_move_set(sfen::get_color(sfen));

    let sfen2 = "lnsgkgsnl/1r5b1/pppppppp1/8p/9/8P/1PPPPPPPP/1B5R1/LNSGKGSNL w - 1";
    let positions2 = sfen::sfen_parse(sfen2);// creates list of board squares and the pieces on them (if there are any)
    let mut pos2 = sfen::generate_pos(positions2.clone()); // creates a "partial position" out of it
    pos2.side_to_move_set(sfen::get_color(sfen2));
    
    let sfen3 = "lnsgkgsnl/1r5b1/pppppppp1/8p/9/7PP/2PPPPPPP/1B5R1/LNSGKGSNL b - 1";
    let positions3 = sfen::sfen_parse(sfen3);// creates list of board squares and the pieces on them (if there are any)
    let mut pos3 = sfen::generate_pos(positions3.clone()); // creates a "partial position" out of it
    pos3.side_to_move_set(sfen::get_color(sfen3));
    
    let sfen4 = "lnsgkgsnl/1r5b1/ppppppp2/7pp/9/7PP/2PPPPPPP/1B5R1/LNSGKGSNL w - 1";
    let positions4 = sfen::sfen_parse(sfen4);// creates list of board squares and the pieces on them (if there are any)
    let mut pos4 = sfen::generate_pos(positions4.clone()); // creates a "partial position" out of it
    pos4.side_to_move_set(sfen::get_color(sfen4));

    // Create a new BinaryTree
    let mut tree = tree::Tree::new();
   
    // Insert some elements
    println!("Inserting nodes into the tree...");
    tree.insert(pos.clone(), sfen.to_string(), 8);
    tree.insert(pos2.clone(), sfen2.to_string(), 10);
    tree.insert(pos3.clone(), sfen3.to_string(), 11);
    tree.insert(pos4.clone(), sfen4.to_string(), 2);
    println!("Nodes inserted.\n");

    // Search for some elements
    println!("Searching for values in the tree...");
    println!("Searching for 2: {}", if tree.search(&2) { "Found" } else { "Not Found" });  
    println!("Searching for 5: {}", if tree.search(&5) { "Found" } else { "Not Found" });  
    println!("Searching for 7: {}", if tree.search(&7) { "Found" } else { "Not Found" });  
    println!("Searching for 8: {}", if tree.search(&8) { "Found" } else { "Not Found" });  
    println!("Searching for 10: {}", if tree.search(&10) { "Found" } else { "Not Found" }); 
    println!("Searching for 11: {}", if tree.search(&11) { "Found" } else { "Not Found" }); 
    println!();

    // Print the tree in order
    println!("Printing the nodes in the tree:");
    tree.print_tree();

    println!();
    
    // Print the sfen of nodes with specific scores
    println!("Printing sfen of nodes with specific scores:");
    for score in &[2, 5, 7, 8, 10, 11] {
        match tree.find_sfen(score) {
            Some(sfen) => println!("Score: {}, SFEN: {}", score, sfen),
            None => println!("Score: {} not found", score),
        }
    }
    
    println!();
    
    // Print the position of nodes with specific scores
    println!("Printing position of nodes with specific scores:");
    for score in &[2, 5, 7, 8, 10, 11] {
        match tree.find_position(score) {
            Some(position) => println!("Score: {}, Position (as sfen): {:?}", score, position.to_sfen_owned()),
            None => println!("Score: {} not found", score),
        }
    }
    
    println!();

}


pub fn test_tree_search() {
    
    let start = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";
    let depth = 3;
    let current_depth = 0;

    let moves = search::treesearch(&start, depth, current_depth, None);
   
    println!("{:?}", moves);
    
    for node in &moves {
        println!("{:?}", node);
        println!("sfen: {:?}", node.sfen);
        println!("---------------------------------------------------------------------------------");
    }

}


//pub fn minimax_playground() {

    //let root_sfen = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";
    //let root_sfen = "lnsgkgsnl/7b1/ppppppppp/4r4/9/9/PPPP1PPPP/1B5R1/LNSGKGSNL b - 1";
    //let root_sfen = "lnsgkgsnl/4r2b1/ppppppppp/9/9/9/PPPP1PPPP/3B3R1/LNSGKGSNL b - 1";
    
    //println!();
    //view::display_sfen(&root_sfen);
    //println!();
    //let dep = 2;
    //let color = sfen::get_color(&root_sfen);
    //println!("Root Sfen: {:?}", root_sfen);
    //println!("color: {:?}", color);
    //println!("search depth: {:?}", dep);
    
    //let root = search::treesearch(&root_sfen, dep, 0, None); // Create the root GameTree node
    //println!("Resulting GameTree: {:?}", root);

    //let ((white_score, black_score), best_move) = search::minimax(&root, dep, true); 
    //let ((white_score, black_score), best_move, feature_vec) = search::get_best_move(&root, dep, color); 

    //println!();
    //println!("#####################################################################################");
    //println!();

    //println!("Best move: {:?}", best_move);
    //println!("Best score for white: {}", white_score);
    //println!("Best score for black: {}", black_score);

//}


pub fn debug(sfen: &str) {
    
    println!("SFEN: {:?}", sfen);
    view::display_sfen(sfen);

    // Parse the SFEN string and generate a PartialPosition from it
    let positions = sfen::sfen_parse(sfen);
    let mut pos = sfen::generate_pos(positions); 
    pos.side_to_move_set(sfen::get_color(sfen));


    let next_moves = all_legal_moves_partial(&pos);

    // For each legal move, create a new game state and add it as a child to the current node
    for move_item in next_moves {
        let mut temp_pos = pos.clone();
        temp_pos.make_move(move_item.clone());
        let sfen = temp_pos.to_sfen_owned();

        view::display_sfen(&sfen);
        println!("{:?}" , &move_item);
        println!("-----------------");
    }

}

fn main() {
    
    let sfen = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";
    let prom_sfen = "lnsgkgs+nl/1+r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R+L/L+N+SGKGSN1 w - 1";
    let sfen2 = "9/9/ppppppppp/9/9/9/9/7R1/9 b - 1";
    
    //debug(&sfen2);
    
    //--------------------------PIECE_SQR_TBL_TEST--------------------------
    //println!("SFEN: {:?}", sfen);
    //view::display_sfen(sfen);
    //let eval_pst = eval::evaluate_piece_table(sfen, "black");
    //println!("{:?}", eval_pst);

    //--------------------------PROM_PIECES_TEST--------------------------
    //let (black_pieces, white_pieces) = eval::promoted_pieces(prom_sfen);
    //println!("Number blacks promoted pieces: {:?}", black_pieces);
    //println!("Number whites promoted pieces: {:?}", white_pieces);

    //---------------------------MOBILITY_TEST---------------------------
    //test_rook_mobility();

    //---------------------------KING_VULN_TEST---------------------------
    //king_vuln_test();
    //king_attackers_test();

    //-----------------------------EVAL_TEST-----------------------------
    //eval_test();

    //-----------------------------EVAL_TEST-----------------------------
    //test_pvs();

    //eval2_test();
    //search_test();
    //coord_test();
    //mobility_tests();
    //hand_test();
    //test_tree_search();
    //minimax_playground();
    
    play::play();
    //play::play_one_move();
    
    //book::booktest();    

}



