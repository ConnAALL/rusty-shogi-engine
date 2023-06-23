// Russell Kosovsky

mod search;
mod view;
mod sfen;

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

fn main() {

    let start = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";
    let depth = 2;

    let nodes = search::search(&start, depth);
    
    for node in &nodes {
        println!("{:?}", node);
    }
    if search::has_duplicates(&nodes) {
        println!("Duplicates found in the vector");
    } else {
        println!("No duplicates found in the vector");
    }

    println!("Number of nodes: {:?}", nodes.len());
}


