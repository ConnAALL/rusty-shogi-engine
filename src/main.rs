// Russell Kosovsky
mod view;
use shogi::{Move, Position};
use shogi::bitboard::Factory as BBFactory;
use shogi::square::consts::*;
    
fn main() {
    BBFactory::init();
    let mut pos = Position::new();

    // Position can be set from the SFEN formatted string.
    pos.set_sfen("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1").unwrap();
    
    println!("STARTING BOARD:");
    println!("{}", pos.to_sfen());
    let sfen = pos.to_sfen();
    view::display_sfen(&sfen);

    println!("MOVING 7G TO 7F");
    // You can programatically create a Move instance.
    let m = Move::Normal{from: SQ_7G, to: SQ_7F, promote: false};
    pos.make_move(m).unwrap();

    println!("MOVING 7C TO 7D");
    // Move can be created from the SFEN formatted string as well.
    let m = Move::from_sfen("7c7d").unwrap();
    pos.make_move(m).unwrap();

    // Position can be converted back to the SFEN formatted string.
    assert_eq!("lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1 moves 7g7f 7c7d", pos.to_sfen());
    
    println!("BOARD AFTER MOVES:");
    println!("{}", pos.to_sfen());
    let new_sfen = pos.to_sfen();
    view::display_sfen(&new_sfen);

}
