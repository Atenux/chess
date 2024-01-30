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

struct Move{
  start: (i8,i8),
  end: (i8,i8),
}

struct Rule{
}

impl fmt::Display for Piece {
  
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    use Piece::*;
    use Player::*;
    match self {
      K(player) => match player {
          White => write!(f, "♔"),
          Black => write!(f, "♚"),
      },
      Q(player) =>  match player {
        White => write!(f, "♕"),
        Black => write!(f, "♛"),
      },
      
      R(player) => match player {
        White => write!(f, "♖"),
        Black => write!(f, "♜"),
      },
      B(player) => match player {
        White => write!(f, "♗"),
        Black => write!(f, "♝"),
      },
      N(player) => match player {
        White => write!(f, "♘"),
        Black => write!(f, "♞"),
      },
      P(player) => match player {
        White => write!(f, "♙"),
        Black => write!(f, "♟︎"),
      },
      Empty => write!(f," "),
    }
  }
}

// struct Tile{
//   x: i8,
//   y: i8,
//   piece: Piece,
// }
pub struct Board{
  tiles: [[Piece; 8]; 8]
}

impl fmt::Display for Board {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let tiles = self.tiles.iter();
    let mut s: String = "".to_string();
    for row in tiles {
      for tile in row {
        s.push_str(&tile.to_string());
        s.push_str(" ")
      }
      s.push_str("\n");
    }
    write!(f,"{}",s)
  }
}

pub struct Coordinate{
  pub row: i8,
  pub column: i8,
}

impl Coordinate{
  pub fn to_notation(self) -> String {
    todo!()
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
        [Empty, Empty,Empty,Empty,Empty,Empty,Empty,Empty,],
        [Empty,Empty,Empty,Empty,Empty,Empty,Empty,Empty,],
        [Empty,Empty,Empty,Empty,Empty,Empty,Empty,Empty,],
        [Empty,Empty,Empty,Empty,Empty,Empty,Empty,Empty,],
        [P(White),P(White),P(White),P(White),P(White),P(White),P(White),P(White),],
        [R(White), N(White), B(White), Q(White),K(White),B(White), N(White), R(White),]
      ];
    Board { tiles }
  }
  pub fn make_move(&self, start: Coordinate, end: Coordinate){
    todo!()
  }
}