

pub fn display_sfen(sfen: &str) {
    let parts: Vec<&str> = sfen.split(' ').collect();
    let board_layout = parts[0];

    //println!("+---------+---------+---------+---------+---------+---------+---------+---------+---------+");
    println!("----------------------------------------------");

    for row in board_layout.split('/') {
        let mut row_string = "|".to_string();
        let mut squares_count = 0;
        let mut chars = row.chars();
        while let Some(ch) = chars.next() {
            if ch.is_numeric() {
                let num_spaces: u32 = ch.to_digit(10).unwrap();
                squares_count += num_spaces;
                for _ in 0..num_spaces {
                    row_string.push_str("    |");
                }
            } else if ch == '+' {
                // Check if there is a piece character after '+'
                match chars.next() {
                    Some(piece) => {
                        squares_count += 1;
                        row_string.push_str(&format!(" +{} |", piece));
                    },
                    None => panic!("Invalid SFEN string: '+' not followed by piece character"),
                }
            } else {
                squares_count += 1;
                row_string.push_str(&format!("  {} |", ch));
            }
            // Check for row size
            if squares_count > 9 {
                panic!("Invalid SFEN string: too many squares in a row");
            }
        }
        println!("{}", row_string);
        println!("----------------------------------------------");
    }
}

//fn test() {
//    let sfen = "lnsgkgsnl/1r5b1/ppp+pppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL w - 1";
//    display_sfen(sfen);
//}

