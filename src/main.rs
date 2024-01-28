mod grid;

fn main() {
    let king = grid::Piece::K(grid::Player::white);
    let queen = grid::Piece::N(grid::Player::black);
    println!("{king}");
    println!("{queen}");
}
