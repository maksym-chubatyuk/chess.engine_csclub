use std::{fmt, slice::SliceIndex};

// Useful masks to stop pieces from wrapping around the board when doing binary magic
pub const NOT_A_FILE: u64 = 0xFEFEFEFEFEFEFEFE;
pub const NOT_H_FILE: u64 = 0x7F7F7F7F7F7F7F7F;
pub const NOT_FIRST_RANK: u64 = 0xFFFFFFFFFFFFFF00;
pub const NOT_LAST_RANK: u64 = 0x00FFFFFFFFFFFFFF;

#[derive(Clone, Copy)]
#[repr(usize)]
pub enum Color {
    White = 0,
    Black = 1,
}

#[derive(Clone, Copy)]
#[repr(usize)]
pub enum Piece {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

#[derive(Clone, Copy)]
pub struct CastlingRights(pub u8);

impl CastlingRights {
    pub const WHITE_QUEENSIDE: Self = CastlingRights(0b0001);
    pub const WHITE_KINGSIDE: Self = CastlingRights(0b0010);
    pub const BLACK_QUEENSIDE: Self = CastlingRights(0b0100);
    pub const BLACK_KINGSIDE: Self = CastlingRights(0b1000);

    pub const ANY_WHITE: Self = CastlingRights(0b0011);
    pub const ANY_BLACK: Self = CastlingRights(0b1100);
    pub const ANY: Self = CastlingRights(0b1111);

    pub fn has(&self, right: Self) -> bool {
        (self.0 & right.0) == right.0
    }

    pub fn revoke(&mut self, right: Self) {
        self.0 &= !right.0;
    }
}

#[derive(Debug, Clone)]
pub struct ForbiddenMoveError(pub &'static str);

impl fmt::Display for ForbiddenMoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let piece = &self.0;
        write!(f, "Forbidden {piece} move")?;
        Ok(())
    }
}

impl std::error::Error for ForbiddenMoveError {}

pub fn mask_to_square(mask: u64) -> Option<String> {
    if mask.count_ones() != 1 {
        return None;
    }
    let index = mask.trailing_zeros();
    let file_idx = index % 8;
    let rank_idx = index / 8;

    let file = (b'a' + file_idx as u8) as char;
    let rank = (b'1' + rank_idx as u8) as char;

    Some(format!("{file}{rank}"))
}

pub fn square_to_mask(square: &str) -> Option<u64> {
    let mut chars = square.chars();

    let file = chars.next()?;
    let rank = chars.next()?;

    if chars.next().is_some() {
        return None;
    }

    let file_idx = match file {
        'a'..='h' => file as u64 - 'a' as u64,
        _ => return None,
    };

    let rank_idx = match rank {
        '1'..='8' => rank as u64 - '1' as u64,
        _ => return None,
    };

    let bit_index = rank_idx * 8 + file_idx;

    Some(1u64 << bit_index)
}
