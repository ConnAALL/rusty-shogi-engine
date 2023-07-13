// Russell Kosovsky

mod search;
mod view;
mod sfen;
mod eval;
use shogi_legality_lite::{normal_from_candidates, is_legal_partial_lite, all_legal_moves_partial};
use shogi_core::{PartialPosition, Square, Piece, Color, Move, PieceKind};


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

        
    //let (white_hand, black_hand) = eval::hand_pieces(&sfen);
    let (white_hand, black_hand) = eval::eval_hand(&sfen);
    
    println!("white hand: {:?}", white_hand);
    println!("black hand: {:?}", black_hand);

}


fn eval_test() {
    
    let sfen = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";
    view::display_sfen(&sfen);
    let fitness = eval::evaluate(&sfen);
    let (white_fitness, black_fitness) = eval::evaluate(&sfen);
    println!("white fitness: {:?}", white_fitness);
    println!("black fitness: {:?}", black_fitness);

}


fn main() {
    
    let sfen = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";
    let prom_sfen = "lnsgkgs+nl/1+r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R+L/L+N+SGKGSN1 w - 1";
    
    // testing sfen side setter
    println!("sfen: {:?}", sfen);
    let new_sfen = sfen::set_sfen_turn(&sfen, "b");
    println!("changing side to black {:?}", new_sfen);


    //search_test();
    
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
    eval_test();

    //coord_test();
    //mobility_tests();
    //hand_test();

}



