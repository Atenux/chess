use core::fmt;


pub enum Player{
  Black,
  White,
}

pub enum Piece{
  K(Player),
  Q(Player),
  R(Player),
  B(Player),
  N(Player),
  P(Player),
  Empty,
}

struct Tile{
  x: i8,
  y: i8,
  piece: Piece,
}

pub struct Board{
  tiles: [[Piece; 8]; 8]
}

impl fmt::Display for Board {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut s: String;
    for row in self.tiles{
      for tile in row{
        s.push_str("")
      }
    }
    write!(f, )
  }
}
impl Board{
  pub fn new() -> Board {
    use Piece::*;
    use Player::*;
    let tiles: [[Piece; 8]; 8] = 
      [
        [R(Black), N(Black), B(Black), Q(Black), K(Black), B(Black), N(Black), R(Black)],
        [P(Black),P(Black),P(Black),P(Black),P(Black),P(Black),P(Black),P(Black)],
        [Empty,Empty,Empty,Empty,Empty,Empty,Empty,Empty,],
        [Empty,Empty,Empty,Empty,Empty,Empty,Empty,Empty,],
        [Empty,Empty,Empty,Empty,Empty,Empty,Empty,Empty,],
        [Empty,Empty,Empty,Empty,Empty,Empty,Empty,Empty,],
        [P(White),P(White),P(White),P(White),P(White),P(White),P(White),P(White),],
        [R(White), N(White), B(White), Q(White),K(White),B(White), N(White), R(White),]
      ];
    Board { tiles }
  }
}