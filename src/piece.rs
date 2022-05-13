#[derive(Clone, Copy)]
pub enum Piece {
    Pn,
    Rk,
    Kt,
    Bp,
    Qn,
    Kg
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
        let piece = self.piece.to_string();

        if self.white {
            piece.to_uppercase()
        } else {
            piece.to_lowercase()
        }
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
}

impl ToString for Piece {
    fn to_string(&self) -> String {
        let s = match self {
            Piece::Pn => "p",
            Piece::Rk => "r",
            Piece::Bp => "b",
            Piece::Kt => "n",
            Piece::Qn => "q",
            Piece::Kg => "k"
        };

        String::from(s)
    }
}