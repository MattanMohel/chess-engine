use colored::Colorize;


const PIECE_UNICODE: u32 = 9812;
const BLACK_OFFSET: u32  = 6;

#[derive(Clone, Copy)]
pub enum Piece {
    Kg = 0,
    Qn = 1,
    Rk = 2,
    Bp = 3,
    Kt = 4,
    Pn = 5
}

#[derive(Clone, Copy)]
pub struct PieceInfo {
    pub piece: Piece,
    pub moved: bool,
    pub white: bool
}

impl PieceInfo {
    pub fn new(piece: Piece, white: bool) -> Self {
        Self {
            piece: piece,
            moved: false,
            white: white
        }
    }
}

impl ToString for PieceInfo {
    fn to_string(&self) -> String {
        let unicode = 
            if self.white {
                char::from_u32(self.piece.unicode())
                    .unwrap()
                    .to_string()
                    .bright_cyan()
            } else {
                char::from_u32(self.piece.unicode() + BLACK_OFFSET)
                    .unwrap()
                    .to_string()
                    .black()
            };

        unicode.to_string()
    }
}

impl Piece {
    pub fn value(self) -> usize {
        match self {
            Piece::Pn => 1,
            Piece::Kt => 3,
            Piece::Bp => 3,
            Piece::Rk => 5,
            Piece::Qn => 9,
            Piece::Kg => usize::max_value()
        }
    }

    pub fn unicode(self) -> u32 {
        PIECE_UNICODE + self as u32
    }
}