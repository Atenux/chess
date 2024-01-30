use crate::grid::Board;

mod grid;

fn main() {
    println!("board!");
    let board = Board::new();
    print!("{board}");
}
