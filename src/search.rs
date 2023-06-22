
use crate::sfen;
use shogi_legality_lite::all_legal_moves_partial;


pub fn search(sfen: &str, depth: u32) -> Vec<String> {
    let positions = sfen::sfen_parse(sfen);
    let pos = sfen::generate_pos(positions);
    
    if depth == 0 {
        return Vec::new();
    }
    
    let next_moves = all_legal_moves_partial(&pos);
    let mut sfen_list = Vec::new();
    
    for move_item in next_moves {
        let mut temp_pos = pos.clone();
        temp_pos.make_move(move_item);
        let sfen = temp_pos.to_sfen_owned();
        sfen_list.push(sfen.clone());
        
        let deeper_moves = search(&sfen, depth - 1);
        sfen_list.extend(deeper_moves);
    }

    sfen_list
}





pub fn old_search(sfen: &str) -> Vec<String> {
    
    let positions = sfen::sfen_parse(sfen);
    let pos = sfen::generate_pos(positions);
    let next_moves = all_legal_moves_partial(&pos); 
    let mut sfen_list = Vec::new();
    
    for move_item in next_moves {
        let mut temp_pos = pos.clone();
        temp_pos.make_move(move_item);
        let sfen = temp_pos.to_sfen_owned();
        sfen_list.push(sfen);
    }

    sfen_list
}


pub fn search_dep_2(depth: i32, sfen: &str) -> Vec<String> {
    
    let mut parent = Vec::new();
    parent.push(sfen.to_string());
    
    let mut final_result = Vec::new();
    for dep in 1..=depth {
        let mut next_sfen_list = Vec::new();
        for sfen in &parent {
            let moves = old_search(sfen);
            next_sfen_list.extend(moves);
        }
        final_result.extend(next_sfen_list.clone());
        parent = next_sfen_list;

    }

    final_result
}





pub fn search_dep(depth: i32, parent: Vec<String>) -> Vec<String> {
    
    let mut result = parent.clone();
    let mut final_result = Vec::new();
    for dep in 1..=depth {
        
        let mut next_sfen_list = Vec::new();
        for sfen in &result {
            let moves = old_search(sfen);
            next_sfen_list.extend(moves.clone());
        }

        final_result.extend(next_sfen_list.clone());
        result = next_sfen_list;    
    }
    
    final_result
}

