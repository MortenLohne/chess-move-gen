use self::PieceType::*;
use board_game_traits::board::Color;
use board_game_traits::board::Color::*;
use pgn_traits::pgn;
use std::fmt;
use std::mem;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialOrd, Ord, PartialEq, Serialize, Deserialize)]
pub enum PieceType {
    Empty = 0,
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen = 5,
    King = 6,
}

impl PieceType {
    pub fn value(self) -> f32 {
        match self {
            Pawn => 1.0,
            Knight => 3.0,
            Bishop => 3.0,
            Rook => 5.0,
            Queen => 9.0,
            King => 100.0,
            Empty => 0.0,
        }
    }
    pub fn letter(self) -> char {
        match self {
            Empty => ' ',
            Pawn => 'P',
            Knight => 'N',
            Bishop => 'B',
            Rook => 'R',
            Queen => 'Q',
            King => 'K',
        }
    }
    pub fn from_letter(ch: char) -> Option<Self> {
        match ch {
            ' ' => Some(Empty),
            'P' => Some(Pawn),
            'N' => Some(Knight),
            'B' => Some(Bishop),
            'R' => Some(Rook),
            'Q' => Some(Queen),
            'K' => Some(King),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub fn from_disc(disc: u32) -> Option<Self> {
        if disc > 6 {
            None
        } else {
            Some(unsafe { mem::transmute(disc as u8) })
        }
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let _ = fmt.write_str(match *self {
            Pawn => "Pawn",
            Knight => "Knight",
            Bishop => "Bishop",
            Rook => "Rook",
            Queen => "Queen",
            King => "King",
            Empty => "Empty square",
        });
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum Piece {
    Empty = 0,
    WhitePawn = 2,
    BlackPawn = 3,
    WhiteKnight = 4,
    BlackKnight = 5,
    WhiteBishop = 6,
    BlackBishop = 7,
    WhiteRook = 8,
    BlackRook = 9,
    WhiteQueen = 10,
    BlackQueen = 11,
    WhiteKing = 12,
    BlackKing = 13,
}

impl Default for Piece {
    fn default() -> Self {
        Piece::Empty
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            fmt,
            "{}",
            match self.color().unwrap_or(White) {
                White => self.piece_type().letter(),
                Black => self.piece_type().letter().to_ascii_lowercase(),
            }
        )
    }
}

impl Piece {
    pub fn from_letter(ch: char) -> Option<Self> {
        match (
            PieceType::from_letter(ch.to_ascii_uppercase()),
            ch.is_lowercase(),
        ) {
            (Some(Empty), _) => Some(Piece::Empty),
            (Some(piece_type), true) => Some(Self::from_type_color(piece_type, Black)),
            (Some(piece_type), false) => Some(Self::from_type_color(piece_type, White)),
            (None, _) => None,
        }
    }

    pub fn from_type_color(piece_type: PieceType, color: Color) -> Self {
        if piece_type == Empty {
            Piece::Empty
        } else {
            unsafe {
                match color {
                    White => mem::transmute::<u8, Piece>((piece_type as u32 as u8) << 1),
                    Black => mem::transmute::<u8, Piece>(((piece_type as u32 as u8) << 1) + 1),
                }
            }
        }
    }
    pub fn value(self) -> f32 {
        if self.is_empty() {
            0.0
        } else {
            let abs_value = self.piece_type().value();
            match self.color().unwrap() {
                White => abs_value,
                Black => -1.0 * abs_value,
            }
        }
    }
    pub fn piece_type(self) -> PieceType {
        unsafe { mem::transmute::<u8, PieceType>((self as u32 >> 1) as u8) }
    }
    pub fn color(self) -> Option<Color> {
        match self {
            Piece::Empty => None,
            _ => {
                if self as u32 % 2 == 0 {
                    Some(White)
                } else {
                    Some(Black)
                }
            }
        }
    }
    pub fn is_empty(self) -> bool {
        self == Piece::Empty
    }
    pub fn empty() -> Self {
        Piece::Empty
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Square(pub u8);

impl fmt::Display for Square {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let (file, rank) = self.file_rank();
        let actual_rank = ((rank as i8 - 8).abs() as u8 + b'0') as char;

        let _ = fmt.write_str(&format!("{}{}", (file + b'a') as char, actual_rank));
        Ok(())
    }
}

impl Square {
    pub fn from_alg(alg: &str) -> Result<Self, pgn::Error> {
        if alg.len() != 2 {
            Err(pgn::Error::new(
                pgn::ErrorKind::ParseError,
                format!("Invalid square {}", alg),
            ))
        } else {
            let (file, rank) = (alg.chars().nth(0).unwrap(), alg.chars().nth(1).unwrap());
            if file < 'a' || file > 'h' || rank < '1' || rank > '8' {
                Err(pgn::Error::new(
                    pgn::ErrorKind::ParseError,
                    format!("Invalid square {}", alg),
                ))
            } else {
                let (file_u8, rank_u8) = (file as u8 - b'a', 8 - (rank as u8 - b'0'));

                let square = rank_u8 * 8 + file_u8;
                Ok(Square(square))
            }
        }
    }
    pub fn from_ints(file: u8, rank: u8) -> Self {
        debug_assert!(file < 8 && rank < 8);
        Square((rank << 3) | file)
    }
    pub fn file_rank(self) -> (u8, u8) {
        (self.file(), self.rank())
    }
    pub fn file(self) -> u8 {
        self.0 & 0b0000_0111
    }
    pub fn rank(self) -> u8 {
        self.0 >> 3
    }

    pub const H1: Square = Square(63);
    pub const G1: Square = Square(62);
    pub const E1: Square = Square(60);
    pub const C1: Square = Square(58);
    pub const A1: Square = Square(56);

    pub const H8: Square = Square(7);
    pub const G8: Square = Square(6);
    pub const E8: Square = Square(4);
    pub const C8: Square = Square(2);
    pub const A8: Square = Square(0);
}
