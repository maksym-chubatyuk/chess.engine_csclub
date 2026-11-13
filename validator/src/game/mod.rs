mod helpers;
// pub for debugging from main.rs,
// this should probably be private later
pub mod pieces;

pub use helpers::*;
use std::fmt;

pub struct GameState {
    pub pieces: [[u64; 6]; 2],
    pub player_turn: Color,
    pub castling_rights: CastlingRights,

    //NOTE: The 50 "move" rule counts a move as *both* sides moving
    pub moves_without_capture: usize,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            pieces: [
                // White: Pawn, Knight, Bishop, Rook, Queen, King
                [
                    0x000000000000FF00, // Rank 2
                    0x0000000000000042, // b1, g1
                    0x0000000000000024, // c1, f1
                    0x0000000000000081, // a1, h1
                    0x0000000000000008, // d1
                    0x0000000000000010, // e1
                ],
                // Black: Pawn, Knight, Bishop, Rook, Queen, King
                [
                    0x00FF000000000000, // Rank 7
                    0x4200000000000000, // b8, g8
                    0x2400000000000000, // c8, f8
                    0x8100000000000000, // a8, h8
                    0x0800000000000000, // d8
                    0x1000000000000000, // e8
                ],
            ],
            player_turn: Color::White,
            castling_rights: CastlingRights::ANY,
            moves_without_capture: 0,
        }
    }

    pub fn get_piece(&self, color: Color, piece: Piece) -> u64 {
        self.pieces[color as usize][piece as usize]
    }

    pub fn get_piece_mut(&mut self, color: Color, piece: Piece) -> &mut u64 {
        &mut self.pieces[color as usize][piece as usize]
    }

    pub fn get_pieces(&self, color: Color) -> u64 {
        self.pieces[color as usize]
            .iter()
            .fold(0u64, |acc, piece| acc | piece)
    }

    pub fn get_occupied_squares(&self) -> u64 {
        self.get_pieces(Color::White) | self.get_pieces(Color::Black)
    }

    pub fn move_piece(&mut self, piece: Piece, color: Color, from: u64, to: u64) {
        unimplemented!("Only move creation for now");
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const PIECES: [Piece; 6] = [
            Piece::Pawn,
            Piece::Knight,
            Piece::Bishop,
            Piece::Rook,
            Piece::Queen,
            Piece::King,
        ];

        writeln!(f, "  +-----------------+")?;

        for rank in (0..8).rev() {
            write!(f, "{} | ", rank + 1)?;

            for file in 0..8 {
                let mask = 1u64 << (rank * 8 + file);
                let mut symbol = '•';

                'find_piece: for color in [Color::White, Color::Black] {
                    for piece in PIECES {
                        if self.pieces[color as usize][piece as usize] & mask != 0 {
                            symbol = match piece {
                                Piece::Pawn => 'P',
                                Piece::Knight => 'N',
                                Piece::Bishop => 'B',
                                Piece::Rook => 'R',
                                Piece::Queen => 'Q',
                                Piece::King => 'K',
                            };

                            if matches!(color, Color::Black) {
                                symbol = symbol.to_ascii_lowercase();
                            }

                            break 'find_piece;
                        }
                    }
                }

                write!(f, "{} ", symbol)?;
            }

            writeln!(f, "|")?;
        }

        writeln!(f, "  +-----------------+")?;
        writeln!(f, "    a b c d e f g h")?;

        let turn = match self.player_turn {
            Color::White => "White",
            Color::Black => "Black",
        };
        write!(f, "Turn: {}", turn)
    }
}
