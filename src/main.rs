use crate::grid::Board;

mod grid;

fn main() {
    let king = grid::Piece::K(grid::Player::White);
    let queen = grid::Piece::Q(grid::Player::White);
    let bqueen = grid::Piece::Q(grid::Player::Black);
    let bking = grid::Piece::K(grid::Player::Black);

    print!("{king} ");
    print!("{queen}\n");
    print!("{bqueen} ");
    print!("{bking}\n");

    println!("board!");
    let board = Board::new();
    print!("{board}");
}
