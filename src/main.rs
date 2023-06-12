// Russell Kosovsky
mod view;
use shogi::Position;
use shogi::bitboard::Factory as BBFactory;
use shogi_core::PartialPosition;
use shogi_legality_lite::all_legal_moves_partial;
    
fn main() {
    BBFactory::init();
    let mut pos = Position::new();

    // Position can be set from the SFEN formatted string.
    pos.set_sfen("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1").unwrap();
    
    println!("STARTING BOARD:");
    println!("{}", pos.to_sfen());
    let sfen = pos.to_sfen();
    view::display_sfen(&sfen);

    let startpos = PartialPosition::startpos();

    let moves = all_legal_moves_partial(&startpos);

    println!("All Possible Moves:");
    for move_item in moves {
        println!("{:?}", move_item);
    }
}
