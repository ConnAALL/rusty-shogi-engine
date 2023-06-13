// Russell Kosovsky

mod view;
use shogi_core::PartialPosition;
use shogi_legality_lite::all_legal_moves_partial;


fn main() {
    let startpos = PartialPosition::startpos();

    let stri = startpos.to_sfen_owned();
    println!("STARTING BOARD");
    println!("{}", stri);
    view::display_sfen(&stri);
    

    let moves = all_legal_moves_partial(&startpos);
    println!("All Possible Moves:");
    for move_item in moves {
        println!("{:?}", move_item);
    }
    
}






