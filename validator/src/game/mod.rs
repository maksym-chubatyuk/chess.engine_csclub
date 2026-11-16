mod checks;
mod helpers;
// pub for debugging from main.rs,
// this should probably be private later
pub mod pieces;

use checks::is_king_in_check;
pub use helpers::*;
use std::fmt;

#[derive(Clone, Copy)]
pub struct GameState {
    pieces: [[u64; 6]; 2],
    player_turn: Color,
    castling_rights: CastlingRights,

    //NOTE: The 50 "move" rule counts a move as *both* sides moving
    moves_without_capture: u8,
    en_passant_square: Option<u8>,
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
            en_passant_square: None,
        }
    }

    pub fn get_piece(&self, color: Color, piece: Piece) -> u64 {
        self.pieces[color as usize][piece as usize]
    }

    pub fn get_piece_mut(&mut self, color: Color, piece: Piece) -> &mut u64 {
        &mut self.pieces[color as usize][piece as usize]
    }

    pub fn get_piece_on(&self, square: u64) -> Option<(Color, Piece)> {
        for color in [Color::White, Color::Black] {
            for piece in Piece::list() {
                if self.get_piece(color, piece) & square != 0 {
                    return Some((color, piece));
                }
            }
        }
        None
    }

    pub fn is_valid(&self) -> bool {
        // TODO : more checks necessary, probably?
        !is_king_in_check(self, Color::White).unwrap()
            && !is_king_in_check(self, Color::Black).unwrap()
    }

    pub fn get_all_moves(&self) -> Vec<String> {
        let mut result = Vec::new();

        for square in 0..64 {
            let from_mask = 1u64 << square;

            let Some((color, piece)) = self.get_piece_on(from_mask) else {
                continue;
            };

            if std::mem::discriminant(&color) != std::mem::discriminant(&self.player_turn) {
                continue;
            }

            let (team_pieces, enemy_pieces) = match color {
                Color::White => (
                    self.get_color_pieces(Color::White),
                    self.get_color_pieces(Color::Black),
                ),
                Color::Black => (
                    self.get_color_pieces(Color::Black),
                    self.get_color_pieces(Color::White),
                ),
            };

            let moves =
                pieces::get_piece_moves_from(from_mask, piece, color, team_pieces, enemy_pieces);
            let threats =
                pieces::get_piece_threats_from(from_mask, piece, color, team_pieces, enemy_pieces);

            let (moves, threats) = match (moves, threats) {
                (Ok(moves), Ok(threats)) => (moves, threats),
                _ => break,
            };

            let possible_plays = moves | (threats & enemy_pieces);

            for target_mask in set_bits(possible_plays) {
                //NOTE: This is "copy-make" and not "make-unmake", maybe consider hybrid approach??
                let mut next_state = self.clone();

                *next_state.get_piece_mut(color, piece) &= !from_mask;

                // Remove any captured piece
                if let Some((captured_color, captured_piece)) = next_state.get_piece_on(target_mask)
                {
                    assert!(
                        std::mem::discriminant(&color) != std::mem::discriminant(&captured_color)
                    );
                    *next_state.get_piece_mut(captured_color, captured_piece) &= !target_mask;
                }

                *next_state.get_piece_mut(color, piece) |= target_mask;

                if !next_state.is_valid() {
                    continue;
                }

                let mut move_text = mask_to_square(from_mask).unwrap_or_default();
                move_text.push_str(&mask_to_square(target_mask).unwrap_or_default());
                result.push(move_text);
            }
        }

        result
    }

    pub fn get_color_pieces(&self, color: Color) -> u64 {
        self.pieces[color as usize]
            .iter()
            .fold(0u64, |acc, piece| acc | piece)
    }

    pub fn get_occupied_squares(&self) -> u64 {
        self.get_color_pieces(Color::White) | self.get_color_pieces(Color::Black)
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
