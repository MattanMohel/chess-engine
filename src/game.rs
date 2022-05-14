use std::{
    fmt, 
    io,
    io::Write
};
use colored::Colorize;

use crate::piece::{
    Piece,
    PieceInfo
};

pub struct Game {
    pub board: [[Option<PieceInfo>; 8]; 8],
}

impl Game {
    pub fn new(src: &String) -> Self {        
        Game { 
            board: Game::board_from_file(src)
        }
    }

    pub fn board_from_file(src: &str) -> [[Option<PieceInfo>; 8]; 8] {
        let mut board = [[None; 8]; 8];

        let mut i = 0;
        let mut j = 0;

        for c in src.chars() {
            let lower = c
                .to_lowercase()
                .to_string();

            match lower.as_str() {
                "\n" => { 
                    i += 1; 
                    j = 0;
                },

                " "  => j += 1,

                "p" => board[i][j] = Some(PieceInfo::new(Piece::Pn, c.is_uppercase())),
                "r" => board[i][j] = Some(PieceInfo::new(Piece::Rk, c.is_uppercase())),
                "n" => board[i][j] = Some(PieceInfo::new(Piece::Kt, c.is_uppercase())),
                "b" => board[i][j] = Some(PieceInfo::new(Piece::Bp, c.is_uppercase())),
                "q" => board[i][j] = Some(PieceInfo::new(Piece::Qn, c.is_uppercase())),
                "k" => board[i][j] = Some(PieceInfo::new(Piece::Kg, c.is_uppercase())),
                _ => ()
            }
        }

        board
    }

    pub fn play(&mut self) -> io::Result<()> {
        loop {
            println!("{}\n", self);

            print!("input move: ");

            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            println!()
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {    
        let mut on_white = true;
        let mut row: u8 = 8;
        let mut col: u8 = 8;

        let mut buf = String::new();
        buf.reserve(200);

        for r in self.board {
            buf += &format!("{} ", row.to_string());
            row -= 1;

            for c in r {
                let square =
                    match c {
                        Some(p) => format!(" {} ", p.to_string()),
                        None => format!("   ")
                    };

                let square = 
                    if on_white {
                        square.on_bright_black()
                    } else {
                        square.on_bright_white()
                    };

                buf += &square.to_string();

                on_white = !on_white; 
            }

            buf.push('\n');

            on_white = !on_white;
        }

        buf.push_str("   ");

        for _ in self.board {
            buf += &format!("{}  ", ('i' as u8 - col) as char);
            col -= 1;
        }

        write!(f, "{}", buf)
    }
}