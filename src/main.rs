// Russell Kosovsky

mod search;
mod view;
mod sfen;
use shogi_legality_lite::all_legal_moves_partial;
//use shogi_core::{PartialPosition, Square, Piece};

const SQUARES: [&str; 82] = ["init",
"SQ_1A",
"SQ_1B",
"SQ_1C",
"SQ_1D",
"SQ_1E",
"SQ_1F",
"SQ_1G",
"SQ_1H",
"SQ_1I",
"SQ_2A",
"SQ_2B",
"SQ_2C",
"SQ_2D",
"SQ_2E",
"SQ_2F",
"SQ_2G",
"SQ_2H",
"SQ_2I",
"SQ_3A",
"SQ_3B",
"SQ_3C",
"SQ_3D",
"SQ_3E",
"SQ_3F",
"SQ_3G",
"SQ_3H",
"SQ_3I",
"SQ_4A",
"SQ_4B",
"SQ_4C",
"SQ_4D",
"SQ_4E",
"SQ_4F",
"SQ_4G",
"SQ_4H",
"SQ_4I",
"SQ_5A",
"SQ_5B",
"SQ_5C",
"SQ_5D",
"SQ_5E",
"SQ_5F",
"SQ_5G",
"SQ_5H",
"SQ_5I",
"SQ_6A",
"SQ_6B",
"SQ_6C",
"SQ_6D",
"SQ_6E",
"SQ_6F",
"SQ_6G",
"SQ_6H",
"SQ_6I",
"SQ_7A",
"SQ_7B",
"SQ_7C",
"SQ_7D",
"SQ_7E",
"SQ_7F",
"SQ_7G",
"SQ_7H",
"SQ_7I",
"SQ_8A",
"SQ_8B",
"SQ_8C",
"SQ_8D",
"SQ_8E",
"SQ_8F",
"SQ_8G",
"SQ_8H",
"SQ_8I",
"SQ_9A",
"SQ_9B",
"SQ_9C",
"SQ_9D",
"SQ_9E",
"SQ_9F",
"SQ_9G",
"SQ_9H",
"SQ_9I"];

fn search_test(sfen: &str) -> Vec<String> {

    let positions = sfen::sfen_parse(sfen);
    println!("{:?}", positions);
    let pos = sfen::generate_pos(positions);
    let next_moves = all_legal_moves_partial(&pos);
    println!("All Possible Moves:");
    let mut sfen_list = Vec::new();
    for move_item in next_moves {
            
        // display move object
        println!("{:?}", move_item);
            
        // display from sqr
        let fromindex = move_item.from().unwrap().index();
        let fromsqr = SQUARES[fromindex as usize];
        println!("FROM: {:?}", fromsqr);
            
        // display to sqr
        let toindex = move_item.to().index();
        let tosqr = SQUARES[toindex as usize];
        println!("TO: {:?}", tosqr);
            
        // clone position and "make" the move so we can obtain the sfen
        let mut temp_pos = pos.clone();
        temp_pos.make_move(move_item);
        let sfen = temp_pos.to_sfen_owned();
        sfen_list.push(sfen.clone());
        println!("{}", sfen);
        view::display_sfen(&sfen); 
        println!("\n");
    }

    sfen_list
}

fn main() {

    let start = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";
    let sfens = search_test(&start);
    for sfen in sfens {
        println!("{:?}", sfen);
    }
    println!("---------------------------------------------");

    let test = search::search(&start);
    for item in test {
        println!("{:?}", item);
    }

}



