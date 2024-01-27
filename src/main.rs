mod grid;

fn add2(x: i32) -> i32{
    x+2
}
fn main() {
    let a = grid::Board::new();
    println!("{a}")
}
