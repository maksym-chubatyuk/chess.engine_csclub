mod helpers;
mod pieces;

pub use helpers::*;
use std::fmt;

// TODO Use enum indexed list rather than individual variables
pub struct GameState {
    pub black_rooks: u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_king: u64,
    pub black_queen: u64,
    pub black_pawns: u64,

    pub white_rooks: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_king: u64,
    pub white_queen: u64,
    pub white_pawns: u64,

    pub player_turn: Color,
    pub castling_rights: CastlingRights,

    //NOTE: The 50 "move" rule counts a move as *both* sides moving
    pub moves_without_capture: usize,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            black_rooks: 0x8100000000000000,   // a8, h8
            black_knights: 0x4200000000000000, // b8, g8
            black_bishops: 0x2400000000000000, // c8, f8
            black_queen: 0x0800000000000000,   // d8
            black_king: 0x1000000000000000,    // e8
            black_pawns: 0x00FF000000000000,   // Rank 7

            white_rooks: 0x0000000000000081,   // a1, h1
            white_knights: 0x0000000000000042, // b1, g1
            white_bishops: 0x0000000000000024, // c1, f1
            white_queen: 0x0000000000000008,   // d1
            white_king: 0x0000000000000010,    // e1
            white_pawns: 0x000000000000FF00,   // Rank 2

            // I have these here for testing move validation logic
            // with only one piece on the board
            // black_rooks: 0,
            // black_knights: 0,
            // black_bishops: 0,
            // black_queen: 0,
            // black_king: 0,
            // black_pawns: 0,

            // white_rooks: 0,
            // white_knights: 0,
            // white_bishops: 0,
            // white_queen: 0,
            // white_king: 0,
            // white_pawns: 0,
            player_turn: Color::White,
            castling_rights: CastlingRights::ANY,
            moves_without_capture: 0,
        }
    }

    pub fn get_white_pieces(&self) -> u64 {
        self.white_rooks
            | self.white_knights
            | self.white_bishops
            | self.white_king
            | self.white_queen
            | self.white_pawns
    }

    pub fn get_black_pieces(&self) -> u64 {
        self.black_rooks
            | self.black_knights
            | self.black_bishops
            | self.black_king
            | self.black_queen
            | self.black_pawns
    }

    pub fn get_occupied_squares(&self) -> u64 {
        self.get_white_pieces() | self.get_black_pieces()
    }

    pub fn move_piece(&mut self, piece: Piece, color: Color, from: u64, to: u64) {
        unimplemented!("Only move creation for now");
    }

    // TODO: See if piece was actually taken and if so, reset move counter
    fn remove_black_piece_at(&mut self, mask: u64) {
        let inverted = !mask;
        self.black_rooks &= inverted;
        self.black_knights &= inverted;
        self.black_bishops &= inverted;
        self.black_queen &= inverted;
        self.black_pawns &= inverted;
    }

    fn remove_white_piece_at(&mut self, mask: u64) {
        let inverted = !mask;
        self.white_rooks &= inverted;
        self.white_knights &= inverted;
        self.white_bishops &= inverted;
        self.white_queen &= inverted;
        self.white_pawns &= inverted;
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  +-----------------+")?;

        for rank in (0..8).rev() {
            write!(f, "{} | ", rank + 1)?;

            for file in 0..8 {
                let bit_index = rank * 8 + file;
                let mask = 1u64 << bit_index;

                let symbol = if (self.white_pawns & mask) != 0 {
                    "P"
                } else if (self.white_rooks & mask) != 0 {
                    "R"
                } else if (self.white_knights & mask) != 0 {
                    "N"
                } else if (self.white_bishops & mask) != 0 {
                    "B"
                } else if (self.white_queen & mask) != 0 {
                    "Q"
                } else if (self.white_king & mask) != 0 {
                    "K"
                } else if (self.black_pawns & mask) != 0 {
                    "p"
                } else if (self.black_rooks & mask) != 0 {
                    "r"
                } else if (self.black_knights & mask) != 0 {
                    "n"
                } else if (self.black_bishops & mask) != 0 {
                    "b"
                } else if (self.black_queen & mask) != 0 {
                    "q"
                } else if (self.black_king & mask) != 0 {
                    "k"
                } else {
                    "•"
                };
                write!(f, "{} ", symbol)?;
            }
            writeln!(f, "|")?;
        }

        writeln!(f, "  +-----------------+")?;
        writeln!(f, "    a b c d e f g h")?;

        let turn_str = match self.player_turn {
            Color::White => "White",
            Color::Black => "Black",
        };
        write!(f, "Turn: {}", turn_str)?;

        Ok(())
    }
}
