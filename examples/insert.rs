//! ```cargo
//! [dependencies]
//! piece_table = { path = "../" }
//! ```

pub fn main() {
    let sample = include_str!("../samples/daffodils.txt");

    let mut table = piece_table::PieceTable::new();

    for ch in sample.chars() {
        table.insert_at_end(ch);
    }

    // table.show();
    let stringy = table.to_string();
    println!("len: {}, \n{}", stringy.len(), stringy);
    // println!("Real Length: {},\nsample: {}", sample.len(), sample);
}
