use core::fmt;
use std::ops::Add;


pub enum Player{
  black,
  white,
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

impl fmt::Display for Piece {
  
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    use Piece::*;
    use Player::*;
    match self {
      K(player) => match player {
          white => write!(f, "♔"),
          black => write!(f, "♚"),
      },
      Q(player) =>  match player {
        white => write!(f, "♕"),
        black => write!(f, "♛"),
      },
      
      R(player) => match player {
        white => write!(f, "♖"),
        black => write!(f, "♜"),
      },
      B(player) => match player {
        white => write!(f, "♗"),
        black => write!(f, "♝"),
      },
      N(player) => match player {
        white => write!(f, "♘"),
        black => write!(f, "♞"),
      },
      P(player) => match player {
        white => write!(f, "♙"),
        black => write!(f, "♟︎"),
      },
      Empty => write!(f," "),
    }
  }
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
    todo!();
    // let mut s: String = "".to_string();
    // for row in self.tiles{
    //   for tile in row{
    //     s.add(tile.to_string().as_str());
    //   }
    //   s.add("\n");
    // }
    // return Result::Ok(());
  }
}
impl Board{
  pub fn new() -> Board {
    use Piece::*;
    use Player::*;
    let tiles: [[Piece; 8]; 8] = 
      [
        [R(black), N(black), B(black), Q(black), K(black), B(black), N(black), R(black)],
        [P(black),P(black),P(black),P(black),P(black),P(black),P(black),P(black)],
        [Empty,Empty,Empty,Empty,Empty,Empty,Empty,Empty,],
        [Empty,Empty,Empty,Empty,Empty,Empty,Empty,Empty,],
        [Empty,Empty,Empty,Empty,Empty,Empty,Empty,Empty,],
        [Empty,Empty,Empty,Empty,Empty,Empty,Empty,Empty,],
        [P(white),P(white),P(white),P(white),P(white),P(white),P(white),P(white),],
        [R(white), N(white), B(white), Q(white),K(white),B(white), N(white), R(white),]
      ];
    Board { tiles }
  }
}