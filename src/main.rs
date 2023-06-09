
mod view;

fn main() {
    println!("Hello World!");
    
    let sfen = "lnsgkgsnl/1r5b1/ppp+pppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL w - 1";

    view::display_sfen(sfen);
}


