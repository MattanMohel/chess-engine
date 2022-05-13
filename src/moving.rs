use crate::piece::Piece;

pub struct Pos(usize, usize);

pub struct Move {
    pos_1: Pos,
    pos_2: Pos,
    captured: Option<Piece>,
    moved: Piece
}