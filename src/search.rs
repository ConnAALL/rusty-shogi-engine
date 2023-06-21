
use crate::sfen;
use shogi_legality_lite::all_legal_moves_partial;


pub fn search(sfen: &str) -> Vec<String> {
    
    let positions = sfen::sfen_parse(sfen);
    let pos = sfen::generate_pos(positions);
    let color = pos.side_to_move();
    //println!("{:?}", pos);
    let next_moves = all_legal_moves_partial(&pos);
    
    let mut sfen_list = Vec::new();
    for move_item in next_moves {

        // clone position and "make" the move so we can obtain the sfen
        let mut temp_pos = pos.clone();
        temp_pos.make_move(move_item);
        temp_pos.side_to_move_set(color);
        let sfen = temp_pos.to_sfen_owned();
        sfen_list.push(sfen.clone());
    }

    sfen_list
}


pub fn search_dep(depth: i32, parent: Vec<String>) -> Vec<String> {
    
    let dep = depth -1;
    let mut f_result = Vec::new();
    let mut result = parent;

    for _ in 1..=dep {
        
        let mut next_sfen_list = Vec::new();
        for sfen in &result {
            let moves = search(sfen);
            next_sfen_list.extend(moves.clone());
        }
        
        f_result.extend(next_sfen_list.clone());
        result = next_sfen_list;    
    }
    
    f_result
}


