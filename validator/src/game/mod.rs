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

    // TODO: Threefold repetition
    halfmove_clock: u8,
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
            halfmove_clock: 0,
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
        // We need to invert the color because after the move is made, the color is inverted automatically
        // so it would fail if the player put the other in check but be fine if they put themselves
        // in check
        !is_king_in_check(self, self.player_turn.invert()).unwrap_or(true)
    }

    fn apply_move_unchecked(&mut self, from: u64, to: u64) {
        let Some((color, piece)) = self.get_piece_on(from) else {
            return;
        };

        self.halfmove_clock += 1;

        // Handle en passant capture
        if matches!(piece, Piece::Pawn) {
            self.halfmove_clock = 0; // Also resets when a pawn is moved
            if let Some(ep_square) = self.en_passant_square {
                let ep_mask = 1u64 << ep_square;
                if to == ep_mask {
                    let captured_pawn_mask = match color {
                        Color::White => ep_mask >> 8,
                        Color::Black => ep_mask << 8,
                    };

                    *self.get_piece_mut(color.invert(), Piece::Pawn) &= !captured_pawn_mask;
                }
            }
        }

        // Handle normal capture
        if let Some((captured_color, captured_piece)) = self.get_piece_on(to) {
            self.halfmove_clock = 0;
            *self.get_piece_mut(captured_color, captured_piece) &= !to;
        }

        // Update en-passant square
        self.en_passant_square = None;
        if matches!(piece, Piece::Pawn) {
            match color {
                Color::White if to == from << 16 => {
                    self.en_passant_square = Some((from << 8).trailing_zeros() as u8);
                }
                Color::Black if to == from >> 16 => {
                    self.en_passant_square = Some((from >> 8).trailing_zeros() as u8);
                }
                _ => {}
            }
        }

        // Revoke castling rights
        match from {
            0x1 => self.castling_rights.revoke(CastlingRights::WHITE_QUEENSIDE),
            0x80 => self.castling_rights.revoke(CastlingRights::WHITE_KINGSIDE),
            0x10 => self.castling_rights.revoke(CastlingRights::ANY_WHITE),
            0x100000000000000 => self.castling_rights.revoke(CastlingRights::BLACK_QUEENSIDE),
            0x8000000000000000 => self.castling_rights.revoke(CastlingRights::BLACK_KINGSIDE),
            0x1000000000000000 => self.castling_rights.revoke(CastlingRights::ANY_BLACK),
            _ => {}
        }

        *self.get_piece_mut(color, piece) &= !from;
        *self.get_piece_mut(color, piece) |= to;
        self.player_turn = self.player_turn.invert();
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

            let (team_pieces, mut enemy_pieces) = match color {
                Color::White => (
                    self.get_color_pieces(Color::White),
                    self.get_color_pieces(Color::Black),
                ),
                Color::Black => (
                    self.get_color_pieces(Color::Black),
                    self.get_color_pieces(Color::White),
                ),
            };

            if matches!(piece, Piece::Pawn) && self.en_passant_square.is_some() {
                enemy_pieces |= 1 << self.en_passant_square.unwrap();
            }

            let moves =
                pieces::get_piece_moves_from(from_mask, piece, color, team_pieces, enemy_pieces);
            let threats =
                pieces::get_piece_threats_from(from_mask, piece, color, team_pieces, enemy_pieces);

            let (moves, threats) = match (moves, threats) {
                (Ok(moves), Ok(threats)) => (moves, threats),
                _ => break,
            };

            let en_passant_mask = match (piece, self.en_passant_square) {
                (Piece::Pawn, Some(sq)) => 1u64 << sq,
                _ => 0,
            };

            let possible_plays = moves | (threats & (enemy_pieces | en_passant_mask));

            for target_mask in set_bits(possible_plays) {
                //NOTE: This is "copy-make" and not "make-unmake", maybe consider hybrid approach??
                let mut next_state = self.clone();

                next_state.apply_move_unchecked(from_mask, target_mask);

                if !next_state.is_valid() {
                    continue;
                }

                let mut move_text = mask_to_square(from_mask).unwrap_or_default();
                move_text.push_str(&mask_to_square(target_mask).unwrap_or_default());
                result.push(move_text);
            }
            if self.halfmove_clock >= 100 {
                result.push(String::from("draw"))
            }
        }

        if result == vec![String::from("draw")] {
            // Checkmate takes precedence
            return vec![];
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

    /// Attempt to make a from a UCI formatted move
    pub fn move_piece(&mut self, movement: String) -> Result<(), ForbiddenMoveError> {
        if movement.len() != 4 {
            return Err(ForbiddenMoveError("Invalid format"));
        }

        let (from_str, to_str) = movement.split_at(2);
        let from = square_to_mask(from_str).ok_or(ForbiddenMoveError("invalid starting format"))?;
        let to = square_to_mask(to_str).ok_or(ForbiddenMoveError("invalid destination format"))?;

        let moves = self.get_all_moves();
        if !moves.contains(&format!("{from_str}{to_str}")) {
            return Err(ForbiddenMoveError("not a legal move"));
        }

        self.apply_move_unchecked(from, to);
        Ok(())
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
